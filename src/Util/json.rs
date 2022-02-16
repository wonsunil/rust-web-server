use std::collections::HashMap;

use crate::util::{ replace };

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

pub fn stringify<T: std::fmt::Debug>(datas: HashMap<&str, T>) -> String
where
    T: std::fmt::Display
{
    let datas = datas.into_iter();
    let mut json_string = "".to_owned();

    if datas.len() != 0 {
        let length: usize = datas.len();
        let mut counter = 0;

        json_string.push_str(r#"{"#);

        for (key, value) in datas {
            if counter > length {
                break;
            };

            json_string.push_str(r#"""#);
            json_string.push_str(key);
            json_string.push_str(r#"""#);
            json_string.push_str(":");
            json_string.push_str(r#"""#);
            json_string.push_str(&value.to_string());
            json_string.push_str(r#"""#);
            
            counter += 1;
            
            if counter < length {
                json_string.push_str(",");
            };
        }

        json_string.push_str(r#"}"#);
    };

    json_string
}