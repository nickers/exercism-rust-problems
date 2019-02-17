use std::collections::HashSet;

pub fn check(candidate: &str) -> bool {
    let mut seen_letters = HashSet::new();
    for c in candidate.to_lowercase().chars() {
        if c != ' ' && c != '-' && seen_letters.contains(&c) {
            return false;
        }
        seen_letters.insert(c);
    }
    return true;
}
