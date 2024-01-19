use std::{collections::HashMap, io::Error};

use actix_web::{post, web, HttpRequest, HttpResponse};
use config::{meta::stream::StreamType, metrics};

use crate::{
    common::{
        meta::{
            self,
            http::HttpResponse as MetaHttpResponse,
            usage::{RequestStats, UsageType},
        },
        utils::{http::get_stream_type_from_request, json},
    },
    service::usage::report_request_usage_stats,
};

#[post("/{org_id}/_search_fed")]
pub async fn federated_search(
    org_id: web::Path<String>,
    in_req: HttpRequest,
    body: web::Bytes,
) -> Result<HttpResponse, Error> {
    let start = std::time::Instant::now();
    let session_id = uuid::Uuid::new_v4().to_string();

    let org_id = org_id.into_inner();
    let query = web::Query::<HashMap<String, String>>::from_query(in_req.query_string()).unwrap();
    let stream_type = match get_stream_type_from_request(&query) {
        Ok(v) => v.unwrap_or(StreamType::Logs),
        Err(e) => return Ok(MetaHttpResponse::bad_request(e)),
    };

    // handle encoding for query and aggs
    let req: meta::proxy::FedSearchRequest = match json::from_slice(&body) {
        Ok(v) => v,
        Err(e) => return Ok(MetaHttpResponse::bad_request(e)),
    };

    let mut search_response = Vec::new();
    for mut cluster_req in req.queries {
        if cluster_req.cluster.url.is_empty() {
            let took_wait = start.elapsed().as_millis() as usize;
            match crate::handler::http::request::search::search_in(
                &mut cluster_req.search_request,
                stream_type,
                &org_id,
                session_id.clone(),
            )
            .await
            {
                Ok(mut res) => {
                    let time = start.elapsed().as_secs_f64();
                    metrics::HTTP_RESPONSE_TIME
                        .with_label_values(&[
                            "/api/org/_search",
                            "200",
                            &org_id,
                            "",
                            stream_type.to_string().as_str(),
                        ])
                        .observe(time);
                    metrics::HTTP_INCOMING_REQUESTS
                        .with_label_values(&[
                            "/api/org/_search",
                            "200",
                            &org_id,
                            "",
                            stream_type.to_string().as_str(),
                        ])
                        .inc();
                    res.set_session_id(session_id.clone());
                    res.set_local_took(start.elapsed().as_millis() as usize, took_wait);

                    let req_stats = RequestStats {
                        records: res.hits.len() as i64,
                        response_time: time,
                        size: res.scan_size as f64,
                        request_body: Some(cluster_req.search_request.query.sql),
                        ..Default::default()
                    };
                    let num_fn = cluster_req.search_request.query.query_fn.is_some() as u16;
                    report_request_usage_stats(
                        req_stats,
                        &org_id,
                        "", // TODO see if we can steam name
                        StreamType::Logs,
                        UsageType::Search,
                        num_fn,
                    )
                    .await;
                    search_response.push(meta::proxy::ClusterResponse {
                        cluster: cluster_req.cluster,
                        response: res,
                    });
                }
                Err(err) => {
                    let time = start.elapsed().as_secs_f64();
                    metrics::HTTP_RESPONSE_TIME
                        .with_label_values(&[
                            "/api/org/_search",
                            "500",
                            &org_id,
                            "",
                            stream_type.to_string().as_str(),
                        ])
                        .observe(time);
                    metrics::HTTP_INCOMING_REQUESTS
                        .with_label_values(&[
                            "/api/org/_search",
                            "500",
                            &org_id,
                            "",
                            stream_type.to_string().as_str(),
                        ])
                        .inc();
                    log::error!("search error: {:?}", err);

                    let resp = meta::search::Response {
                        function_error: err.to_string(),
                        ..Default::default()
                    };
                    search_response.push(meta::proxy::ClusterResponse {
                        cluster: cluster_req.cluster,
                        response: resp,
                    });
                }
            }
        } else {
            // Reqwest setup
            let client = reqwest::Client::new();
            let req = serde_json::to_string(&cluster_req.search_request).unwrap();
            let mut headers = reqwest::header::HeaderMap::new();
            headers.insert("Authorization", cluster_req.cluster.auth.parse().unwrap());
            headers.insert(
                "Content-type",
                reqwest::header::HeaderValue::from_static("application/json"),
            );

            let req_client = client
                .post(cluster_req.cluster.url.as_str())
                .headers(headers);

            match req_client.body(req).send().await {
                Ok(res) => {
                    let resp: meta::search::Response = res.json().await.unwrap();
                    search_response.push(meta::proxy::ClusterResponse {
                        cluster: cluster_req.cluster,
                        response: resp,
                    });
                }
                Err(err) => {
                    log::error!("search error: {:?}", err);
                    let resp = meta::search::Response {
                        function_error: err.to_string(),
                        ..Default::default()
                    };
                    search_response.push(meta::proxy::ClusterResponse {
                        cluster: cluster_req.cluster,
                        response: resp,
                    });
                }
            }
        }
    }

    Ok(HttpResponse::Ok().json(search_response))
}
