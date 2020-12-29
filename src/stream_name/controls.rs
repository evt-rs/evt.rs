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

pub fn category<'a>() -> &'a str {
    "category"
}

pub fn id<'a>() -> &'a str {
    "id"
}

pub fn cardinal_id<'a>() -> &'a str {
    "cardinal"
}

pub fn ids<'a>() -> Vec<&'a str> {
    vec!["idOne", "idTwo"]
}

pub fn category_type<'a>() -> &'a str {
    "type"
}

pub fn category_types<'a>() -> Vec<&'a str> {
    vec!["typeOne", "typeTwo"]
}
