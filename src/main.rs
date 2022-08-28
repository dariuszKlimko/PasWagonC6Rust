#[macro_use] extern crate rocket;

use rand::Rng;
use rocket::serde::json::Json;
use rocket::serde::Serialize;

// Here is 8 warnings 6 "snake case name" and 2 "unused variable".
// I do not change snake case because in Technical Requirements is used camel case.
// Also I received 2 unused parameters but customer want to them here

#[derive(Serialize)]
struct Fuel {
    fuelUsage: f32,
}

#[get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculatedieseldsagefordistance(distance: &str, yearOfProduction: &str, fuelUsagePer100KM: &str) -> Json<Fuel> {
    let distf32: f32 = distance.parse().unwrap();
    let yearf32: f32 = yearOfProduction.parse().unwrap();
    let fuelf32: f32 = fuelUsagePer100KM.parse().unwrap();
    let fuelusage: f32 = fuelf32*distf32/100.0;
    let fuel = Fuel {
        fuelUsage: fuelusage,
    };
    Json(fuel)
}

#[derive(Serialize)]
struct Fail {
    failProbability: String,
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
fn probabilityofunitinjectorfail(VIN: &str) -> Json<Fail>{
    let mut rng = rand::thread_rng();
    let failprobability = rng.gen_range(0..101);
    let failStr = failprobability.to_string();
    let fail = Fail {
        failProbability: format!("{}%",failStr),
    };
    Json(fail)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calculatedieseldsagefordistance, probabilityofunitinjectorfail])
}