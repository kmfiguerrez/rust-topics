use crate::{chapter::Section, chapter_eleven::{eleven_point_one, eleven_point_three, eleven_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("How to Write Tests", "Section 11.1", eleven_point_one::content),
    Section::new("Controlling How Tests Are Run", "Section 11.2", eleven_point_two::content),
    Section::new("Test Organization", "Section 11.3", eleven_point_three::content),
  ]
}