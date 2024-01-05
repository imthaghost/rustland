use warp;

mod routes;
mod handlers;


#[tokio::main]
async fn main() {
    // routes
    let dataframe_routes = routes::dataframe_routes();
    // listen and serve
    warp::serve(dataframe_routes)
    .run(([0,0,0,0], 8080))
    .await;
}