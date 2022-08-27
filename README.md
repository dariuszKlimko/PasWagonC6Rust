#PasWagonC6Rust

## Table of contents
* [Technologiess](#technologies)
* [Setup](#setup)
* [Run app](#run-app)

## Technologies
* [Rust](https://www.rust-lang.org/)
  * [Rocket](https://rocket.rs/)

## Setup
  Download repository "PaswagonC6Rust".

  Open folder "PasWagonC6Rust-main" in e.g. vscode editor and run:
  ```
  cargo run
  ```
  
## Run app
  To start app again type in terminal:
  ```
  cargo run
  ```
  In browser paste:

  Fuel usage. Type fuel consumption with dot e.g fuelUsagePer100KM=6.5 if PasWagonC6 is comsuming only!!! 6.5 liters/100KM:
  ```
  http://127.0.0.1:8000/calculateDieselUsageForDistance?distance=30&yearOfProduction=1999&fuelUsagePer100KM=6.5
  ```
  Fail probability:
  ```
  http://127.0.0.1:8000/probabilityOfUnitInjectorFail?VIN=OINION9098
  ```
