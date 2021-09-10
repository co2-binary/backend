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

    pub fn get_data_types(&self) -> DataTypes {
        let mut results = Vec::new();

        let mut i = 1;

        for header in &self.headers {
            if let Some((name, units)) = header.split_once(",") {
                results.push(DataType {
                    id: i,
                    name: name.trim(),
                    units: units.trim(),
                });

                i += 1;
            }
        }

        DataTypes {
            results,
        }
    }

    pub fn get_regions(&self) -> Regions {
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
        
        Regions { results }
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Region<'a> {
    pub id: u64,
    pub name: &'a str,
}

#[derive(Serialize)]
pub struct Regions<'a> {
    pub results: Vec<Region<'a>>,
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
