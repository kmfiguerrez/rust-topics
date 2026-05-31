use crate::{chapter::Section, chapter_twenty::twenty_point_one};


pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Unsafe Rust", "Section 20.1", twenty_point_one::content),
  ]
}