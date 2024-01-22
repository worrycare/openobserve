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

use std::sync::Arc;

use config::ider;
use datafusion::{
    arrow::{
        array::{Array, ArrayRef, StringArray},
        datatypes::DataType,
    },
    error::DataFusionError,
    logical_expr::{ScalarUDF, Volatility},
    physical_plan::functions::make_scalar_function,
    prelude::create_udf,
};
use once_cell::sync::Lazy;

/// Implementation of unique document id for a given row of data
pub(crate) static DOC_ID_UDF: Lazy<ScalarUDF> = Lazy::new(|| {
    create_udf(
        super::DOC_ID_UDF_NAME,
        // expects two string
        vec![DataType::Utf8],
        // returns boolean
        Arc::new(DataType::Utf8),
        Volatility::Volatile,
        make_scalar_function(doc_id_generator),
    )
});

fn doc_id_generator(args: &[ArrayRef]) -> datafusion::error::Result<ArrayRef> {
    if args.len() != 1 {
        return Err(DataFusionError::Internal(format!(
            "{} was called with {} arguments. Only pass doc_id_generator(any-existing-column).",
            super::DOC_ID_UDF_NAME,
            args.len()
        )));
    }

    let input_rows = datafusion::common::cast::as_string_array(&args[0])?;
    let count = input_rows.len();
    let result: Vec<String> = (0..count).map(|_| ider::generate()).collect();
    Ok(Arc::new(StringArray::from(result)) as ArrayRef)
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use datafusion::{
        arrow::{
            array::{Int64Array, StringArray},
            datatypes::{DataType, Field, Schema},
            record_batch::RecordBatch,
        },
        datasource::MemTable,
        prelude::SessionContext,
    };

    use super::*;

    #[tokio::test]
    async fn test_generate_doc_id() {
        let sql = "select t.*, doc_id(log) as doc_ids from t";

        // define a schema.
        let schema = Arc::new(Schema::new(vec![
            Field::new("log", DataType::Utf8, false),
            Field::new("id", DataType::Int64, false),
            Field::new("city", DataType::Utf8, false),
        ]));

        // define data.
        let batch = RecordBatch::try_new(
            schema.clone(),
            vec![
                Arc::new(StringArray::from(vec!["a", "b", "c", "d"])),
                Arc::new(Int64Array::from(vec![1, 2, 3, 4])),
                Arc::new(StringArray::from(vec!["NY", "Pune", "SF", "Beijing"])),
            ],
        )
        .unwrap();

        // declare a new context. In spark API, this corresponds to a new spark
        // SQLsession
        let ctx = SessionContext::new();
        ctx.register_udf(DOC_ID_UDF.clone());

        // declare a table in memory. In spark API, this corresponds to
        // createDataFrame(...).
        let provider = MemTable::try_new(schema, vec![vec![batch]]).unwrap();
        ctx.register_table("t", Arc::new(provider)).unwrap();

        let df = ctx.sql(sql).await.unwrap();
        let result = df.collect().await.unwrap();
        assert_eq!(result.len(), 1);

        let count: Vec<_> = result
            .iter()
            .map(|batch| {
                let a = batch.column_by_name("doc_ids").unwrap();
                a.into_data().len()
            })
            .collect();
        assert_eq!(count[0], 4);
    }
}
