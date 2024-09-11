
mod config;
mod libs;
mod controller;

#[tokio::main]
async fn main() {
    tokio::task::spawn_blocking(|| {
        let _ = libs::redis::start_pub_sub();

        println!("PubSub started");
    });

    tokio::task::spawn_blocking(|| {
        let _ = libs::http::start_web_server();

        println!("Web Server started");
    })
    .await
    .unwrap();
}
