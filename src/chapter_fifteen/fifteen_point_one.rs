use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction: Closures", si_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction");
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction");
}