use crate::identity;

pub fn example() -> String {
    format!("{}-{}", category(), id())
}

pub fn unique_example() -> String {
    format!("{}-{}", category(), identity::uuid())
}

pub fn unique_category() -> String {
    format!("uniqueCategory{}", identity::random(36))
}

pub fn category_type_example() -> String {
    format!("{}-{}", category_with_type_example(), id())
}

pub fn category_with_type_example() -> String {
    format!("{}:{}", category(), category_type())
}

pub fn category_with_types_example() -> String {
    let types = category_types();
    let type_one = types.first().unwrap();
    let type_two = types.last().unwrap();
    format!("{}:{}+{}", category(), type_one, type_two)
}

pub fn compound_id_example() -> String {
    format!("{}-{}+{}", category(), cardinal_id(), id())
}

pub fn compound_category_type_example() -> String {
    format!("{}-{}", category_with_types_example(), id())
}

pub fn category() -> String {
    "category".to_string()
}

pub fn id() -> String {
    "id".to_string()
}

pub fn cardinal_id() -> String {
    "cardinal".to_string()
}

pub fn ids() -> Vec<String> {
    vec![String::from("idOne"), String::from("idTwo")]
}

pub fn category_type() -> String {
    "type".to_string()
}

pub fn category_types() -> Vec<String> {
    vec![String::from("typeOne"), String::from("typeTwo")]
}
