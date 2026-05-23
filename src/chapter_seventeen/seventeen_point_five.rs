use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("The Future Trait", tft_content),
    chapter::SubHeader::new("The Pin Type and the Unpin Trait", tpatut_content),
    chapter::SubHeader::new("The Stream Trait", tst_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: A Closer Look at the Traits for Async");

  println!(
  "Throughout the chapter, we've used the {0}, {1}, and {2} traits in various ways. So far, though, we've avoided getting too \
  far into the details of how they work or how they fit together, which is fine most of the time for your day-to-day Rust work.\n\
  Sometimes, though, you'll encounter situations where you'll need to understand a few more of these traits' details, along with \
  the {3} type and the {4} trait. In this section, we'll dig in just enough to help in those scenarios, still leaving the really \
  deep dive for other documentation.
  ",
  "Future".bright_yellow().bold(),
  "Stream".bright_yellow().bold(),
  "StreamExt".bright_yellow().bold(),
  "Pin".bright_yellow().bold(),
  "Unpin".bright_yellow().bold(),
  )
}

// Header: The Future Trait. Abbreviated as tft.
fn tft_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("The Future Trait");

  println!(
  "Let's start by taking a closer look at how the {0} trait works. Here's how Rust defines it:\n\n\
  See: {1}, for code sample and complete reading.\n\n\

  ",
  "Future".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-future-trait".bright_cyan(),
  )
}

// Header: The Pin Type and the Unpin Trait. Abbreviated as tpatut.
fn tpatut_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The Pin Type and the Unpin Trait");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-pin-type-and-the-unpin-trait".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {1} is a wrapper for pointer-like types such as {2}, {3}, {4}, and {5}.\n\
  (Technically, {1} works with types that implement the {6} or {7} traits, but this is effectively equivalent to working only \
  with references and smart pointers.)\n\
  {1} is not a pointer itself and doesn't have any behavior of its own like {5} and {8} do with reference counting; it's purely a \
  tool the compiler can use to enforce constraints on pointer usage.\n\n\
  When we pin a value by wrapping a pointer to that value in {1}, it can no longer move.\n\
  Thus, if you have {9}, you actually pin the {10} value, not the {11} pointer.
  ",
  "The Pin type".bright_magenta().bold(),
  "Pin".bright_yellow().bold(),
  "&".bright_yellow().bold(),
  "&mut".bright_yellow().bold(),
  "Box".bright_yellow().bold(),
  "Rc".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "DerefMut".bright_yellow().bold(),
  "Arc".bright_yellow().bold(),
  "Pin<Box<SomeType>>".bright_yellow().bold(),
  "SomeType".bright_yellow().bold(),
  "Box".bright_yellow().bold(),
);

  println!(
  "{0}\n\n\
  When we move a future—whether by pushing it into a data structure to use as an iterator with {1} or by returning it from a \
  function—that actually means moving the state machine Rust creates for us.\n\
  And unlike most other types in Rust, the futures Rust creates for async blocks can end up with references to themselves in \
  the fields of any given variant.\n\n\
  By default, though, any object that has a reference to itself is unsafe to move, because references always point to the \
  actual memory address of whatever they refer to.\n\
  If you move the data structure itself, those internal references will be left pointing to the old location.\n\
  However, that memory location is now invalid. For one thing, its value will not be updated when you make changes to the \
  data structure.\n\
  For another—more important—thing, the computer is now free to reuse that memory for other purposes!\n\
  You could end up reading completely unrelated data later.
  ",
  "Moving futures".bright_magenta().bold(),
  "join_all".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {1} is a marker trait, similar to the {2} and {3} traits we saw in Chapter 16, and thus has no functionality of its own.\n\
  Marker traits exist only to tell the compiler it's safe to use the type implementing a given trait in a particular context.\n\
  {1} informs the compiler that a given type does not need to uphold any guarantees about whether the value in question can be \
  safely moved.
  ",
  "The Unpin trait".bright_magenta().bold(),
  "Unpin".bright_yellow().bold(),
  "Send".bright_yellow().bold(),
  "Sync".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Most types are perfectly safe to move around, even if they happen to be behind a Pin pointer. \
  We only need to think about pinning when items have internal references.\n\
  {solid_disc} Primitive values such as numbers and Booleans are safe because they obviously don't have any internal references.\n\
  {solid_disc} The compiler implements {2} automatically for all types where it can prove it is safe.\n\
  {solid_disc} Futures may have internal references, so they don't automatically implement {2}.\n\
  {solid_disc} {1} and {2} are mostly important for building lower-level libraries, or when you're building a runtime itself, \
  rather than for day-to-day Rust code.\n\
  {solid_disc} References always point to the actual memory address of whatever they refer to.
  ",
  "REMEMBER".bright_white().bold(),
  "Pin".bright_yellow().bold(),
  "Unpin".bright_yellow().bold(),
  )
}

// Header: The Stream Trait. Abbreviated as tst.
fn tst_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("The Stream Trait");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch17-05-traits-for-async.html#the-stream-trait".bright_cyan(),
  )
}








