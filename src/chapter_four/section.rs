use crate::chapter::Section;

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("What is ownership?"),
    Section::new("References and Borrowing"),
    Section::new("The Slice Type")
  ]
}