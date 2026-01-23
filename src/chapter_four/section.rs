use crate::{chapter::Section, chapter_four::{four_point_one, four_point_three, four_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("What is ownership?", "Section 4.1", four_point_one::content),
    Section::new("References and Borrowing", "Section 4.2", four_point_two::content),
    Section::new("The Slice Type", "Section 4.3", four_point_three::content)
  ]
}

