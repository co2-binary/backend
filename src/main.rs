mod model;

use rocket::{get, http::Status, launch, routes, serde::json::Json, State};

use crate::model::*;

#[get("/distribution/regions")]
fn regions(records: &State<Records>) -> Json<Regions> {
    Json(Regions {
        results: records.get_regions(),
    })
}

#[get("/distribution/dataTypes")]
fn data_types(records: &State<Records>) -> Json<DataTypes> {
    Json(DataTypes {
        results: records.get_data_types(),
    })
}

#[get("/distribution/summary?<year>&<region>&<dataType>")]
fn summary(
    records: &State<Records>,
    year: u32,
    region: u64,
    dataType: u64,
) -> Result<Json<Summary<'_>>, Status> {
    let regions = records.get_regions();

    let region = regions
        .iter()
        .find(|r| r.id == region)
        .ok_or_else(|| Status::NotFound)?
        .clone();

    let data_types = records.get_data_types();

    let data_type = data_types
        .iter()
        .find(|d| d.id == dataType)
        .ok_or_else(|| Status::NotFound)?
        .clone();

    let region_header_index = records
        .get_header_index("region")
        .expect("Failed to find region header");

    let year_header_index = records
        .get_header_index("year")
        .expect("Failed to find year header");

    let month_header_index = records
        .get_header_index("month")
        .expect("Failed to find month header");

    let mut results = Vec::new();

    records
        .records
        .iter()
        .filter(|r| r.get(region_header_index).unwrap() == region.name)
        .filter(|r| r.get(year_header_index).unwrap().parse::<u32>().unwrap() == year)
        .for_each(|r| {
            let value = r
                .get(data_type.header_index)
                .expect("Failed to find value")
                .parse()
                .expect("Failed to parse value");

            let month = r.get(month_header_index).expect("Failed to get month");

            results.push(SummaryResult {
                date_start: format!("{}-{}-1", year, month),
                value,
            });
        });

    Ok(Json(Summary {
        data_type,
        region,
        results,
    }))
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
        .mount("/api/v1", routes![regions, data_types, summary])
}
