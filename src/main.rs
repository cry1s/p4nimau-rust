#[tokio::main]
async fn main() {
    match p4nimau_rust::run().await {
        Ok(_) => (),
        Err(e) => {
            println!("Error while starting: {}", e);
        }
    };
}
