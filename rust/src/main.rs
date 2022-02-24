extern crate rusoto_core;
extern crate rusoto_dynamodb;

use std::default::Default;

use once_cell::sync::OnceCell;

use rusoto_core::Region;
use rusoto_dynamodb::{DynamoDb, DynamoDbClient, ListTablesInput};

use lambda::{handler_fn, Context};
use serde_json::{json, Value};

type Error = Box<dyn std::error::Error + Send + Sync + 'static>;

fn dynamo_db_client() -> &'static DynamoDbClient {
    static INSTANCE: OnceCell<DynamoDbClient> = OnceCell::new();
    INSTANCE.get_or_init(|| {
        DynamoDbClient::new(Region::UsWest2)
    })
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    dynamo_db_client(); // Warm it up during init phase.
    let func = handler_fn(func);
    lambda::run(func).await?;
    Ok(())
}

async fn func(_event: Value, _: Context) -> Result<Value, Error> {
    let list_tables_input: ListTablesInput = Default::default();

    match dynamo_db_client().list_tables(list_tables_input).sync() {
        Ok(output) => {
          match output.table_names {
            Some(table_name_list) => {
              println!("Tables in database:");
              Ok(json!({ "message": "Found tables", "tables": &table_name_list}))
            }
            None => Ok(json!({ "message": "No tables in database!"}))
          }
        }
        Err(error) => {
            Ok(json!({ "message": format!("Error: {:?}", error)}))
        }
    }
}
