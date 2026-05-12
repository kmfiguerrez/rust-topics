use crate::{chapter::Section, chapter_sixteen::{sixteen_point_one, sixteen_point_two, sixteen_point_three}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Using Threads to Run Code Simultaneously", "Section 16.1", sixteen_point_one::content),
    Section::new("Transfer Data Between Threads with Message Passing", "Section 16.2", sixteen_point_two::content),
    Section::new("Shared-State Concurrency", "Section 16.3", sixteen_point_three::content),
  ]
}