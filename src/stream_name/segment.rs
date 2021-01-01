use uuid::Uuid;

pub trait Segment {
    fn process(self) -> Option<String>;
}

impl Segment for String {
    fn process(self) -> Option<String> {
        Some(self)
    }
}

impl Segment for Uuid {
    fn process(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Segment for &Uuid {
    fn process(self) -> Option<String> {
        Some(self.to_string())
    }
}

impl Segment for &String {
    fn process(self) -> Option<String> {
        Some(String::from(self))
    }
}

impl Segment for &str {
    fn process(self) -> Option<String> {
        Some(String::from(self))
    }
}

impl Segment for Vec<&str> {
    fn process(self) -> Option<String> {
        None
    }
}

impl Segment for Vec<String> {
    fn process(self) -> Option<String> {
        None
    }
}

impl Segment for &Vec<&str> {
    fn process(self) -> Option<String> {
        None
    }
}

impl Segment for &Vec<String> {
    fn process(self) -> Option<String> {
        None
    }
}

impl Segment for &[&str] {
    fn process(self) -> Option<String> {
        None
    }
}
impl Segment for &[String] {
    fn process(self) -> Option<String> {
        None
    }
}
