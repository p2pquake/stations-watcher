use lambda_runtime::{handler_fn, Context, Error};
use serde_json::{json, Value};

mod retreiver;
mod storage;
mod update;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let func = handler_fn(func);
    lambda_runtime::run(func).await?;
    Ok(())
}

async fn func(_: Value, _: Context) -> Result<Value, Error> {
    let update_result = update::update().await;

    if update_result {
        return Ok(json!({ "message": "Update available!" }));
    }

    Ok(json!({ "message": "No diff" }))
}
