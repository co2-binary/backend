mod model;

use rocket::{get, http::Status, launch, routes};

use crate::model::Record;

#[get("/")]
fn index() -> Status {
    Status::ImATeapot
}

#[launch]
fn rocket() -> _ {
    let mut rdr = csv::Reader::from_path("co2.csv").expect("Failed to read file");
    
    for result in rdr.deserialize() {
        let record: Record = result.unwrap();

        dbg!(record);
    }

    rocket::build().mount("/", routes![index])
}
