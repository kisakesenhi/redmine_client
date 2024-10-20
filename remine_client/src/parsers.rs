//! # Parsers
//!
//! This module contains the functions that parsers would need.
//! Mainly, reader
//!
use serde_json;

#[derive(Debug,Serialize,Deserialize)]
pub struct CustomFieldRecord {
    pub id: usize,
    pub name: String,
    pub value: String,
}

pub fn read_custom_fields_from_template(path: String ) -> Result<Vec<serde_json::Value>, std::io::Error>{
    match std::fs::read_to_string(path){
        Ok(csv) =>{
            let mut readfields=Vec::new();
            let mut reader = csv::ReaderBuilder::new()
                .from_reader(csv.as_bytes());
            for field in reader.deserialize(){
                let customfield: CustomFieldRecord = field?;
                readfields.push(json!(customfield)); 
            }
            Ok(readfields)
        },
        Err(e)=> Err(e)
    }
}

