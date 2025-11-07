use std::convert::TryInto;

fn length_of_longest_substring(s: String) -> i32 {
    let mut vector: Vec<char> = Vec::new();
    let mut max_length: i32 = 0;

    if s.len() <= 1 {
        return s.len().try_into().unwrap();
    }

    vector.push(s.chars().nth(0).expect("No char found"));

    for i in 1..s.len() {
        let character: char = s.chars().nth(i).expect("No char found");
        
        if let Some(index) = vector.iter().position(|x| *x == character) {
            let new_vec = vector.split_off(index + 1);
            vector.clear();
            vector = new_vec;
        }

        vector.push(character);

        if max_length < vector.len().try_into().unwrap() {
            max_length = vector.len().try_into().unwrap();
        }
    }

    max_length
}

#[test]
fn abcabcbb() {
    assert_eq!(length_of_longest_substring("abcabcbb".to_string()), 3);
}

#[test]
fn bbbbb() {
    assert_eq!(length_of_longest_substring("bbbbb".to_string()), 1);
}

#[test]
fn pwwkew() {
    assert_eq!(length_of_longest_substring("pwwkew".to_string()), 3);
}

#[test]
fn dvdf() {
    assert_eq!(length_of_longest_substring("dvdf".to_string()), 3);
}

#[test]
fn ckilbkd() {
    assert_eq!(length_of_longest_substring("ckilbkd".to_string()), 5);
}
