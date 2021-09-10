mod model;

use rocket::{get, http::Status, launch, routes, serde::json::Json, State};

use crate::model::*;

#[get("/distribution/regions")]
fn regions(records: &State<Records>) -> Json<Regions> {
    Json(records.get_regions())
}

#[get("/distribution/dataTypes")]
fn data_types(records: &State<Records>) -> Json<DataTypes> {
    Json(records.get_data_types())
}

#[get("/")]
fn index() -> Status {
    Status::ImATeapot
}

#[launch]
fn rocket() -> _ {
    let mut rdr = csv::Reader::from_path("co2.csv").expect("Failed to read file");

    let header_record = rdr.headers().expect("Failed to read header");

    let mut records = Records::new(header_record.clone());

    for record in rdr.records() {
        records.add_record(record.unwrap());
    }

    rocket::build()
        .manage(records)
        .mount("/", routes![index])
        .mount("/api/v1", routes![regions, data_types])
}
