use csv::StringRecord;
use serde::{Deserialize, Serialize};

pub struct Records {
    headers: Vec<String>,
    records: Vec<StringRecord>,
}

impl Records {
    pub fn new(header_record: StringRecord) -> Self {
        let mut headers = Vec::new();

        for field in header_record.iter() {
            headers.push(field.to_string());
        }

        Self {
            headers,
            records: Vec::new(),
        }
    }

    pub fn add_record(&mut self, r: StringRecord) {
        self.records.push(r);
    }

    pub fn get_data_types(&self) -> Vec<DataType> {
        let mut data_types = Vec::new();

        let mut i = 1;

        for header in &self.headers {
            if let Some((name, units)) = header.split_once(",") {
                data_types.push(DataType {
                    id: i,
                    name: name.trim(),
                    units: units.trim(),
                });

                i += 1;
            }
        }

        data_types
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region {
    pub id: u64,
    pub name: String,
}

#[derive(Serialize)]
pub struct Regions {
    pub results: Vec<Region>,
}

#[derive(Serialize)]
pub struct DataType<'a> {
    id: u64,
    name: &'a str,
    units: &'a str,
}

#[derive(Serialize)]
pub struct DataTypes<'a> {
    pub results: Vec<DataType<'a>>,
}
