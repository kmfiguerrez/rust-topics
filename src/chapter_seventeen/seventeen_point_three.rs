use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Yielding Control to the Runtime", ycttr_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Yielding Control to the Runtime. Abbreviated as ycttr.
fn ycttr_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Yielding Control to the Runtime");

  println!(
  "Recall from the “Our First Async Program” section that at each await point, Rust gives a runtime a chance to pause the task and \
  switch to another one if the future being awaited isn't ready.\n\
  The inverse is also true: Rust only pauses async blocks and hands control back to a runtime at an await point.\n\
  Everything between await points is synchronous.\n\n\
  That means if you do a bunch of work in an async block without an await point, that future will block any other futures from \
  making progress.\n\
  You may sometimes hear this referred to as one future {0} other futures.\n\
  In some cases, that may not be a big deal. However, if you are doing some kind of expensive setup or long-running work, or \
  if you have a future that will keep doing some particular task indefinitely, you'll need to think about when and where to \
  hand control back to the runtime.\n\n\
  Let's simulate a long-running operation to illustrate the starvation problem, then explore how to solve it.\n\
  Listing 17-14 introduces a {1} function.\n\
  See Listing 17:14:{2}, for code sample.\n\n\
  This code uses {3} instead of {4} so that calling {1} will block the current thread for some number of milliseconds.\n\
  We can use {1} to stand in for real-world operations that are both long-running and blocking.\n\n\
  In Listing 17-15, we use slow to emulate doing this kind of CPU-bound work in a pair of futures.\n\
  See Listing 17-15: {5}, for code sample and complete reading.\n\n\
  ",
  "starving".italic().bold(),
  "slow".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-03-more-futures.html#listing-17-14".bright_cyan(),
  "std::thread::sleep".bright_yellow().bold(),
  "trpl::sleep".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-03-more-futures.html#listing-17-15".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  Using {1} is another way of handing off control to the runtime instead of using {2} which is more clearer about the actual \
  intent and can be significantly faster than using {2}, because timers such as the one used by {2} often have limits on how \
  granular they can be.\n\
  The version of {2} we are using, for example, will always sleep for at least a millisecond, even if we pass it a {3} of \
  one nanosecond.\n\
  Again, modern computers are fast: they can do a lot in one millisecond!
  ",
  "The trpl::yield_now".bright_magenta().bold(),
  "trpl::yield_now".bright_yellow().bold(),
  "trpl::sleep".bright_yellow().bold(),
  "Duration".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} if you do a bunch of work in an async block without an await point, that future will block any other futures from \
  making progress.\n\
  {solid_disc} Always measure to see what your code's actual performance bottlenecks are.\n\
  {solid_disc} The implementation of {1} is not fair: it always polls arguments in the order in which they are passed \
  (other select implementations will randomly choose which argument to poll first).
  ",
  "REMEMBER".bright_white().bold(),
  "trpl::select".bright_yellow().bold(),
  )
}