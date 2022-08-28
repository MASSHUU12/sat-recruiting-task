use std::collections::HashMap;
use validate_diesel_usage_params::validate_diesel_usage_params;

#[path = "../utils/validateDieselUsageParams.rs"]
mod validate_diesel_usage_params;

/**
 * Calculate diesel usage.
 */
pub async fn diesel_usage_controller(
    distance: HashMap<String, String>,
    year_of_production: HashMap<String, String>,
    fuel_usage_per_100_km: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get errors from validation function.
    let errors = validate_diesel_usage_params(
        String::from(&distance["distance"]),
        String::from(&year_of_production["yearOfProduction"]),
        String::from(&fuel_usage_per_100_km["fuelUsagePer100KM"]),
    );
    let mut result = HashMap::new();

    // Check if validation collected any errors.
    if errors.len() >= 1 as usize {
        // Return errors to user.
        return Ok(warp::reply::json(&errors));
    }

    // Convert params into f32.
    let distance_num = distance["distance"].parse::<f32>().unwrap();
    let usage_num = fuel_usage_per_100_km["fuelUsagePer100KM"]
        .parse::<f32>()
        .unwrap();

    // Calculate usage.
    let usage = usage_num / 100.0 * distance_num;

    result.insert("fuelUsage", usage);

    // Return usage to user.
    return Ok(warp::reply::json(&result));
}
