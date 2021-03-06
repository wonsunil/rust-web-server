use std::fmt;
use std::collections::HashMap;
use regex::Regex;

use crate::util::{ replace, contains, vector };

pub struct Json{
    data: HashMap<String, String>
}

impl Json{
    pub fn get_data(&self) -> HashMap<String, String> {
        self.data.clone()
    }

    pub fn get(&self, key: &str) -> String {
        match self.data.get(key.into()) {
            Some(value) => value.into(),
            None => "".into()
        }
    }

    pub fn insert(&mut self, key: String, value: String) {
        self.data.insert(key.into(), value.into());
    }

    pub fn insert_map(&mut self, key: String, value: HashMap<String, String>) {
        self.data.insert(key, stringify(value));
    }
}

impl std::fmt::Debug for Json {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut data: HashMap<String, String> = HashMap::new();

        self.data.iter().for_each(|(key, value)| {
            data.insert(key.into(), value.into());
        });
        
        write!(f, "{}", stringify(data))
    }
}

pub fn parse(string_data: &str) -> Json {   
    let target_data = replace(string_data.to_string(), vector!{ "{", "}"}, "");
    let target_data_map = target_data.split(",").map(|data| data);
    let mut data_map: HashMap<String, String> = HashMap::new();

    println!("input {}", string_data);

    for data in target_data_map {
        let data: Vec<&str> = data.split(":").collect();
        
        println!("split {:?}", data);

        if data.len() != 0 && data[0] != "" {
            let key = data[0].replace(r#"""#, "");

            #[allow(unused_assignments)]
            let mut value = data[1].replace(r#"""#, "");

            if key == "session" {
                let start = string_data.find("session").unwrap();
                let string = &string_data[start + 9..];
                let end = string.find("}").unwrap();
                
                value = (&string[..end + 1]).to_string();
            };

            data_map.insert(key.into(), value.into());
        }
    }

    Json{
        data: data_map
    }
}

pub fn stringify<S, T: std::fmt::Debug>(datas: HashMap<S, T>) -> String
where
    S: Into<String> + std::fmt::Display,
    T: std::fmt::Display
{
    let datas = datas.into_iter();
    let mut json_string = "".to_owned();

    if datas.len() != 0 {
        let length: usize = datas.len();
        let mut counter = 0;

        json_string.push_str(r#"{"#);

        if contains(&datas, "session") {
            println!("Http Response");
        };

        for (key, value) in datas {
            if counter > length {
                break;
            };

            json_string.push_str(r#"""#);
            json_string += &key.into();
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

pub fn vector_stringify(datas: Vec<String>) -> String {
    datas.join(",")
}