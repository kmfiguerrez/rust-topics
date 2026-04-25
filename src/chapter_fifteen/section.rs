use crate::{chapter::Section, chapter_fifteen::{fifteen_point_one, fifteen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Using Box<T> to Point to Data on the Heap", "Section 15.1", fifteen_point_one::content),
    Section::new("Treating Smart Pointers Like Regular References", "Section 15.2", fifteen_point_two::content),
  ]
}