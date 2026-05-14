use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Transferring Ownership Between Threads", tobt_content),
    chapter::SubHeader::new("Accessing from Multiple Threads", afmt_content),
    chapter::SubHeader::new("Implementing Send and Sync Manually Is Unsafe", isasmiu_content),
    chapter::SubHeader::new("Summary", s_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction: Extensible Concurrency with Send and Sync");

  println!(
  "Interestingly, almost every concurrency feature we've talked about so far in this chapter has been part of the standard library, \
  not the language.\n\
  Your options for handling concurrency are not limited to the language or the standard library; you can write your own concurrency \
  features or use those written by others.\n\n\
  However, among the key concurrency concepts that are embedded in the language rather than the standard library are the {0} traits \
  {1} and {2}.
  ",
  "std::marker".bright_yellow().bold(),
  "Send".bright_yellow().bold(),
  "Sync".bright_yellow().bold()
  )
}

// Header: Transferring Ownership Between Threads. Abbreviated as tobt.
fn tobt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Transferring Ownership Between Threads");

  println!(
  "The {0} marker trait indicates that ownership of values of the type implementing {0} can be transferred between threads.\n\
  Almost every Rust type implements {0}, but there are some exceptions, including {1}: This cannot implement {0} because if you \
  cloned an {1} value and tried to transfer ownership of the clone to another thread, both threads might update the reference \
  count at the same time.\n\
  For this reason, {1} is implemented for use in single-threaded situations where you don't want to pay the thread-safe \
  performance penalty.\n\n\
  Therefore, Rust's type system and trait bounds ensure that you can never accidentally send an {1} value across threads unsafely.\n\
  When we tried to do this in Listing 16-14, we got the error {2}.\n\
  When we switched to {3}, which does implement {0}, the code compiled.\n\n\
  Any type composed entirely of {0} types is automatically marked as {0} as well.\n\
  Almost all primitive types are {0}, aside from raw pointers, which we'll discuss in Chapter 20.
  ",
  "Send".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "the trait `Send` is not implemented for `Rc<Mutex<i32>>`".bright_yellow().bold(),
  "Arc<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Almost every Rust type implements {1}, but there are some exceptions, including {2}.\n\
  {solid_disc} Any type composed entirely of {1} types is automatically marked as {1} as well.\n\
  {solid_disc} Almost all primitive types are {1}, aside from raw pointers, which we'll discuss in Chapter 20.
  ",
  "REMEMBER".bright_white().bold(),
  "Send".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  )
}

// Header: Accessing from Multiple Threads. Abbreviated as afmt.
fn afmt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Accessing from Multiple Threads");

  println!(
  "The {0} marker trait indicates that it is safe for the type implementing {0} to be referenced from multiple threads.\n\
  In other words, any type {1} implements {0} if {2} (an immutable reference to {1}) implements {3}, meaning the reference can be \
  sent safely to another thread.\n\
  Similar to {3}, primitive types all implement {0}, and types composed entirely of types that implement {0} also implement {0}.\n\n\
  The smart pointer {4} also doesn't implement {0} for the same reasons that it doesn't implement {3}.\n\
  The {5} type (which we talked about in Chapter 15) and the family of related {6} types don't implement {0}.\n\
  The implementation of borrow checking that {5} does at runtime is not thread-safe.\n\
  The smart pointer {7} implements {0} and can be used to share access with multiple threads, as you saw in \
  “Shared Access to {7}”.
  ",
  "Sync".bright_yellow().bold(),
  "T".bright_yellow().bold(),
  "&T".bright_yellow().bold(),
  "Send".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Cell<T>".bright_yellow().bold(),
  "Mutex<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Similar to {1}, primitive types all implement {2}, and types composed entirely of types that implement {2} \
  also implement {2}.\n\
  {solid_disc} The {3} type (which we talked about in Chapter 15) and the family of related {4} types don't implement {2}.\n\
  {solid_disc} The implementation of borrow checking that {3} does at runtime is not thread-safe.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "Send".bright_yellow().bold(),
  "Sync".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Cell<T>".bright_yellow().bold(),
  )
}

// Header: Implementing Send and Sync Manually Is Unsafe. Abbreviated as isasmiu.
fn isasmiu_content() {
  menu::subheader_title("Implementing Send and Sync Manually Is Unsafe");

  println!(
  "Because types composed entirely of other types that implement the {0} and {1} traits also automatically implement {0} and {1}, \
  we don't have to implement those traits manually.\n\
  As marker traits, they don't even have any methods to implement.\n\
  They're just useful for enforcing invariants related to concurrency.\n\n\
  Manually implementing these traits involves implementing unsafe Rust code.\n\
  We'll talk about using unsafe Rust code in Chapter 20; for now, the important information is that building new concurrent types \
  not made up of {0} and {1} parts requires careful thought to uphold the safety guarantees.\n\
  “The Rustonomicon”: {2} has more information about these guarantees and how to uphold them.
  ",
  "Send".bright_yellow().bold(),
  "Sync".bright_yellow().bold(),
  "https://doc.rust-lang.org/nomicon/index.html".bright_cyan()
  )
}

// Header: Summary. Abbreviated as s.
fn s_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Summary");

  println!(
  "This isn't the last you'll see of concurrency in this book: The next chapter focuses on async programming, and the project \
  in Chapter 21 will use the concepts in this chapter in a more realistic situation than the smaller examples discussed here.\n\n\
  As mentioned earlier, because very little of how Rust handles concurrency is part of the language, many concurrency solutions \
  are implemented as crates.\n\
  These evolve more quickly than the standard library, so be sure to search online for the current, state-of-the-art crates to \
  use in multithreaded situations.\n\n\
  The Rust standard library provides channels for message passing and smart pointer types, such as {0} and {1}, that are safe to \
  use in concurrent contexts.\n\
  The type system and the borrow checker ensure that the code using these solutions won't end up with data races or invalid \
  references.\n\
  Once you get your code to compile, you can rest assured that it will happily run on multiple threads without the kinds of \
  hard-to-track-down bugs common in other languages.\n\
  Concurrent programming is no longer a concept to be afraid of: Go forth and make your programs concurrent, fearlessly!
  ",
  "Mutex<T>".bright_yellow().bold(),
  "Arc<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The Rust standard library provides channels for message passing and smart pointer types, such as {1} and {2}, \
  that are safe to use in concurrent contexts.\n\
  {solid_disc} The type system and the borrow checker ensure that the code using these solutions won't end up with data races \
  or invalid references.
  ",
  "REMEMBER".bright_white().bold(),
  "Mutex<T>".bright_yellow().bold(),
  "Arc<T>".bright_yellow().bold(),
  )  

}









