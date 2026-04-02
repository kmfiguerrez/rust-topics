use crate::{chapter::Section, chapter_thirteen::thirteen_point_one};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Closures", "Section 13.1", thirteen_point_one::content),
  ]
}