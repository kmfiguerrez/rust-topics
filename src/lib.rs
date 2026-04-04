pub mod chapter_four;
pub mod chapter_seven;
pub mod chapter_six;
pub mod chapter_nine;
pub mod chapter_ten;
pub mod chapter_eleven;
pub mod chapter_twelve;
pub mod chapter_thirteen;
pub mod chapter_fifteen;
pub mod menu;
pub mod chapter;


// Practicing with tests
pub fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }
    results
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn one_result() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

    // println!("contents: {contents:#?}");
    assert_eq!(vec!["safe, fast, productive."], search(query, contents));
    }
}