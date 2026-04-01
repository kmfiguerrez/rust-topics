use crate::{chapter::Section, chapter_twelve::{twelve_point_five, twelve_point_four, twelve_point_one, twelve_point_three, twelve_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Accepting Command Line Arguments", "Section 12.1", twelve_point_one::content),
    Section::new("Reading a File", "Section 12.2", twelve_point_two::content),
    Section::new("Refactoring to Improve Modularity and Error Handling", "Section 12.3", twelve_point_three::content),
    Section::new("Adding Functionality with Test-Driven Development", "Section 12.4", twelve_point_four::content),
    Section::new("Working with Environment Variables", "Section 12.5", twelve_point_five::content),
  ]
}