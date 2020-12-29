use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use uuid::Uuid;

pub fn uuid() -> String {
    Uuid::new_v4().to_string()
}

pub fn random(len: usize) -> String {
    let chars = thread_rng().sample_iter(&Alphanumeric).take(len).collect();

    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_uuids() {
        assert_eq!(36, uuid().len());
    }

    #[test]
    fn generates_random() {
        assert_eq!(20, random(20).len());
    }
}
