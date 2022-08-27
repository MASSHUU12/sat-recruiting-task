use std::collections::HashMap;

/**
 * Calculate diesel usage.
 */
pub async fn diesel_usage_controller(
    distance: HashMap<String, String>,
    year_of_production: HashMap<String, String>,
    fuel_usage_per_100_km: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    // Get errors from validation function.
    let errors = validate_params(
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

/**
 * Checks if params passed by user are valid, and can be used in program.
 * Returns collected errors or empty Vec.
 */
fn validate_params(distance: String, year: String, usage: String) -> Vec<String> {
    // Defaults can be 0 because distance, year and usage passed by user should not be 0.
    let mut errors = Vec::new();

    if distance.parse::<i32>().unwrap_or_default() == 0 {
        errors.push(format!("Parameter 'distance' is invalid: {}", distance));
    }

    if year.parse::<i32>().unwrap_or_default() == 0 {
        errors.push(format!("Parameter 'yearOfProduction' is invalid: {}", year));
    }

    if usage.parse::<i32>().unwrap_or_default() == 0 {
        errors.push(format!(
            "Parameter 'fuelUsagePer100KM' is invalid: {}",
            usage
        ));
    }
    return errors;
}
