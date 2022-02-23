use std::fmt;
use std::collections::HashMap;

use crate::util::{ replace, vector };

pub struct Json{
    data: HashMap<String, String>
}

impl Json{
    pub fn get(&self, key: &str) -> String {
        match self.data.get(key.into()) {
            Some(value) => value.into(),
            None => "".into()
        }
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

    for data in target_data_map {
        let data: Vec<&str> = data.split(":").collect();
        let key = data[0].replace(r#"""#, "");
        let value = data[1].replace(r#"""#, "");

        data_map.insert(key.into(), value.into());
    }

    Json{
        data: data_map
    }
}

pub fn stringify<S, T: std::fmt::Debug>(datas: HashMap<S, T>) -> String
where
    S: Into<String>,
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