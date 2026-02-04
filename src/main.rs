use rust_topics::{chapter::{self, content::generate_chapters}};

// Temporary import while making content.
// use rust_topics::chapter_ten;

fn main() {
  // rust_topics::chapter_four::four_point_one::display_contents();
  let chapters = generate_chapters();
  
  chapter::Chapter::prompt_chapters(&chapters);
  // chapter_ten::ten_point_three::las_content();
}

