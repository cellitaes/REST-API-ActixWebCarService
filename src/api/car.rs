use actix_web::{
    get,
    error::ResponseError,
    web::Json,
    web,
    HttpResponse,
    http::{header::ContentType, StatusCode}
};
use serde::{Serialize, Deserialize};
use derive_more::{Display};
use rand::Rng;
use chrono::Datelike;


#[derive(Deserialize, Serialize)]
pub struct FuelUsageDetailsIdentifier {
    distance: u32,
    yearOfProduction: u32,
    fuelUsagePer100KM: u32,
}

#[derive(Deserialize, Serialize)]
pub struct CarIdentifier {
    VIN: String,
}

#[derive(Deserialize, Serialize)]
pub struct  GeneralResponse {
    message: String,
    data: f32,
}

#[derive(Debug, Display)]
pub enum CarError {
    IncorrectData,
    BadCarRequest,
}

impl ResponseError for CarError {
    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status_code())
        .insert_header(ContentType::json())
        .body(self.to_string())
    }

    fn status_code(&self) -> StatusCode {
        match self {
            CarError::IncorrectData => StatusCode::UNPROCESSABLE_ENTITY,
            CarError::BadCarRequest => StatusCode::BAD_REQUEST
        }
    }
}

// let params = web::Query::<Params>::from_query(req.query_string()).unwrap(); można też tak i sprawdzic długość???
#[get("/calculateDisselUsageForDistance")]
pub async fn get_disselUsage(
    fuelUsageDetails_identifier: web::Query<FuelUsageDetailsIdentifier>)
     -> Result<Json<GeneralResponse>, CarError> {
    let fuelUsageDetails_identifier = fuelUsageDetails_identifier.into_inner();
    let distance = fuelUsageDetails_identifier.distance;
    let fuelUsagePer100KM = fuelUsageDetails_identifier.fuelUsagePer100KM;

    let yearOfProduction = fuelUsageDetails_identifier.yearOfProduction;
    let current_year = chrono::Utc::now().year();

    if yearOfProduction as i32 - current_year > 0 {
        return Err(CarError::IncorrectData);
    }

    let res = ((fuelUsagePer100KM)*distance) as f32 / 100.0;
    Ok(Json(GeneralResponse {message: "OK".to_string(), data: res as f32}))
}

#[get("/probabilityOfUnitInjectorFail")]
pub async fn get_injectorFail(
    car_identifier: web::Query<CarIdentifier>) 
    -> Result<Json<GeneralResponse>, CarError> {
    let car_identifier = car_identifier.into_inner();

    if vin::verify_checksum(&car_identifier.VIN).is_err() {
        return Err(CarError::IncorrectData);
    }

    let num = rand::thread_rng().gen_range(0..101) as f32/100.0;

    Ok(Json(GeneralResponse { message: "OK".to_string(), data: num }))
}