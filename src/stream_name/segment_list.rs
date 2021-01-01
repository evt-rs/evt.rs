use uuid::Uuid;

pub(crate) trait SegmentList {
    fn process(self) -> Option<Vec<String>>;
}

impl SegmentList for Vec<&str> {
    fn process(self) -> Option<Vec<String>> {
        Some(self.into_iter().map(String::from).collect())
    }
}

impl SegmentList for Vec<String> {
    fn process(self) -> Option<Vec<String>> {
        Some(self)
    }
}

impl SegmentList for &Vec<&str> {
    fn process(self) -> Option<Vec<String>> {
        Some(self.to_owned().into_iter().map(String::from).collect())
    }
}

impl SegmentList for &Vec<String> {
    fn process(self) -> Option<Vec<String>> {
        Some(self.to_owned())
    }
}

impl SegmentList for &[&str] {
    fn process(self) -> Option<Vec<String>> {
        Some(self.to_vec().into_iter().map(String::from).collect())
    }
}

impl SegmentList for &[String] {
    fn process(self) -> Option<Vec<String>> {
        Some(self.to_vec())
    }
}

impl SegmentList for String {
    fn process(self) -> Option<Vec<String>> {
        None
    }
}

impl SegmentList for &String {
    fn process(self) -> Option<Vec<String>> {
        None
    }
}

impl SegmentList for &str {
    fn process(self) -> Option<Vec<String>> {
        None
    }
}
impl SegmentList for &Uuid {
    fn process(self) -> Option<Vec<String>> {
        None
    }
}

impl SegmentList for Uuid {
    fn process(self) -> Option<Vec<String>> {
        None
    }
}
