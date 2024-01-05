use warp::{self, Filter};

use crate::handlers;


/// All dataframe routes
pub fn dataframe_routes(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    random_dataframe()
}


/// GET /rand
fn random_dataframe(
) -> impl Filter<Extract = impl warp::Reply, Error = warp::Rejection> + Clone {
    warp::path("rand")
        .and(warp::get())
        .and_then(handlers::random_dataframe)
}