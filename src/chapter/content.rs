use crate::{chapter::{self, Chapter}, chapter_four, chapter_nine, chapter_seven, chapter_six};

pub fn generate_chapters() -> [Chapter<'static>; 4] {
  let chapters = [
    chapter::Chapter::new(
      "Understanding Ownership",
      "Chapter 4",
      chapter_four::section::generate_sections()
    ),
    chapter::Chapter::new(
      "Enums and Pattern Matching",
      "Chapter 6",
      chapter_six::section::generate_sections()
    ),
    chapter::Chapter::new(
    "Packages, Crates and Modules",
    "Chapter 7",
    chapter_seven::section::generate_sections()
    ),
    chapter::Chapter::new(
    "Error Handling",
    "Chapter 9",
    chapter_nine::section::generate_sections()
    )    
  ];

  chapters
}