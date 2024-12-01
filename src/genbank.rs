use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Locus {
    pub name: String,
    pub length: i32,
    pub molecule: Molecule,
    pub genbank_division: String,
    pub modification_date: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Molecule {
    #[serde(rename = "type")]
    pub type_: String,
    pub kind: String,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct GenbankRecord {
    pub locus: Option<Locus>,
}

impl GenbankRecord {
    pub fn new() -> GenbankRecord {
        GenbankRecord { locus: None }
    }
}
