use std::collections::HashMap;

/**
 * Calculate diesel usage.
 */
pub async fn diesel_usage_controller(
    distance: HashMap<String, String>,
    year_of_production: HashMap<String, String>,
    fuel_usage_per_100_km: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // TODO: Check if params are empty.
    let mut result = HashMap::new();

    result.insert("fuelUsage", 6);

    return Ok(warp::reply::json(&result));
}
