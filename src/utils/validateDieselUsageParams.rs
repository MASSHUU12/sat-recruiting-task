use std::collections::HashMap;

/**
 * Checks if params passed by user are valid, and can be used in program.
 * Returns collected errors or empty Vec.
 */
pub fn validate_diesel_usage_params(
    distance: String,
    year: String,
    usage: String,
) -> HashMap<String, Vec<String>> {
    let mut hold_errors = HashMap::new();
    let mut errors = Vec::new();

    // Defaults can be -1 because distance, year and usage passed by user should not be -1.
    // Therefore, if the result is -1, you know that something has gone wrong.
    if distance.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!("Parameter 'distance' is invalid: {}", distance));
    }

    if year.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!("Parameter 'yearOfProduction' is invalid: {}", year));
    }

    if usage.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!(
            "Parameter 'fuelUsagePer100KM' is invalid: {}",
            usage
        ));
    }

    hold_errors.insert(String::from("err"), errors);

    return hold_errors;
}
