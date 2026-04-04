use crate::{chapter::{self, Chapter}, chapter_eleven, chapter_fifteen, chapter_four, chapter_nine, chapter_seven, chapter_six, chapter_ten, chapter_thirteen, chapter_twelve};

pub fn generate_chapters() -> [Chapter<'static>; 9] {
  let chapters: [Chapter<'_>; 9] = [
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
    ),
    chapter::Chapter::new(
    "Generic Types, Traits, and lifetimes",
    "Chapter 10",
    chapter_ten::section::generate_sections()
    ),
    chapter::Chapter::new(
    "Writing Automated Tests",
    "Chapter 11",
    chapter_eleven::section::generate_sections()
    ),
    chapter::Chapter::new(
    "An I/O Project: Building a Command Line Program",
    "Chapter 12",
    chapter_twelve::section::generate_sections()
    ),
    chapter::Chapter::new(
    "Functional Language Features: Iterators and Closures",
    "Chapter 13",
    chapter_thirteen::section::generate_sections()
    ),
    chapter::Chapter::new(
    "Smart Pointers",
    "Chapter 15",
    chapter_fifteen::section::generate_sections()
    )
  ];

  chapters
}