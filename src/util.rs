// use std::any::type_name;

pub mod cmd;
pub mod json;

pub fn replace(target_string: String, change_string: Vec<&str>, replace_string: &str) -> String {
    let mut new_string = target_string;

    for change in change_string {
        new_string = new_string.replace(&change, replace_string);
    }

    new_string
}

pub fn contains<T, S>(iterable: &T, key: S) -> bool
where
    S: Into<String>
{
    // iterable.iter();

    false
}

// pub fn type_of<T>(_: T) -> &'static str {
//     type_name::<T>()
// }

macro_rules! map {
    { $($key:expr => $value:tt), + } => {
        {
            let mut map = ::std::collections::HashMap::new();

            $(
                map.insert($key, $value);
            )+

            map
        }
    }
}

macro_rules! vector {
    { $($value:tt), + } => {
        {
            let mut vec = std::vec::Vec::new();

            $(
                vec.push($value);
            )+

            vec
        }
    }
}

pub(crate) use map;
pub(crate) use vector;