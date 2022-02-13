mod retreiver;
mod storage;
mod update;

#[tokio::main]
async fn main() {
    let update_result = update::update().await;
    if update_result {
        std::process::exit(1);
    }

    std::process::exit(0);
}
