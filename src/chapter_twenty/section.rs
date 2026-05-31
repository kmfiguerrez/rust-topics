use crate::{chapter::Section, chapter_twenty::{twenty_point_five, twenty_point_four, twenty_point_one, twenty_point_three, twenty_point_two}};


pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Unsafe Rust", "Section 20.1", twenty_point_one::content),
    Section::new("Advanced Traits", "Section 20.2", twenty_point_two::content),
    Section::new("Advanced Types", "Section 20.3", twenty_point_three::content),
    Section::new("Advanced Functions and Closures", "Section 20.4", twenty_point_four::content),
    Section::new("Macros", "Section 20.5", twenty_point_five::content),
  ]
}