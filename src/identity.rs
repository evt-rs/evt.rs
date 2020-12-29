use crate::Uuid;
use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};

pub fn uuid() -> Uuid {
    Uuid::new_v4()
}

pub fn random(len: usize) -> String {
    let chars = thread_rng().sample_iter(&Alphanumeric).take(len).collect();

    String::from_utf8(chars).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn generates_random() {
        assert_eq!(20, random(20).len());
    }
}
