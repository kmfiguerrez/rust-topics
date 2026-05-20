use crate::{chapter::Section, chapter_seventeen::{seventeen_point_one, seventeen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Futures and the Async Syntax", "Section 17.1", seventeen_point_one::content),
    Section::new("Applying Concurrency with Async", "Section 17.2", seventeen_point_two::content),
  ]
}