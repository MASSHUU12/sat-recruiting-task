use diesel_usage_controller::diesel_usage_controller;
use probability_of_fail_controller::probability_of_fail_controller;
use std::collections::HashMap;
use warp::{filters::BoxedFilter, Filter, Reply};

#[path = "../controllers/dieselUsageController.rs"]
mod diesel_usage_controller;

#[path = "../controllers/probabilityOfFailController.rs"]
mod probability_of_fail_controller;

/**
 * Contains all routes and returns them.
 *
 * @returns - BoxedFilter<(impl Reply,)>
 */
pub fn routes() -> BoxedFilter<(impl Reply,)> {
    // GET: /calculateDieselUsageForDistance
    let calculate_diesel_usage_for_distance = warp::get()
        // Path
        .and(warp::path("calculateDieselUsageForDistance"))
        // Params
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::query::<HashMap<String, String>>())
        .and(warp::query::<HashMap<String, String>>())
        // End
        .and(warp::path::end())
        // Call controller.
        .and_then(diesel_usage_controller);

    // GET /probabilityOfUnitInjectorFail
    let probability_of_unit_injector_fail = warp::get()
        // Path
        .and(warp::path("probabilityOfUnitInjectorFail"))
        // Params
        .and(warp::query::<HashMap<String, String>>())
        // End
        .and(warp::path::end())
        // Call controller.
        .and_then(probability_of_fail_controller);

    // Return boxed routes.
    return warp::get()
        .and(calculate_diesel_usage_for_distance.or(probability_of_unit_injector_fail))
        .boxed();
}
