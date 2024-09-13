mod config;
mod libs;
mod functions;

#[tokio::main]
async fn main() {
    let _ = crate::functions::pubsub::start_pub_sub().await;

    println!("PubSub started");
}
