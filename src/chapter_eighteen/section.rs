use crate::{chapter::Section, chapter_eighteen::{eighteen_point_one, eighteen_point_three, eighteen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Characteristics of Object-Oriented Languages", "Section 18.1", eighteen_point_one::content),
    Section::new("Using Trait Objects to Abstract over Shared Behavior", "Section 18.2", eighteen_point_two::content),
    Section::new("Implementing an Object-Oriented Design Pattern", "Section 18.3", eighteen_point_three::content),
  ]
}