use crate::{chapter::Section, chapter_thirteen::{thirteen_point_four, thirteen_point_one, thirteen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Closures", "Section 13.1", thirteen_point_one::content),
    Section::new("Processing a Series of Items with Iterators", "Section 13.2", thirteen_point_two::content),
    Section::new("Performance in Loops vs. Iterators", "Section 13.4", thirteen_point_four::content),
  ]
}