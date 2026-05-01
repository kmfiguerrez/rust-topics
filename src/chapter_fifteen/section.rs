use crate::{chapter::Section, chapter_fifteen::{fifteen_point_one, fifteen_point_two, fifteen_point_three, fifteen_point_four, fifteen_point_five}};

pub fn generate_sections() -> Vec<Section<'static>> {
  vec![
    Section::new("Using Box<T> to Point to Data on the Heap", "Section 15.1", fifteen_point_one::content),
    Section::new("Treating Smart Pointers Like Regular References", "Section 15.2", fifteen_point_two::content),
    Section::new("Running Code on Cleanup with the Drop Trait", "Section 15.3", fifteen_point_three::content),
    Section::new("Rc<T>, the Reference-Counted Smart Pointer", "Section 15.4", fifteen_point_four::content),
    Section::new("RefCell<T> and the Interior Mutability Pattern", "Section 15.5", fifteen_point_five::content),
  ]
}