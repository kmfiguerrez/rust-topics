use crate::{chapter::Section, chapter_eleven::eleven_point_one};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("How to Write Tests", "Section 11.1", eleven_point_one::content),
  ]
}