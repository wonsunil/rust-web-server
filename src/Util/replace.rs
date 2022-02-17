pub fn replace(target_string: String, change_string: Vec<&str>, replace_string: &str) -> String {
    let mut new_string = target_string;

    for change in change_string {
        new_string = new_string.replace(&change, replace_string);
    }

    new_string
}