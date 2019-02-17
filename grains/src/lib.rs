pub fn square(s: u32) -> u64 {
    assert!(s >= 1 && s <= 64, "Square must be between 1 and 64");
    // 1u64 << (s-1) // fast solution
    match s {
        1 => 1,
        n => square(n-1) * 2,
    }
}

pub fn total() -> u64 {
    // !0u64 // fast solution :)
    (1..64+1).map(|i| square(i)).sum()
}
