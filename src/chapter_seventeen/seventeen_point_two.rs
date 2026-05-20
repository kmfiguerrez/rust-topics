use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Creating a New Task with spawn_task", cantws_content),
    chapter::SubHeader::new("Sending Data Between Two Tasks Using Message Passing", sdbttump_content),
    chapter::SubHeader::new("Code Within One Async Block Executes Linearly", cwoabel_content),
    chapter::SubHeader::new("Moving Ownership Into an Async Block", moiaab_content),
    chapter::SubHeader::new("Joining a Number of Futures with the join! Macro", janofwtjm_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Applying Concurrency with Async");

  println!(
  "In this section, we'll apply async to some of the same concurrency challenges we tackled with threads in Chapter 16.\n\
  Because we already talked about a lot of the key ideas there, in this section we'll focus on what's different between threads \
  and futures.\n\n\
  In many cases, the APIs for working with concurrency using async are very similar to those for using threads.\n\
  In other cases, they end up being quite different.\n\
  Even when the APIs look similar between threads and async, they often have different behavior—and they nearly always have \
  different performance characteristics.
  ")
}

// Header: Creating a New Task with spawn_task. Abbreviated as cantws.
fn cantws_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Creating a New Task with spawn_task");

  println!(
  "The first operation we tackled in the “Creating a New Thread with spawn”: {0} section in Chapter 16 was counting up on two \
  separate threads.\n\
  Let's do the same using async.\n\
  The {1} crate supplies a {2} function that looks very similar to the {3} API, and a {4} function that is an async version of \
  the {5} API. We can use these together to implement the counting example, as shown in Listing 17-6.\n\n\
  See Listing 17-6: {6}, for code sample and complete reading.\n\n\
  Note that the {2} does not spawn a new thread but rather create a new async task, hand that task to the async runtime \
  allow it to run concurrently with other async tasks.\n\
  Just like a thread, you should await it before the main async ends, otherwise it will be dropped whether or not it finishes.\n\
  See Listing 17-7: {7}, for code sample.\n\n\
  The {8} function used in Listing 17-8 is fair, meaning it checks each future equally often, alternating between them, \
  and never lets one race ahead if the other is ready.\n\
  See Listing 17-8: {9}, for code sample.
  ",
  "https://doc.rust-lang.org/book/ch16-01-threads.html#creating-a-new-thread-with-spawn".bright_cyan(),
  "trpl".bright_yellow().bold(),
  "spawn_task".bright_yellow().bold(),
  "thread::spawn".bright_yellow().bold(),
  "sleep".bright_yellow().bold(),
  "thread::sleep".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-6".bright_cyan(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-7".bright_cyan(),
  "trpl::join".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-8".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} With threads, the operating system decides which thread to check and how long to let it run.\n\
  {solid_disc} With async Rust, the runtime decides which task to check.\n\
  {solid_disc} Runtimes dont have to guarantee fairness for any given operation, and they often offer different \
  APIs to let you choose whether or not you want fairness.
  ",
  "REMEMBER".bright_white().bold(),
  )
}

// Header: Sending Data Between Two Tasks Using Message Passing. Abbreviated as sdbttump.
fn sdbttump_content() {
  let solid_disc = "\u{2022}";
 
  menu::subheader_title("Sending Data Between Two Tasks Using Message Passing");

  println!(
  "Sharing data between futures will also be familiar: we'll use message passing again, but this time with async versions of the \
  types and functions.\n\
  We'll take a slightly different path than we did in the “Transfer Data Between Threads with Message Passing”: {0} section in Chapter \
  16 to illustrate some of the key differences between {1} and {2} concurrency.\n\
  In Listing 17-9, we'll begin with just a single async block—not spawning a separate task as we spawned a separate thread.\n\n\
  See Listing 17-9:{3}, for code sample and complete reading.\n\n\
  Here, we use {4}, an async version of the multiple-producer, single-consumer channel API we used with threads back in \
  Chapter 16.\n\
  The async version of the API is only a little different from the thread-based version: it uses a mutable rather than an \
  immutable receiver {5}, and its {6} method produces a future we need to await rather than producing the value directly.
  ",
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html".bright_cyan(),
  "thread-based".italic().bold(),
  "futures-based".italic().bold(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-9".bright_cyan(),
  "trpl::channel".bright_yellow().bold(),
  "rx".bright_yellow().bold(),
  "recv".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  The synchronous {1} method in {2} blocks until it receives a message.\n\
  The {3} method does not, because it is async.\n\
  Instead of blocking, it hands control back to the runtime until either a message is received or the \
  send side of the channel closes.\n\
  By contrast, we don't await the {4} call, because it doesn't block.\n\
  It doesn't need to, because the channel we're sending it into is unbounded.\n\n\
  For more info about why the {4} doesn't have to wait and unbounded vs bounded channel, see: {5}
  ",
  "Synchronous vs Asynchronous operations".bright_magenta().bold(),
  "Receiver::recv".bright_yellow().bold(),
  "std::mpsc::channel".bright_yellow().bold(),
  "trpl::Receiver::recv".bright_yellow().bold(),
  "send".bright_yellow().bold(),
  "https://gemini.google.com/share/a39188d4420d".bright_cyan()
  );

  println!(
  "{0}: Because all of this async code runs in an async block in a {1} call, everything within it can avoid blocking.\n\
  However, the code outside it will block on the block_on function returning.\n\
  That's the whole point of the {1} function: it lets you choose where to block on some set of async code, and thus where to \
  transition between sync and async code.
  ",
  "Note".bright_white().bold(),
  "trpl::block_on".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  In Listing 16-10, we used a {1} loop to process all the items received from a synchronous channel.\n\
  Rust doesn't yet have a way to use a {1} loop with an {2} series of items, however, so we need to use a loop we haven't seen \
  before: the {5} conditional loop.\n\
  This is the loop version of the {3} construct we saw back in the “Concise Control Flow with if let and let...else”:{4} section \
  in Chapter 6.\n\
  The loop will continue executing as long as the pattern it specifies continues to match the value.
  ",
  "The while let".bright_magenta().bold(),
  "for".bright_yellow().bold(),
  "asynchronously produced".italic().bold(),
  "if let".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#sending-multiple-values".bright_cyan(),
  "while let".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Rust uses {1} for {2} series of items
  ",
  "REMEMBER".bright_white().bold(),
  "while let".bright_yellow().bold(),
  "asynchronously produced".italic().bold(),

  )
}

// Header: Code Within One Async Block Executes Linearly. Abbreviated as cwoabel.
fn cwoabel_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Code Within One Async Block Executes Linearly");

  println!(
  "Let's start by examining why the messages come in all at once after the full delay, rather than coming in with delays between \
  each one.\n\
  Within a given async block, the order in which {0} keywords appear in the code is also the order in which they're executed when \
  the program runs.\n\n\
  There's only one async block in Listing 17-10, so everything in it runs linearly.\n\
  There's still no concurrency.\n\
  All the {1} calls happen, interspersed with all of the {2} calls and their associated await points.\n\
  Only then does the {3} loop get to go through any of the {0} points on the {4} calls.\n\n\
  To get the behavior we want, where the sleep delay happens between each message, we need to put the {5} and {6} operations in \
  their own async blocks, as shown in Listing 17-11.\n\
  Then the runtime can execute each of them separately using {7}, just as in Listing 17-8.\n\
  Once again, we await the result of calling {7}, not the individual futures.\n\
  If we awaited the individual futures in sequence, we would just end up back in a sequential flow—exactly what we're trying not \
  to do.\n\n\
  See Listing 17-11:{8}, for code sample\n\n\
  With the updated code in Listing 17-11, the messages get printed at 500-millisecond intervals, rather than all in a rush \
  after 2 seconds.
  ",
  "await".bright_yellow().bold(),
  "tx.send".bright_yellow().bold(),
  "trpl::sleep".bright_yellow().bold(),
  "while let".bright_yellow().bold(),
  "recv".bright_yellow().bold(),
  "tx".bright_yellow().bold(),
  "rx".bright_yellow().bold(),
  "trpl::join".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-11".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} Within a given async block, the order in which {1} keywords appear in the code is also the order in which \
  they're executed when the program runs.
  ",
  "REMEMBER".bright_white().bold(),
  "await".bright_yellow().bold()
  )  


}

// Header: Moving Ownership Into an Async Block. Abbreviated as moiaab.
fn moiaab_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Moving Ownership Into an Async Block");

  println!(
  "See: {0}, for complete reading
  ",
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#moving-ownership-into-an-async-block".bright_cyan()
  )
}


// Header: Joining a Number of Futures with the join! Macro. Abbreviated as janofwtjm.
fn janofwtjm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Joining a Number of Futures with the join! Macro");

  println!(
  "This async channel is also a multiple-producer channel, so we can call {0} on {1} if we want to send messages from \
  multiple futures, as shown in Listing 17-13.\n\n\
  See Listing 17-13: {2}, for code sample and output.\n\n\
  First, we clone {1}, creating {3} outside the first async block.\n\
  We move {3} into that block just as we did before with {1}.\n\
  Then, later, we move the original {1} into a new async block, where we send more messages on a slightly slower delay.\n\
  We happen to put this new async block after the async block for receiving messages, but it could go before it just as well.\n\
  The key is the order in which the futures are awaited, not in which they're created.
  ",
  "clone".bright_yellow().bold(),
  "tx".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#listing-17-13".bright_cyan(),
  "tx1".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Both of the async blocks for sending messages need to be {1} blocks so that both {2} and {3} get dropped when those \
  blocks finish.\n\
  Otherwise, we'll end up back in the same infinite loop we started out in.
  ",
  "async move".bright_magenta().bold(),
  "async move".bright_yellow().bold(),
  "tx".bright_yellow().bold(),
  "tx1".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  Finally, we switch from {1} to {2} to handle the additional future: the {2} macro awaits an arbitrary number of futures \
  where we know the number of futures at compile time.\n\
  We'll discuss awaiting a collection of an unknown number of futures later in this chapter.\n\n\
  Now we see all the messages from both sending futures, and because the sending futures use slightly different delays after \
  sending, the messages are also received at those different intervals.\n\n\
  What we've explored:\n\
  {solid_disc} how to use message passing to send data between futures,\n\
  {solid_disc} how code within an async block runs sequentially,\n\
  {solid_disc} how to move ownership into an async block,\n\
  {solid_disc} and how to join multiple futures\n\n\
  Next, let's discuss how and why to tell the runtime it can switch to another task.
  ",
  "The join! macro".bright_magenta().bold(),
  "trpl::join".bright_yellow().bold(),
  "trpl::join!".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Async runtime executes futures in the order in which they're awaited, not in which they're created.
  ",
  "REMEMBER".bright_white().bold(),
  )
}








