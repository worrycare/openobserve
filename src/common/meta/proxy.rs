use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct QueryParamProxyURL {
    #[serde(alias = "proxy-token")]
    pub proxy_token: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct PathParamProxyURL {
    pub target_url: String,
    pub org_id: String,
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize, ToSchema)]
pub struct Cluster {
    pub url: String,
    pub auth: String,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClusterRequest {
    pub cluster: Cluster,
    pub search_request: crate::common::meta::search::Request,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct FedSearchRequest {
    pub queries: Vec<ClusterRequest>,
}

#[derive(Clone, Debug, Serialize, Deserialize, ToSchema)]
pub struct ClusterResponse {
    pub cluster: Cluster,
    pub response: crate::common::meta::search::Response,
}
