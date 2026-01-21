use crate::{chapter::{self, Chapter}, chapter_four, chapter_six};

pub fn generate_chapters() -> [Chapter<'static>; 2] {
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
    )
  ];
  chapters
}