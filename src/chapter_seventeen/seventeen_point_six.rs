// use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Putting It All Together: Futures, Tasks, and Threads", ftt_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Putting It All Together: Futures, Tasks, and Threads. Abbreviated as ftt.
fn ftt_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Putting It All Together: Futures, Tasks, and Threads");
}