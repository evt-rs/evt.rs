use super::*;

pub fn entity(category: &str, id: &str) -> String {
    stream_name!(category, id = id)
}

pub fn split(stream_name: &str) -> (String, Option<String>) {
    match split_once(stream_name) {
        Some((start, end)) => (String::from(start), Some(String::from(end))),
        None => (String::from(stream_name), None),
    }
}

// this exists in unstable
fn split_once(value: &str) -> Option<(&str, &str)> {
    let mut splitter = value.splitn(2, ID_SEPARATOR);
    let first = splitter.next()?;
    let second = splitter.next()?;

    Some((first, second))
}

pub fn get_id(stream_name: &str) -> Option<String> {
    let (_, id) = split(stream_name);
    id
}

pub fn get_ids(stream_name: &str) -> Option<Vec<String>> {
    Some(
        get_id(stream_name)?
            .split(COMPOUND_ID_SEPARATOR)
            .map(String::from)
            .collect(),
    )
}

pub fn get_category(stream_name: &str) -> String {
    let (category, _) = split(stream_name);
    category
}

pub fn is_category(stream_name: &str) -> bool {
    !stream_name.contains(ID_SEPARATOR)
}

pub fn get_category_type(stream_name: &str) -> Option<String> {
    if !stream_name.contains(CATEGORY_TYPE_SEPARATOR) {
        return None;
    }

    let mut parts: Vec<String> = get_category(stream_name)
        .split(CATEGORY_TYPE_SEPARATOR)
        .map(String::from)
        .collect();

    Some(parts.remove(1))
}

pub fn get_category_types(stream_name: &str) -> Option<Vec<String>> {
    let types: Vec<String> = get_category_type(stream_name)?
        .split(COMPOUND_TYPE_SEPARATOR)
        .map(String::from)
        .collect();

    Some(types)
}

pub fn get_entity_name(stream_name: &str) -> String {
    let mut parts: Vec<String> = get_category(stream_name)
        .split(CATEGORY_TYPE_SEPARATOR)
        .map(String::from)
        .collect();

    parts.remove(0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_an_entity_stream() {
        let example = controls::example();
        let category = controls::category();
        let id = controls::id();

        let stream = entity(&category, &id);

        assert_eq!(example, stream);
    }

    #[test]
    fn splits_category_and_id() {
        let example = controls::example();
        let category = controls::category();
        let id = controls::id();

        let (c, i) = split(&example);

        assert_eq!(category, c);
        assert_eq!(id, i.unwrap())
    }

    #[test]
    fn entity_name_is_the_bare_category() {
        let example = controls::category_type_example();
        let category = controls::category();

        let result = get_entity_name(&example);

        assert_eq!(category, result);
    }

    #[test]
    fn parses_id() {
        let example = controls::example();
        let id = controls::id();

        let result = get_id(&example).unwrap();

        assert_eq!(id, result);
    }

    #[test]
    fn parses_ids() {
        let example = controls::compound_id_example();
        let id = controls::id();
        let cardinal_id = controls::cardinal_id();

        let mut result = get_ids(&example).unwrap();

        assert_eq!(2, result.len());
        assert_eq!(cardinal_id, result.remove(0));
        assert_eq!(id, result.remove(0));
    }

    #[test]
    fn parses_category() {
        let example = controls::example();
        let category = controls::category();

        let result = get_category(&example);

        assert_eq!(category, result);
    }

    #[test]
    fn detecting_category() {
        let example = controls::category();

        assert!(is_category(&example))
    }

    #[test]
    fn detecting_category_with_type() {
        let example = controls::category_with_type_example();

        assert!(is_category(&example))
    }

    #[test]
    fn detecting_streams_with_ids_are_not_categories() {
        let example = controls::example();

        assert!(!is_category(&example))
    }

    #[test]
    fn reads_the_correct_category_type() {
        let example = controls::category_type_example();
        let category_type = controls::category_type();

        let result = get_category_type(&example).unwrap();

        assert_eq!(category_type, result);
    }

    #[test]
    fn no_category_type() {
        let example = controls::example();

        let result = get_category_type(&example);

        assert!(result.is_none());
    }

    #[test]
    fn reads_the_correct_category_types() {
        let example = controls::category_with_types_example();
        let types = controls::category_types();

        let result = get_category_types(&example).unwrap();

        assert_eq!(types, result);
    }

    #[test]
    fn no_type() {
        let example = controls::example();

        let result = get_category_type(&example);

        assert!(result.is_none());
    }
}
