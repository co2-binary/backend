use csv::StringRecord;
use serde::{Deserialize, Serialize};

pub struct Records {
    headers: Vec<String>,
    pub records: Vec<StringRecord>,
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

    pub fn get_header_index(&self, name: &str) -> Option<usize> {
        for i in 0..self.headers.len() {
            if self.headers[i] == name {
                return Some(i);
            }
        }

        return None;
    }

    pub fn add_record(&mut self, r: StringRecord) {
        self.records.push(r);
    }

    pub fn get_data_types(&self) -> Vec<DataType> {
        let mut results = Vec::new();

        let mut id = 1;

        for header_index in 0..self.headers.len() {
            let header = &self.headers[header_index];

            if let Some((name, units)) = header.split_once(",") {
                results.push(DataType {
                    id,
                    name: name.trim(),
                    units: units.trim(),
                    header_index,
                });

                id += 1;
            }
        }

        results
    }

    pub fn get_regions(&self) -> Vec<Region> {
        let header_index = self
            .headers
            .iter()
            .position(|h| h == "region")
            .expect("Failed to find region header");

        let mut regions = Vec::new();

        for record in &self.records {
            let region = record.get(header_index).expect("Missing region");

            if !regions.contains(&region) {
                regions.push(region);
            }
        }

        regions.sort();

        let mut results = Vec::new();

        let mut i = 1;

        for region in regions {
            results.push(Region {
                id: i,
                name: region,
            });

            i += 1;
        }

        results
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Region<'a> {
    pub id: u64,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct Regions<'a> {
    pub results: Vec<Region<'a>>,
}

#[derive(Serialize, Clone)]
pub struct DataType<'a> {
    pub id: u64,
    pub name: &'a str,
    pub units: &'a str,
    #[serde(skip_serializing)]
    pub header_index: usize,
}

#[derive(Serialize)]
pub struct DataTypes<'a> {
    pub results: Vec<DataType<'a>>,
}

#[derive(Serialize)]
pub struct Summary<'a> {
    #[serde(rename = "dataType")]
    pub data_type: DataType<'a>,
    pub region: Region<'a>,
    pub results: Vec<SummaryResult>,
}

#[derive(Serialize)]
pub struct SummaryResult {
    #[serde(rename = "dateStart")]
    pub date_start: String,
    pub value: f64,
}
