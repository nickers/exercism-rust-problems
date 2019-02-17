pub fn series(digits: &str, len: usize) -> Vec<String> {
    match len <= digits.len() {
        false => Vec::new(),
        true => (0..digits.len()-len+1)
            .map(|i| digits[i..len+i].to_string())
            .collect(),
    }
}
