use crate::{chapter::Section, chapter_ten::{ten_point_one, ten_point_three, ten_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Generic Data Types", "Section 10.1", ten_point_one::content),
    Section::new("Defining Shared Behavior with Traits", "Section 10.2", ten_point_two::content),
    Section::new("Validating References with Lifetimes", "Section 10.3", ten_point_three::content),
  ]
}