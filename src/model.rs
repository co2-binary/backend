use serde::{Serialize, Deserialize};

#[derive(Debug, Deserialize)]
pub struct Record {
    year: u32,
    month: u8,
    region: String,
    #[serde(rename = "co2, tons")]
    co2: f64,
    #[serde(rename = "trees, pcs")]
    trees: u64,
}

pub struct Records(Vec<Record>);

impl Records {
    pub fn new() -> Self {
        Self { 0: Vec::new() }
    }
    
    pub fn add(&mut self, r: Record) {
        self.0.push(r);
    }
    
    pub fn get_regions(&self) -> Vec<Region> {
        let mut regions: Vec<&str> = Vec::new();

        for record in &self.0 {
            if !regions.contains(&record.region.as_str()) {
                regions.push(&record.region);
            }
        }
        
        regions.sort(); 
        
        let mut result = Vec::new();

        let mut i = 1;

        for region in regions {
            result.push(Region {
                id: i,
                name: region.to_string(),
            });
            
            i += 1;
        }
        
        result
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