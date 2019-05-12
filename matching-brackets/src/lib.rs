pub fn brackets_are_balanced(string: &str) -> bool {
    let mut heap = Vec::new();

    for c in string.chars() {
        match c {
            '}' | ']' | ')' => match heap.pop() {
                x if Some(c) == x => (),
                _ => return false,
            },
            '{' => heap.push('}'),
            '[' => heap.push(']'),
            '(' => heap.push(')'),
            _ => (),
        }
    }

    heap.is_empty()
}
