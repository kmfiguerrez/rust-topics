use crate::{chapter::Section, chapter_ten::ten_point_one};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Generic Data Types", "Section 10.1", ten_point_one::content),
  ]
}