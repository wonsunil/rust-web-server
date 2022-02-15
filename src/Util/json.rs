use std::collections::HashMap;

use crate::util::replace;

pub type Json = HashMap<String, String>;

pub fn parse(string_data: &str) -> Json {   
    let mut vec: Vec<&str> = Vec::new();
    vec.push("{");
    vec.push("}");
    
    let target_data = replace(String::from(string_data), vec, "");
    let mut data_map = HashMap::new();

    for data in target_data.split(",") {
        let data: Vec<&str> = data.split(":").collect();
        
        let key = data[0];
        let value = data[1];

        data_map.insert(key.into(), value.into());
    }
    
    data_map
}