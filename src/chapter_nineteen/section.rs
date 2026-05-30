use crate::{chapter::Section, chapter_nineteen::{nineteen_point_one, nineteen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("All the Places Patterns Can Be Used", "Section 19.1", nineteen_point_one::content),
    Section::new("Refutability: Whether a Pattern Might Fail to Match", "Section 19.2", nineteen_point_two::content),
  ]
}