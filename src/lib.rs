pub mod chapter_four;
pub mod chapter_seven;
pub mod chapter_six;
pub mod chapter_nine;
pub mod chapter_ten;
pub mod chapter_eleven;
pub mod chapter_twelve;
pub mod menu;
pub mod chapter;


// Practicing with tests
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }

    #[test]
    fn aubrey_pangit() {
        panic!("Aubreys is ugly as fuck");
    }    
}