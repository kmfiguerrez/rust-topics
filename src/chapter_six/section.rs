use crate::{chapter::Section, chapter_six::{six_point_one, six_point_three, six_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Defining an Enum", "Section 6.1", six_point_one::content),
    Section::new("The match control Flow Construct", "Section 6.2", six_point_two::content),
    Section::new("Concise Control Flow with if let and let else", "Section 6.3", six_point_three::content)
  ]
}