use crate::{chapter::Section, chapter_four::{four_point_one, four_point_three, four_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("What is ownership?", four_point_one::content),
    Section::new("References and Borrowing", four_point_two::content),
    Section::new("The Slice Type", four_point_three::content)
  ]
}

