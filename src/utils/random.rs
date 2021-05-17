use rand::{thread_rng,Rng};

pub fn generate_random_alpha_key(length: usize) -> String {
    let mut rng = thread_rng();
    let mut result = String::default();
    let min = b'A';
    let max = b'Z' - b'A';
    for _i in 0..length {
        let value = rng.gen::<u8>() % max + min;
        result.push(value as char);
    }

    result
}

#[test]
fn test_random_alpha_key() {
    let key = generate_random_alpha_key(5);
    assert_eq!(key.len(), 5);
    for c in key.chars() {
        assert_eq!(true, c.is_alphabetic());
    }
    let key = generate_random_alpha_key(15);
    assert_eq!(key.len(), 15);
    for c in key.chars() {
        assert_eq!(true, c.is_alphabetic());
    }
}
