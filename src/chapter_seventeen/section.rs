use crate::{chapter::Section, chapter_seventeen::{seventeen_point_five, seventeen_point_four, seventeen_point_one, seventeen_point_six, seventeen_point_three, seventeen_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Futures and the Async Syntax", "Section 17.1", seventeen_point_one::content),
    Section::new("Applying Concurrency with Async", "Section 17.2", seventeen_point_two::content),
    Section::new("Working With Any Number of Futures", "Section 17.3", seventeen_point_three::content),
    Section::new("Streams: Futures in Sequence", "Section 17.4", seventeen_point_four::content),
    Section::new("A Closer Look at the Traits for Async", "Section 17.5", seventeen_point_five::content),
    Section::new("Futures, Tasks, and Threads", "Section 17.6", seventeen_point_six::content),
  ]
}