#[macro_use] extern crate rocket;
use rand::Rng;

// Here is 6 warnings 4 "snake case name" and 2 "unused variable".
// I do not change snake case because in Technical Requirements is used camel case.
// Also I received 2 unused parameters but customer want to them here

#[get("/calculateDieselUsageForDistance?<distance>&<yearOfProduction>&<fuelUsagePer100KM>")]
fn calculatedieseldsagefordistance(distance: &str, yearOfProduction: &str, fuelUsagePer100KM: &str) -> String {
    let dist: f32 = distance.parse().unwrap();
    let year: f32 = yearOfProduction.parse().unwrap();
    let fuel: f32 = fuelUsagePer100KM.parse().unwrap();
    let fuelUsage: f32 = fuel*dist/100.0;
    format!("fuelUsage: {}",fuelUsage)
}

#[get("/probabilityOfUnitInjectorFail?<VIN>")]
fn probabilityofunitinjectorfail(VIN: &str) -> String {
    let mut rng = rand::thread_rng();
    let failprobability: u8 = rng.gen_range(0..101);
    format!("failProbability: {}%",failprobability)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![calculatedieseldsagefordistance, probabilityofunitinjectorfail])
}