use std::collections::HashMap;

/**
 * Calculate probability of Unit Injector fail.
 */
pub async fn probability_of_fail_controller(
    vin: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Check if params are empty.

    let mut result = HashMap::new();

    result.insert("failProbability", "0,77");

    return Ok(warp::reply::json(&result));
}
