pub mod days;

mod utils;

#[cfg(test)]
pub mod tests {
    #[test]
    pub fn day_01_a() {
        assert_eq!(crate::days::day_01::part_a(), 3512133);
    }
    #[test]
    pub fn day_01_b() {
        assert_eq!(crate::days::day_01::part_b(), 5265294);
    }
}
