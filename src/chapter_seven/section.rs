use crate::{chapter::Section, chapter_seven::{seven_point_five, seven_point_four, seven_point_one, seven_point_three, seven_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Packages and Crates", "Section 7.1", seven_point_one::content),
    Section::new("Control Scope and Privacy with Modules", "Section 7.2", seven_point_two::content),
    Section::new("Paths for Referring to an Item in the Module Tree", "Section 7.3", seven_point_three::content),
    Section::new("Bringing Paths Into Scope with the use Keyword", "Section 7.4", seven_point_four::content),
    Section::new("Separating Modules into Different Files", "Section 7.5", seven_point_five::content)
  ]
}