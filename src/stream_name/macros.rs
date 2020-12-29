use super::*;

#[macro_export]
macro_rules! stream_name {
    ($category:expr, $($segment:ident = $value:expr),*) => {
    {
       use $crate::stream_name::segment::Segment;
       use $crate::stream_name::segment_list::SegmentList;
       use $crate::stream_name::macros::build_stream_name;

       let mut cardinal_id: Option<String> = None;
       let mut id: Option<String> = None;
       let mut ids: Option<Vec<String>> = None;
       let mut category_type: Option<String> = None;
       let mut category_types: Option<Vec<String>> = None;

        $(
            match stringify!($segment) {
                "cardinal_id" => cardinal_id = Segment::process($value),
                "id" => id = Segment::process($value),
                "category_type" => category_type = Segment::process($value),
                "ids" => ids = SegmentList::process($value),
                "category_types" => category_types = SegmentList::process($value),
                _ => panic!("Unknown stream name segment type: {}", stringify!($segment)),
            }
        )*

        build_stream_name(String::from($category), cardinal_id, id, ids, category_type, category_types)
    }
    };

    ($category:expr) => { String::from($category) };
}

pub fn build_stream_name(
    category: String,
    cardinal_id: Option<String>,
    identifier: Option<String>,
    identifiers: Option<Vec<String>>,
    category_type: Option<String>,
    category_types: Option<Vec<String>>,
) -> String {
    let mut stream_name = category;
    let mut types: Vec<String> = vec![];

    if let Some(t) = category_type {
        types.push(t);
    }

    if let Some(ts) = category_types {
        for t in ts {
            types.push(t);
        }
    }

    if !types.is_empty() {
        let type_part = types.join(COMPOUND_TYPE_SEPARATOR);
        stream_name.push_str(CATEGORY_TYPE_SEPARATOR);
        stream_name.push_str(type_part.as_str());
    }

    let mut ids: Vec<String> = vec![];

    if let Some(i) = cardinal_id {
        ids.push(i);
    }

    if let Some(i) = identifier {
        ids.push(i);
    }

    if let Some(is) = identifiers {
        for i in is {
            ids.push(i);
        }
    }

    if !ids.is_empty() {
        let id_part = ids.join(COMPOUND_ID_SEPARATOR);
        stream_name.push_str(ID_SEPARATOR);
        stream_name.push_str(id_part.as_str());
    }

    stream_name
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn just_a_category() {
        let category = controls::category();

        let name = stream_name!(category);

        assert_eq!(category, name);
    }

    #[test]
    fn type_comes_after_category() {
        let category = controls::category();
        let category_type = controls::category_type();
        let expected = format!("{}:{}", category, category_type);

        let name = stream_name!(category, category_type = category_type);

        assert_eq!(name, expected);
    }

    #[test]
    fn compound_types_with_separator() {
        let category = controls::category();
        let types = controls::category_types();
        let expected = format!("{}:{}", category, types.join("+"));

        let name = stream_name!(category, category_types = types);

        assert_eq!(expected, name);
    }

    #[test]
    fn id_comes_at_end() {
        let category = controls::category();
        let id = controls::id();

        let expected = format!("{}-{}", category, id);

        let result = stream_name!(category, id = id);

        assert_eq!(expected, result);
    }

    #[test]
    fn ids_are_separated() {
        let category = controls::category();
        let ids = controls::ids();

        let expected = format!("{}-{}", category, ids.join("+"));

        let result = stream_name!(category, ids = ids);

        assert_eq!(expected, result);
    }
}
