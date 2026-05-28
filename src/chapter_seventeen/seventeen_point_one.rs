use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Our First Async Program", ofap_content),
    chapter::SubHeader::new("Defining the page_title Function", dtpf_content),
    chapter::SubHeader::new("Executing an Async Function with a Runtime", eafwr_content),
    chapter::SubHeader::new("Racing Two URLs Against Each Other Concurrently", rtuaeoc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Fundamentals of Asynchronous Programming");

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

  menu::subheader_title("Section Introduction: Futures and the Async Syntax");

  println!(
  "The key elements of asynchronous programming in Rust are {0} and Rust's {1} and {2} keywords.\n\n\
  {6}\n\n\
  A {0} is a value that may not be ready now but will become ready at some point in the future.\n\
  (This same concept shows up in many languages, sometimes under other names such as {3} or {4}.)\n\
  Rust provides a {5} trait as a building block so that different async operations can be implemented with different data \
  structures but with a common interface.\n\
  In Rust, futures are types that implement the {5} trait.\n\
  Each future holds its own information about the progress that has been made and what “ready” means.
  ",
  "futures".italic().bold(),
  "async".bright_yellow().bold(),
  "await".bright_yellow().bold(),
  "task".italic().bold(),
  "promise".italic().bold(),
  "Future".bright_yellow().bold(),
  "future".bright_magenta().bold()
  );

  println!(
  "{0}\n\n\
  You can apply the {1} keyword to blocks and functions to specify that they can be interrupted and resumed.\n\
  Within an async block or async function, you can use the {2} keyword to await a future (that is, wait for it to become ready).\n\
  Any point where you await a future within an async block or function is a potential spot for that block or function to pause \
  and resume.\n\
  The process of checking with a future to see if its value is available yet is called {3}.
  ",
  "The Async Keyword".bright_magenta().bold(),
  "async".bright_yellow().bold(),
  "await".bright_yellow().bold(),
  "polling".italic().bold(),
  );

  println!(
  "Some other languages, such as C# and JavaScript, also use {0} and {1} keywords for async programming.\n\
  If you're familiar with those languages, you may notice some significant differences in how Rust handles the syntax.\n\
  That's for good reason, as we'll see!\n\n\
  When writing async Rust, we use the {0} and {1} keywords most of the time.\n\
  Rust compiles them into equivalent code using the {2} trait, much as it compiles for loops into equivalent code using the {3} \
  trait.\n\
  Because Rust provides the {2} trait, though, you can also implement it for your own data types when you need to.\n\
  Many of the functions we'll see throughout this chapter return types with their own implementations of {2}.\n\
  We'll return to the definition of the trait at the end of the chapter and dig into more of how it works, but this is enough \
  detail to keep us moving forward.
  ",
  "async".bright_yellow().bold(),
  "await".bright_yellow().bold(),
  "Future".bright_yellow().bold(),
  "Iterator".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} In Rust, futures are types that implement the {1} trait. \
  Each future holds its own information about the progress that has been made and what “ready” means.\n\
  {solid_disc} The process of checking with a future to see if its value is available yet is called {2}.
  {solid_disc} you can use the {3} keyword to await a {4} (that is, wait for it to become ready).
  ",
  "REMEMBER".bright_white().bold(),
  "Future".bright_yellow().bold(),
  "polling".italic().bold(),
  "await".bright_yellow().bold(),
  "future".italic().bold(),
  )  

}

// Header: Our First Async Program. Abbreviated as ofap.
fn ofap_content() {
  menu::subheader_title("Our First Async Program");

  println!(
  "See: {0
  }, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html#our-first-async-program".bright_cyan()
  );
}

// Header: Defining the page_title Function. Abbreviated as dtpf.
fn dtpf_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Defining the page_title Function");

  println!(
  "{0}\n\n\
  Let's start by writing a function that takes one page URL as a parameter, makes a request to it, and returns the text of the \
  {1} element (see Listing 17-1).\n\n\
  See Listing 17-1:{2}, for code sample and complete reading.\n\n\
  futures in Rust are lazy: they don't do anything until you ask them to with the {3} keyword.\n\
  (In fact, Rust will show a compiler warning if you don't use a future.)\n\
  This might remind you of the discussion of iterators in the “Processing a Series of Items with Iterators”: {4} section in \
  Chapter 13.\n\
  Iterators do nothing unless you call their {5} method—whether directly or by using {6} loops or methods such as {7} that use \
  {5} under the hood.\n\
  Likewise, futures do nothing unless you explicitly ask them to.\n\
  This laziness allows Rust to avoid running async code until it's actually needed.\n\n\
  When Rust sees a {8} marked with the {9} keyword, it compiles it into a unique, anonymous data type that implements the \
  {10} trait.\n\
  When Rust sees a function marked with {9}, it compiles it into a non-async function whose body is an async block.\n\
  An async function's return type is the type of the anonymous data type the compiler creates for that async block.
  ",
  "Defining the page_title Function".bright_magenta().bold(),
  "<title>".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html#listing-17-1".bright_cyan(),
  "await".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html".bright_cyan(),
  "next".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  "map".bright_yellow().bold(),
  "block".italic().bold(),
  "async".bright_yellow().bold(),
  "Future".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} futures in Rust are {1}: they don't do anything until you ask them to with the {2} keyword.\n\
  {solid_disc} Note that Rust's {2} keyword goes after the expression you're awaiting, not before it, \
  it makes chains of methods much nicer to work with.\n\
  {solid_disc} When Rust sees a {3} marked with the {4} keyword, it compiles it into a unique, anonymous data type that \
  implements the {5} trait.\n\
  {solid_disc} When Rust sees a {6} marked with {4}, it compiles it into a non-async function whose body is an async block.\n\
  {solid_disc} An {4} function's return type is the type of the anonymous data type the compiler creates for that async block.\n\
  {solid_disc} Remember that blocks are expressions.
  ",
  "REMEMBER".bright_white().bold(),
  "lazy".italic().bold(),
  "await".bright_yellow().bold(),
  "block".italic().bold(),
  "async".bright_yellow().bold(),
  "Future".bright_yellow().bold(),
  "function".italic().bold(),
)
}

// Header: Executing an Async Function with a Runtime. Abbreviated as eafwr.
fn eafwr_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Executing an Async Function with a Runtime");

  println!(
  "To start, we'll get the title for a single page, shown in Listing 17-3. Unfortunately, this code doesn't compile yet.\n\n\
  See Listing 17-3:{0}, for code sample and complete reading.\n\n\
  The reason {1} can't be marked {2} is that async code needs a runtime: a Rust crate that manages the details of executing \
  asynchronous code.\n\
  A program's {1} function can initialize a runtime, but it's not a runtime itself.\n\
  (We'll see more about why this is the case in a bit.)\n\
  Every Rust program that executes async code has at least one place where it sets up a runtime that executes the futures.\n\n\
  Most languages that support async bundle a runtime, but Rust does not.\n\
  Instead, there are many different async runtimes available, each of which makes different tradeoffs suitable to the use case \
  it targets.\n\
  For example, a high-throughput web server with many CPU cores and a large amount of RAM has very different needs than a \
  microcontroller with a single core, a small amount of RAM, and no heap allocation ability.\n\
  The crates that provide those runtimes also often supply async versions of common functionality such as file or network I/O.\n\n\
  For example, the {3}'s {4} function, behind the scenes, sets up a runtime using the {5} crate that's used to run \
  the future passed in. Once the future completes, {4} returns whatever value the future produced.
  ",
  "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html#listing-17-3".bright_cyan(),
  "main".bright_yellow().bold(),
  "async".bright_yellow().bold(),
  "trpl".bright_yellow().bold(),
  "block on".bright_yellow().bold(),
  "tokio".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Each {1}—that is, every place where the code uses the {2} keyword—represents a place where control is handed back to \
  the runtime.\n\
  To make that work, Rust needs to keep track of the state (state means codes that use the {2} keword) involved in the async \
  block so that the runtime could kick off some other work (other work means other async function or block) and then come back \
  when it's ready to try advancing the first one again.\n\
  This is an invisible state machine - a compiler-generated enum used to keep track where an async task pauses.\n\n\
  You can think of:\n\
  {solid_disc} An async function or block is an async task.\n\
  {solid_disc} Each await point pauses the entire async function, and the runtime can run other async function outside the body \
  of the current async function.\n\
  {solid_disc} Rust transform an async task into an enum that can be in one state at a time to keep track where it pauses \
  or where to resume and remembers local variables.
  ",
  "The Invisible State Machine".bright_magenta().bold(),
  "await point".italic().bold(),
  "await".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Now you can see why the compiler stopped us from making {1} itself an async function back in Listing 17-3.n\n\
  If {1} were an async function, something else would need to manage the state machine for whatever future {1} returned, but {1} \
  is the starting point for the program! Instead, we called the {2} function in {1} to set up a runtime and run the future \
  returned by the async block until it's done.
  ",
  "The main function is the starting point of the program".bright_magenta().bold(),
  "main".bright_yellow().bold(),
  "trpl::block_on".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} The only place we can use the {1} keyword is in {2} functions or blocks.\n\
  {solid_disc} In Rust, async codes need a {3}: a Rust crate that manages the details of executing asynchronous code.\n\
  {solid_disc} Every Rust program that executes async code has at least one place where it sets up a runtime that \
  executes the futures.\n\
  {solid_disc} The crates that provide those runtimes also often supply async versions of common functionality \
  such as file or network I/O.\n\
  {solid_disc} Each {4}—that is, every place where the code uses the {1} keyword—represents a place where control is handed back to \
  the runtime.\n\
  {solid_disc} Rust uses the {5} to keep track where an async tasks pauses or where to resume.\n\
  {solid_disc} Note that some runtimes provide macros so you can write an async {6} function. \
  Those macros rewrite {7} to be a normal {8}.
  ",
  "REMEMBER".bright_white().bold(),
  "await".bright_yellow().bold(),
  "async".bright_yellow().bold(),
  "runtime".italic().bold(),
  "await point".italic().bold(),
  "invisible state machine".bright_white().bold(),
  "main".bright_yellow().bold(),
  "async fn main()".bright_yellow().bold(),
  "fn main".bright_yellow().bold(),
  )
}

// Header: Racing Two URLs Against Each Other Concurrently. Abbreviated as rtuaeoc.
fn rtuaeoc_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Racing Two URLs Against Each Other Concurrently");

  println!(
  "See: {0}, for complete reading.
  ",
  "https://doc.rust-lang.org/book/ch17-01-futures-and-syntax.html#racing-two-urls-against-each-other-concurrently".bright_cyan()
  )
}











