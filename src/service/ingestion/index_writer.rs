// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use arrow::record_batch::RecordBatch;

use crate::common::{infra::wal::get_or_create_arrow, meta::stream::StreamParams};

pub async fn write_file_arrow(buf: RecordBatch, thread_id: usize, stream: &StreamParams) {
    let rw_file = get_or_create_arrow(
        thread_id,
        stream.clone(),
        None,
        "",
        Some(buf.schema().as_ref().clone()),
    )
    .await;

    rw_file.write_arrow(buf).await;
}

#[cfg(test)]
mod tests {

    use std::sync::Arc;

    use chrono::Utc;
    use datafusion::arrow::{
        array::{Int64Array, StringArray},
        datatypes::{DataType, Field, Schema},
        record_batch::RecordBatch,
    };

    use super::*;

    #[actix_web::test]
    async fn test_write_file_arrow() {
        let data_time = Utc::now().timestamp_millis();
        // define a schema.
        let schema = Arc::new(Schema::new(vec![
            Field::new("log", DataType::Utf8, false),
            Field::new("time", DataType::Int64, false),
        ]));

        // define data.
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(vec!["a"])),
                Arc::new(Int64Array::from(vec![data_time])),
            ],
        )
        .unwrap();
        write_file_arrow(
            batch,
            0,
            &StreamParams {
                org_id: "org".into(),
                stream_name: "stream".into(),
                stream_type: "logs".into(),
            },
        )
        .await;
    }
}
