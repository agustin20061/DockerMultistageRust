use tokio;
use warp::Filter;

#[tokio::main]
async fn main() {
    let filter= warp::path!()
    .map(|| warp::reply::html("Hello, World!"));

    println!("Server started at http://localhost:3030/hello/<name>");

    warp::serve(filter)
    .run(([127, 0, 0, 1], 3030))
    .await;
}

