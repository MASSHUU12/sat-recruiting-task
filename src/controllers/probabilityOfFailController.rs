use rand::Rng;
use std::collections::HashMap;

/**
 * Returns a percentage of the chance that the engine of the C6 model will fail on the Unit Injector element.
 *
 * @params vin - I dunno what this is, but customer really wants it here.
 */
pub async fn probability_of_fail_controller(
    vin: HashMap<String, String>,
) -> Result<impl warp::Reply, warp::Rejection> {
    let mut result = HashMap::new();
    let mut rng = rand::thread_rng();

    // Check if vin is long enough.
    // Since vin is not used in any way, a more thorough validation of it is not needed.
    if vin["vin"].len() != 17 as usize {
        result.insert("err", format!("Parameter vin is invalid: {}", vin["vin"]));
        return Ok(warp::reply::json(&result));
    }

    // Generate random number in range 0 - 100, and make it string.
    let probability = rng.gen_range(0..100).to_string();

    // If generated number equals to "0", return unformatted "0".
    if probability == "0" {
        result.insert("failProbability", "0".to_string());
        return Ok(warp::reply::json(&result));
    }

    // If generated number equals to "100", return unformatted "1".
    if probability == "100" {
        result.insert("failProbability", "1".to_string());
        return Ok(warp::reply::json(&result));
    }

    // Insert new formatted record into HashMap.
    result.insert("failProbability", format!("0,{}", probability));

    // Return result to user as JSON.
    return Ok(warp::reply::json(&result));
}
