use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 4];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Transferring Ownership Through Channels", totc_content),
    chapter::SubHeader::new("Sending Multiple Values", smv_content),
    chapter::SubHeader::new("Creating Multiple Producers", cmp_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Transfer Data Between Threads with Message Passing");

  println!(
  "One increasingly popular approach to ensuring safe concurrency is message passing, where threads or actors communicate by \
  sending each other messages containing data.\n\
  Here's the idea in a slogan from the Go language documentation: “Do not communicate by sharing memory; instead, share \
  memory by communicating.”\n\n\
  See Go language documentation: {0}\n\n\
  To accomplish message-sending concurrency, Rust's standard library provides an implementation of channels.\n\
  A {1} is a general programming concept by which data is sent from one thread to another.\n\n\
  You can imagine a channel in programming as being like a directional channel of water, such as a stream or a river.\n\
  If you put something like a rubber duck into a river, it will travel downstream to the end of the waterway.\n\n\
  A channel has two halves:\n\
  {solid_disc} a transmitter\n\
  {solid_disc} and a receiver.\n\n\
  The transmitter half is the upstream location where you put the rubber duck into the river, and the receiver half is \
  where the rubber duck ends up downstream.\n\
  One part of your code calls methods on the transmitter with the data you want to send, and another part checks the \
  receiving end for arriving messages.n\n\
  A channel is said to be closed if either the transmitter or receiver half is dropped.\n\n\
  Here, we'll work up to a program that has one thread to generate values and send them down a channel, and another thread \
  that will receive the values and print them out.\n\
  We'll be sending simple values between threads using a channel to illustrate the feature.\n\
  Once you're familiar with the technique, you could use channels for any threads that need to communicate with each other, \
  such as a chat system or a system where many threads perform parts of a calculation and send the parts to one thread that \
  aggregates the results.\n\n\
  First, in Listing 16-6, we'll create a channel but not do anything with it.\n\
  Note that this won't compile yet because Rust can't tell what type of values we want to send over the channel.\n\n\
  See Listing 16-6: {2}, for code sample and complete reading.
  ",
  "https://go.dev/doc/effective_go#concurrency".bright_cyan(),
  "channel".italic().bold(),
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#listing-16-6".bright_cyan()
  );

  println!(
  "We create a new channel using the {0} function; mpsc stands for {1}.\n\
  In short, the way Rust's standard library implements channels means a channel can have multiple sending ends that produce values \
  but only one receiving end that consumes those values.\n\
  Imagine multiple streams flowing together into one big river: Everything sent down any of the streams will end up in one river \
  at the end.\n\
  We'll start with a single producer for now, but we'll add multiple producers when we get this example working.\n\n\
  The {0} function returns a tuple, the first element of which is the sending end—the transmitter—and the second element of which \
  is the receiving end—the receiver.\n\
  The abbreviations {2} and {3} are traditionally used in many fields for transmitter and receiver, respectively, so we name our \
  variables as such to indicate each end.\n\
  We're using a {4} statement with a pattern that destructures the tuples; we'll discuss the use of patterns in {4} statements \
  and destructuring in Chapter 19.\n\
  For now, know that using a {4} statement in this way is a convenient approach to extract the pieces of the tuple returned \
  by {0}.
  ",
  "mpsc::channel".bright_yellow().bold(),
  "multiple producer, single consumer".italic().bold(),
  "tx".bright_yellow().bold(),
  "rx".bright_yellow().bold(),
  "let".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  Again, we're using {1} to create a new thread and then using {2} to move {3} into the closure so that the spawned thread owns {3}.\n\
  The spawned thread needs to own the transmitter to be able to send messages through the channel.\n\n\
  The transmitter has a {4} method that takes the value we want to send.\n\
  The {4} method returns a {5} type, so if the receiver has already been dropped and there's nowhere to send a value, the send \
  operation will return an error.\n\
  In this example, we're calling {6} to panic in case of an error.\n\
  But in a real application, we would handle it properly: Return to Chapter 9 to review strategies for proper error handling.\n\n\
  See Listing 16-8: {7}, for code sample and complete reading.
  ",
  "THE TX SEND METHOD".bright_magenta().bold(),
  "thread::spawn".bright_yellow().bold(),
  "move".bright_yellow().bold(),
  "tx".bright_yellow().bold(),
  "send".bright_yellow().bold(),
  "Result<T, E>".bright_yellow().bold(),
  "unwrap".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#listing-16-8".bright_cyan()
  );

  println!(
  "{0}\n\n\
  The receiver has two useful methods: {1} and {2}.\n\
  We're using {1}, short for receive, which will block the main thread's execution and wait until a value is sent down the channel.\n\
  Once a value is sent, {1} will return it in a {3}.\n\
  When the transmitter closes, {1} will return an error to signal that no more values will be coming.\n\n\
  The {2} method doesn't block, but will instead return a {3} immediately: an {4} value holding a message if one is available \
  and an {5} value if there aren't any messages this time.\n\
  Using {2} is useful if this thread has other work to do while waiting for messages: We could write a loop that calls {2} every so \
  often, handles a message if one is available, and otherwise does other work for a little while until checking again.\n\n\
  We've used {1} in this example for simplicity; we don't have any other work for the main thread to do other than wait for \
  messages, so blocking the main thread is appropriate.
  ",
  "THE RX RECV AND TRY_RECV METHODS".bright_magenta().bold(),
  "recv".bright_yellow().bold(),
  "try_recv".bright_yellow().bold(),
  "Result<T, E>".bright_yellow().bold(),
  "Ok".bright_yellow().bold(),
  "Err".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} A {1} is a general programming concept by which data is sent from one thread to another.\n\
  {solid_disc} A channel has two halves: a transmitter and a receiver.\n\
  {solid_disc} A channel is said to be closed if either the transmitter or receiver half is dropped.\n\
  {solid_disc} We create a new channel using the {2} function; {3} stands for {4}.\n\
  {solid_disc} The {2} function returns a tuple, the first element of which is the sending end—the transmitter—and the second element of which \
  is the receiving end—the receiver.
  ",
  "REMEMBER".bright_white().bold(),
  "channel".italic().bold(),
  "mpsc::channel".bright_yellow().bold(),
  "mpsc".bright_yellow().bold(),
  "multiple producer, single consumer".italic().bold(),
  )
}

// Header: Transferring Ownership Through Channels. Abbreviated as totc.
fn totc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Transferring Ownership Through Channels");

  println!(
  "The ownership rules play a vital role in message sending because they help you write safe, concurrent code.\n\
  Preventing errors in concurrent programming is the advantage of thinking about ownership throughout your Rust programs.\n\
  Let's do an experiment to show how channels and ownership work together to prevent problems: We'll try to use a val value in \
  the spawned thread after we've sent it down the channel.\n\
  Try compiling the code in Listing 16-9 to see why this code isn't allowed.\n\n\
  See Listing 16-9: {0}, for code sample and output.\n\n\
  Here, we try to print val after we've sent it down the channel via {1}.\n\
  Allowing this would be a bad idea: Once the value has been sent to another thread, that thread could modify or drop it before we \
  try to use the value again.\n\
  Potentially, the other thread's modifications could cause errors or unexpected results due to inconsistent or nonexistent data.\n\
  However, Rust gives us an error if we try to compile the code in Listing 16-9.\n\n\
  Our concurrency mistake has caused a compile-time error.\n\
  The {1} function takes ownership of its parameter, and when the value is moved the receiver takes ownership of it.\n\
  This stops us from accidentally using the value again after sending it; the ownership system checks that everything is okay.
  ",
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#listing-16-9".bright_cyan(),
  "tx.send".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} The {1} function takes ownership of its parameter, and when the value is moved the receiver takes ownership of it.
  ",
  "REMEMBER".bright_white().bold(),
  "tx.send".bright_yellow().bold()
  )
}

// Header: Sending Multiple Values. Abbreviated as smv.
fn smv_content() {
  menu::subheader_title("Sending Multiple Values");

  println!(
  "The code in Listing 16-8 compiled and ran, but it didn't clearly show us that two separate threads were talking to each other \
  over the channel.\n\n\
  In Listing 16-10, we've made some modifications that will prove the code in Listing 16-8 is running concurrently: \
  The spawned thread will now send multiple messages and pause for a second between each message.\n\n\
  See Listing 16-10: {0}, for code sample and output.\n\n\
  This time, the spawned thread has a vector of strings that we want to send to the main thread.\n\
  We iterate over them, sending each individually, and pause between each by calling the {1} function with a {2} value of \
  one second.\n\n\
  In the main thread, we're not calling the {3} function explicitly anymore: Instead, we're treating {4} as an iterator.\n\
  For each value received, we're printing it.\n\
  When the channel is closed, iteration will end.\n\n\
  Because we don't have any code that pauses or delays in the {5} loop in the main thread, we can tell that the main thread \
  is waiting to receive values from the spawned thread.
  ",
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#listing-16-10".bright_cyan(),
  "thread::sleep".bright_yellow().bold(),
  "Duration".bright_yellow().bold(),
  "recv".bright_yellow().bold(),
  "rx".bright_yellow().bold(),
  "for".bright_yellow().bold(),
  )
}

// Header: Creating Multiple Producers. Abbreviated as cmp.
fn cmp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Creating Multiple Producers");

  println!(
  "Earlier we mentioned that {0} was an acronym for {1}.\n\
  Let's put {0} to use and expand the code in Listing 16-10 to create multiple threads that all send values to the same receiver.\n\
  We can do so by cloning the transmitter, as shown in Listing 16-11.\n\n\
  See Listing 16-11: {2}, for code sample and output.\n\n\
  This time, before we create the first spawned thread, we call {3} on the transmitter.\n\
  This will give us a new transmitter we can pass to the first spawned thread.\n\
  We pass the original transmitter to a second spawned thread.\n\
  This gives us two threads, each sending different messages to the one receiver.\n\n\
  You might see the values in another order, depending on your system.\n\
  This is what makes concurrency interesting as well as difficult.\n\
  If you experiment with {4}, giving it various values in the different threads, each run will be more nondeterministic and \
  create different output each time.
  ",
  "mpsc".bright_yellow().bold(),
  "multiple producer, single consumer".italic().bold(),
  "https://doc.rust-lang.org/book/ch16-02-message-passing.html#listing-16-11".bright_cyan(),
  "clone".bright_yellow().bold(),
  "thread::sleep".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} To create multiple producers, we can clone the transmitter.\n\
  {solid_disc} The order in which threads are run is not guaranteed. This is what makes concurrency interesting as \
  well as difficult.\n\
  ",
  "REMEMBER".bright_white().bold(),
  );
}










