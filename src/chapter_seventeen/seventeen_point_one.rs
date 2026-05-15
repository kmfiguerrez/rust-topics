use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 2];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Fearless Concurrency");

  println!(
  "{0}\n\n\
  We've treated parallelism and concurrency as mostly interchangeable so far.\n\
  Now we need to distinguish between them more precisely, because the differences will show up as we start working.\n\n\
  Consider the different ways a team could split up work on a software project.\n\
  You could assign a single member multiple tasks, assign each member one task, or use a mix of the two approaches.\n\n\
  When an individual works on several different tasks before any of them is complete, this is {1}.\n\
  One way to implement concurrency is similar to having two different projects checked out on your computer, and when you get \
  bored or stuck on one project, you switch to the other.\n\
  You're just one person, so you can't make progress on both tasks at the exact same time, but you can multitask, making progress \
  on one at a time by switching between them.\n\n\
  When the team splits up a group of tasks by having each member take one task and work on it alone, this is {2}.\n\
  Each person on the team can make progress at the exact same time.\n\n\
  See: {3}, for complete reading.
  ",
  "Parallelism and Concurrency".bright_magenta().bold(),
  "concurrency".italic().bold(),
  "parallelism".italic().bold(),
  "https://doc.rust-lang.org/book/ch17-00-async-await.html#parallelism-and-concurrency".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} On a machine with a single CPU core, the CPU can perform only one operation at a time, but it can still \
  work concurrently.\n\
  {solid_disc} On a machine with multiple CPU cores, it can also do work in parallel. One core can be performing one task \
  while another core performs a completely unrelated one, and those operations actually happen at the same time.\n\
  {solid_disc} Running async code in Rust usually happens concurrently.
  ",
  "REMEMBER".bright_yellow().bold()
  )
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Using Threads to Run Code Simultaneously");
}