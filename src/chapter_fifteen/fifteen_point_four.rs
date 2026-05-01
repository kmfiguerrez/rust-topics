use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 3];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Sharing Data", sd_content),
    chapter::SubHeader::new("Cloning to Increase the Reference Count", ctitrc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Rc<T>, the Reference-Counted Smart Pointer");

  println!(
  "In the majority of cases, ownership is clear: You know exactly which variable owns a given value.\n\
  However, there are cases when a single value might have multiple owners.\n\
  For example, in graph data structures, multiple edges might point to the same node, and that node is conceptually owned by all \
  of the edges that point to it.\n\
  A node shouldn't be cleaned up unless it doesn't have any edges pointing to it and so has no owners.\n\n\
  You have to enable multiple ownership explicitly by using the Rust type {0}, which is an abbreviation for {1}.\n\
  The {0} type keeps track of the number of references to a value to determine whether or not the value is still in use.\n\
  If there are zero references to a value, the value can be cleaned up without any references becoming invalid.\n\n\
  Imagine {0} as a TV in a family room.\n\
  When one person enters to watch TV, they turn it on.\n\
  Others can come into the room and watch the TV.\n\
  When the last person leaves the room, they turn off the TV because it's no longer being used.\n\
  If someone turns off the TV while others are still watching it, there would be an uproar from the remaining TV watchers!\n\n\
  We use the {0} type when we want to allocate some data on the heap for multiple parts of our program to read and we can't \
  determine at compile time which part will finish using the data last.\n\
  If we knew which part would finish last, we could just make that part the data's owner, and the normal ownership rules enforced \
  at compile time would take effect.\n\n\
  Note that {0} is only for use in single-threaded scenarios.\n\
  When we discuss concurrency in Chapter 16, we'll cover how to do reference counting in multithreaded programs.
  ",
  "Rc<T>".bright_yellow().bold(),
  "reference counting".italic().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} We use the {1} type when we want to allocate some data on the heap for multiple parts of our program to read and we \
  can't determine at compile time which part will finish using the data last.\n\
  {solid_disc} Note that {1} is only for use in single-threaded scenarios.
  ",
  "REMEMBER".bright_white().bold(),
  "Rc<T>".bright_yellow().bold(),
  )
}

// Header: Sharing Data. Abbreviated as sd.
fn sd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Sharing Data");

  println!(
  "See: {0}, for complete reading.\n\n\
  In Listing 15-18, We could have called {1} rather than {2}, but Rust's convention is to use {3} in this case.\n\
  The implementation of {3} doesn't make a deep copy of all the data like most types' implementations of {4} do.\n\
  The call to {3} only increments the reference count, which doesn't take much time.\n\
  Deep copies of data can take a lot of time.\n\
  By using {3} for reference counting, we can visually distinguish between the deep-copy kinds of clones and the kinds of clones \
  that increase the reference count.\n\
  When looking for performance problems in the code, we only need to consider the deep-copy clones and can disregard calls to {3}.
  ",
  "https://doc.rust-lang.org/book/ch15-04-rc.html#sharing-data".bright_cyan(),
  "a.clone()".bright_yellow().bold(),
  "Rc::clone(&a)".bright_yellow().bold(),
  "Rc::clone".bright_yellow().bold(),
  "clone".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} We need to add a use statement to bring {1} into scope because it's not in the prelude.\n\
  {solid_disc} The implementation of {2} doesn't make a deep copy of all the data like most types' implementations of {3} do.\n\
  {solid_disc} The call to {2} only increments the reference count, which doesn't take much time. \
  Deep copies of data can take a lot of time.
  ",
  "REMEMBER".bright_white().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Rc::clone".bright_yellow().bold(),
  "clone".bright_yellow().bold(),
  )  

}

// Header: Cloning to Increase the Reference Count. Abbreviated as ctitrc.
fn ctitrc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Cloning to Increase the Reference Count");

  println!(
  "See: {0}, for complete reading.\n\n\
  We can see that the {1} in {2} has an initial reference count of 1; then, each time we call {3}, the count goes up by 1.\n\
  When {4} goes out of scope, the count goes down by 1.\n\
  We don't have to call a function to decrease the reference count like we have to call {3} to increase the reference count: \
  The implementation of the {5} trait decreases the reference count automatically when an {6} value goes out of scope.\n\n\
  What we can't see in this example is that when {7} and then {2} go out of scope at the end of {8}, the count is 0, and the \
  {1} is cleaned up completely.\n\
  Using {6} allows a single value to have multiple owners, and the count ensures that the value remains valid as long as any of \
  the owners still exist.
  ",
  "https://doc.rust-lang.org/book/ch15-04-rc.html#cloning-to-increase-the-reference-count".bright_cyan(),
  "Rc<List>".bright_yellow().bold(),
  "a".bright_yellow().bold(),
  "Rc::clone".bright_yellow().bold(),
  "c".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "b".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  );

  println!(
  "Via immutable references, {0} allows you to share data between multiple parts of your program for reading only.\n\
  If {0} allowed you to have multiple mutable references too, you might violate one of the borrowing rules discussed in Chapter 4: \
  Multiple mutable borrows to the same place can cause data races and inconsistencies.\n\
  But being able to mutate data is very useful! In the next section, we'll discuss the interior mutability pattern and the \
  {1} type that you can use in conjunction with an {0} to work with this immutability restriction.
  ",
  "Rc<T>".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} We use {1} to find out how many owners there are to a value in an {2} - reference count.\n\
  {solid_disc} When the reference count is 0, the value is cleaned up.\n\
  {solid_disc} This function is named {3} rather than count because the {2} type also has a {4}.\n\
  {solid_disc} Via immutable references, {2} allows you to share data between multiple parts of your program for reading only.
  ",
  "REMEMBER".bright_white().bold(),
  "Rc::strong_count".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "strong_count".bright_yellow().bold(),
  "weak_count".bright_yellow().bold(),
  )
}











