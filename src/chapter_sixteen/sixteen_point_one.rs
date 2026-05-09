use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Creating a New Thread with spawn", cantws_content),
    chapter::SubHeader::new("Waiting for All Threads to Finish", wfattf_content),
    chapter::SubHeader::new("Using move Closures with Threads", umcwt_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Fearless Concurrency");

  println!(
  "Handling concurrent programming safely and efficiently is another of Rust's major goals.\n\
  {0}, in which different parts of a program execute independently, and {1}, in which different parts of a program execute at \
  the same time, are becoming increasingly important as more computers take advantage of their multiple processors.\n\
  Historically, programming in these contexts has been difficult and error-prone.\n\
  Rust hopes to change that.\n\n\
  Initially, the Rust team thought that ensuring memory safety and preventing concurrency problems were two separate challenges \
  to be solved with different methods.\n\
  Over time, the team discovered that the ownership and type systems are a powerful set of tools to help manage memory safety and \
  concurrency problems!\n\
  By leveraging ownership and type checking, many concurrency errors are compile-time errors in Rust rather than runtime errors.\n\
  Therefore, rather than making you spend lots of time trying to reproduce the exact circumstances under which a runtime \
  concurrency bug occurs, incorrect code will refuse to compile and present an error explaining the problem.\n\
  As a result, you can fix your code while you're working on it rather than potentially after it has been shipped to production.\n\
  We've nicknamed this aspect of Rust {2}.\n\
  Fearless concurrency allows you to write code that is free of subtle bugs and is easy to refactor without introducing new bugs.
  ",
  "Concurrent programming".italic().bold(),
  "Parallel programming".italic().bold(),
  "fearless concurrency".italic().bold(),
  );

  println!(
  "Note: For simplicity's sake, we'll refer to many of the problems as {0} rather than being more precise by \
  saying {1}. For this chapter, please mentally substitute {1} whenever we use {0}.\n\
  In the next chapter, where the distinction matters more, we'll be more specific.
  ",
  "concurrent".italic().bold(),
  "concurrent and/or parallel".italic().bold(),
  );

  println!(
  "Many languages are dogmatic about the solutions they offer for handling concurrent problems.\n\
  For example, Erlang has elegant functionality for message-passing concurrency but has only obscure ways to share state between \
  threads.\n\
  Supporting only a subset of possible solutions is a reasonable strategy for higher-level languages because a higher-level language \
  promises benefits from giving up some control to gain abstractions.\n\
  However, lower-level languages are expected to provide the solution with the best performance in any given situation and have \
  fewer abstractions over the hardware.\n\
  Therefore, Rust offers a variety of tools for modeling problems in whatever way is appropriate for your situation and requirements.\n\n\
  Here are the topics we'll cover in this chapter:\n\n\
  {solid_disc} How to create threads to run multiple pieces of code at the same time\n\
  {solid_disc} {0} concurrency, where channels send messages between threads\n\
  {solid_disc} {1} concurrency, where multiple threads have access to some piece of data\n\
  {solid_disc} The {2} and {3} traits, which extend Rust's concurrency guarantees to user-defined types as well as types \
  provided by the standard library\n\
  ",
  "Message-passing".italic().bold(),
  "Shared-state".italic().bold(),
  "Sync".bright_yellow().bold(),
  "Send".bright_yellow().bold(),
  )

}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Using Threads to Run Code Simultaneously");

  println!(
  "In most current operating systems, an executed program's code is run in a process, and the operating system will manage \
  multiple processes at once.\n\
  Within a program, you can also have independent parts that run simultaneously.\n\
  The features that run these independent parts are called {0}.\n\
  For example, a web server could have multiple threads so that it can respond to more than one request at the same time.\n\n\
  Splitting the computation in your program into multiple threads to run multiple tasks at the same time can improve performance, \
  but it also adds complexity.\n\
  Because threads can run simultaneously, there's no inherent guarantee about the order in which parts of your code on different \
  threads will run.\n\
  This can lead to problems, such as:\n\
  {solid_disc} Race conditions, in which threads are accessing data or resources in an inconsistent order\n\
  {solid_disc} Deadlocks, in which two threads are waiting for each other, preventing both threads from continuing\n\
  {solid_disc} Bugs that only happen in certain situations and are hard to reproduce and fix reliably\n\n\
  Rust attempts to mitigate the negative effects of using threads, but programming in a multithreaded context still takes \
  careful thought and requires a code structure that is different from that in programs running in a single thread.\n\n\
  Programming languages implement threads in a few different ways, and many operating systems provide an API the programming \
  language can call for creating new threads.\n\
  The Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one operating system thread per one \
  language thread.\n\
  There are crates that implement other models of threading that make different trade-offs to the 1:1 model.\n\
  (Rust's async system, which we will see in the next chapter, provides another approach to concurrency as well.)
  ",
  "threads".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} allow independent parts of a program to run simultaneously, which can improve performance\n\
  {solid_disc} he Rust standard library uses a 1:1 model of thread implementation, whereby a program uses one \
  operating system thread per one language thread.
  ",
  "REMEMBER".bright_white().bold(),
  "Threads".italic().bold(),
  )
}

// Header: Creating a New Thread with spawn. Abbreviated as cantws.
fn cantws_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Creating a New Thread with spawn");

  println!(
  "To create a new thread, we call the {0} function and pass it a closure (we talked about closures in Chapter 13) \
  containing the code we want to run in the new thread.\n\
  The example in Listing 16-1 prints some text from a main thread and other text from a new thread.\n\n\
  See Listing 16-1: {1}, for code sample and complete reading.\n\n\
  Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have \
  finished running.\n\
  The output from this program might be a little different every time.\n\n\
  The calls to {2} force a thread to stop its execution for a short duration, allowing a different thread to run.\n\
  The threads will probably take turns, but that isn't guaranteed: It depends on how your operating system schedules the threads.\n\
  In this run, the main thread printed first, even though the print statement from the spawned thread appears first in the code.
  ",
  "thread::spawn".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-01-threads.html#listing-16-1".bright_cyan(),
  "thread::sleep".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Note that when the main thread of a Rust program completes, all spawned threads are shut down, whether or not they have \
  finished running.\n\
  {solid_disc} The Operting System's thread scheduler determines which thread runs at any given time, so the output from this \
  program might be a little different every time.
  ",
  "REMEMBER".bright_white().bold()
  )

}

// Header: Waiting for All Threads to Finish. Abbreviated as wfattf.
fn wfattf_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Waiting for All Threads to Finish");

  println!(
  "The code in Listing 16-1 not only stops the spawned thread prematurely most of the time due to the main thread ending, \
  but because there is no guarantee on the order in which threads run, {0}\n\n\
  We can fix the problem of the spawned thread not running or of it ending prematurely by saving the return value of {1} in a \
  variable.\n\
  The return type of {1} is {2}.\n\
  A {2} is an owned value that, when we call the {3} method on it, will wait for its thread to finish.\n\
  Listing 16-2 shows how to use the {2} of the thread we created in Listing 16-1 and how to call {3} to make sure the spawned \
  thread finishes before {4} exits.\n\n\
  See Listing 16-2: {5}, for code sample, outputs and complete reading.\n\n\
  Calling {3} on the handle blocks the thread currently running (The current thread mentioned here is the main thread because \
  the {3} method is called in the {4} function) until the thread represented by the handle terminates.\n\
  {6} a thread means that thread is prevented from performing work or exiting.
  ",
  "we also can't guarantee that the spawned thread will get to \
  run at all!".bright_white().bold(),
  "thread::spawn".bright_yellow().bold(),
  "JoinHandle<T>".bright_yellow().bold(),
  "join".bright_yellow().bold(),
  "main".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-01-threads.html#listing-16-2".bright_cyan(),
  "Blocking".italic().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} There is no guarantee on the order in which threads run, we also can't guarantee that the spawned thread \
  will get to run at all!\n\
  {solid_disc} We can fix the problem of the spawned thread not running or of it ending prematurely by saving the return value \
  of {1} in a variable.\n\
  {solid_disc} The return type of {1} is {2}.\n\
  {solid_disc} A {2} is an owned value that, when we call the {3} method on it, will wait for its thread to finish.\n\
  {solid_disc} {4} a thread means that thread is prevented from performing work or exiting.\n\
  {solid_disc} The location at which you call the {3} method determines where the program will block while waiting for the \
  spawned thread to finish.
  ",
  "REMEMBER".bright_white().bold(),
  "thread::spawn".bright_yellow().bold(),
  "JoinHandle<T>".bright_yellow().bold(),
  "join".bright_yellow().bold(),
  "Blocking".italic().bold()
  )
}

// Header: Using move Closures with Threads. Abbreviated as umcwt.
fn umcwt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using move Closures with Threads");

  println!(
  "We'll often use the {0} keyword with closures passed to {1} because the closure will then take ownership of the values it uses \
  from the environment, thus transferring ownership of those values from one thread to another.\n\
  In “Capturing References or Moving Ownership”: {2}, in Chapter 13, we discussed {0} in the context of closures.\n\
  Now we'll concentrate more on the interaction between {0} and {1}.\n\n\
  See Listing 16-3: {3}, for code sample, outputs and complete reading.\n\n\
  Rust {4} how to capture {5}, and because {6} only needs a reference to {5}, the closure tries to borrow {5}.\n\
  However, there's a problem: Rust can't tell how long the spawned thread will run, so it doesn't know whether the reference \
  to {5} will always be valid.\n\n\
  Listing 16-4 provides a scenario that's more likely to have a reference to {5} that won't be valid.\n\n\
  See Listing 16-4: {7}, for code sample, outputs and complete reading.\n\n\
  If Rust allowed us to run the code in Listing 16-4, there's a possibility that the spawned thread would be immediately \
  put in the background without running at all.\n\
  The spawned thread has a reference to {5} inside, but the main thread immediately drops {5}, using the {7} function we \
  discussed in Chapter 15.\n\
  Then, when the spawned thread starts to execute, {5} is no longer valid, so a reference to it is also invalid. Oh no!\n\n\
  By adding the {0} keyword before the closure, we force the closure to take ownership of the values it's using rather than \
  allowing Rust to infer that it should borrow the values.\n\
  The modification to Listing 16-3 shown in Listing 16-5 will compile and run as we intend.\n\n
  See Listing 16-5: {8}, for code sample, outputs and complete reading.
  ",
  "move".bright_yellow().bold(),
  "thread::spawn".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch13-01-closures.html#capturing-references-or-moving-ownership".bright_cyan(),
  "https://doc.rust-lang.org/book/ch16-01-threads.html#listing-16-3".bright_cyan(),
  "infers".italic().bold(),
  "v".bright_yellow().bold(),
  "println!".bright_yellow().bold(),
  "drop".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-01-threads.html#listing-16-5".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Closure has two environments: the environment where the closure is defined and the environment where \
  the closure is called (closure's environment).\n\
  {solid_disc} By default, Rust will infer how the closure should capture values from the environment in which the closure is defined, \
  and the closure will borrow those values.\n\
  {solid_disc} The {1} keyword overrides Rust's conservative default of borrowing; it doesn't let us violate the ownership rules.
  ",
  "REMEMBER".bright_white().bold(),
  "move".bright_yellow().bold()
  )
}












