use crate::{chapter::Section, chapter_nine::{nine_point_one, nine_point_three, nine_point_two}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Unrecoverable Errors with panic!", "Section 9.1", nine_point_one::content),
    Section::new("Recoverable Errors with Result", "Section 9.2", nine_point_two::content),
    Section::new("To panic! or Not to panic!", "Section 9.3", nine_point_three::content),
  ]
}