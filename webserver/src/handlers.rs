use std::convert::Infallible;

use warp;


/// Returns a random dataframe as JSON
///
/// # Arguments
///
pub async fn random_dataframe() -> Result<impl warp::Reply, Infallible> {
    // create empty vector
    let vec = vec![1, 2, 3];
    Ok(warp::reply::json(&vec))

}