mod model;

use rocket::{get, http::Status, launch, routes, serde::json::Json, State};

use crate::model::*;

#[get("/distribution/regions")]
fn regions(records: &State<Records>) -> Json<Regions> {
    Json(Regions {
        results: records.get_regions(),
    })
}

#[get("/")]
fn index() -> Status {
    Status::ImATeapot
}

#[launch]
fn rocket() -> _ {
    let mut rdr = csv::Reader::from_path("co2.csv").expect("Failed to read file");

    let mut records = Records::new();

    for result in rdr.deserialize() {
        let record: Record = result.unwrap();

        records.add(record);
    }

    rocket::build()
        .manage(records)
        .mount("/", routes![index])
        .mount("/api/v1", routes![regions])
}
