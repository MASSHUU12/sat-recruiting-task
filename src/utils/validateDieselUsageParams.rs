/**
 * Checks if params passed by user are valid, and can be used in program.
 * Returns collected errors or empty Vec.
 */
pub fn validate_diesel_usage_params(distance: String, year: String, usage: String) -> Vec<String> {
    let mut errors = Vec::new();

    // Defaults can be -1 because distance, year and usage passed by user should not be -1.
    // Therefore, if the result is -1, you know that something has gone wrong.
    if distance.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!(
            "[server]: Parameter 'distance' is invalid: {}",
            distance
        ));
    }

    if year.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!(
            "[server]: Parameter 'yearOfProduction' is invalid: {}",
            year
        ));
    }

    if usage.parse::<i32>().unwrap_or(-1) == -1 {
        errors.push(format!(
            "[server]: Parameter 'fuelUsagePer100KM' is invalid: {}",
            usage
        ));
    }
    return errors;
}
