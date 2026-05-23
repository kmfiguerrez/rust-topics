use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 1];
  subheaders = [
    chapter::SubHeader::new("Streams: Futures in Sequence", sfis_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Streams: Futures in Sequence. Abbreviated as sfis.
fn sfis_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Streams: Futures in Sequence");

  println!(
  "Recall how we used the receiver for our async channel earlier in this chapter in the “Message Passing” section: {0}.\n\
  The async {1} method produces a sequence of items over time.\n\
  This is an instance of a much more general pattern known as a {2}.\n\n\
  {2}\n\n\
  Many concepts are naturally represented as streams: items becoming available in a queue, chunks of data being pulled \
  incrementally from the filesystem when the full data set is too large for the computer's memory, or data arriving over \
  the network over time.\n\n\
  {3}\n\n\
  Because streams are futures, we can use them with any other kind of future and combine them in interesting ways.\n\
  For example, we can batch up events to avoid triggering too many network calls, set timeouts on sequences of long-running \
  operations, or throttle user interface events to avoid doing needless work.
  ",
  "https://doc.rust-lang.org/book/ch17-02-concurrency-with-async.html#message-passing".bright_cyan(),
  "recv".bright_yellow().bold(),
  "Examples of streams".bright_magenta().bold(),
  "Streams are futures".bright_magenta().bold(),
  );

  println!(
  "{0}\n\n\
  We saw a sequence of items back in Chapter 13, when we looked at the {2} trait in \
  “The Iterator Trait and the next Method” section: {1}, but there are two differences between iterators and the async channel \
  receiver.\n\
  {solid_disc} The first difference is time: iterators are synchronous, while the channel receiver is asynchronous.\n\
  {solid_disc} The second difference is the API.\n\n\
  When working directly with {2}, we call its synchronous {5} method.\n\
  With the {3} stream in particular, we called an asynchronous {4} method instead.\n\
  Otherwise, these APIs feel very similar, and that similarity isn't a coincidence.\n\
  A stream is like an asynchronous form of iteration.\n\
  Whereas the {3} specifically waits to receive messages, though, the general-purpose stream API is much broader: it provides \
  the next item the way {2} does, but asynchronously.\n\n\
  The similarity between iterators and streams in Rust means we can actually create a stream from any iterator.\n\
  As with an iterator, we can work with a stream by calling its {5} method and then awaiting the output, as \
  in Listing 17-21, which won't compile yet.\n\n\
  See Listing 17-21: {6}, for code sample and complete reading.
  ",
  "Difference between iterators and async channel receiver".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch13-02-iterators.html#the-iterator-trait-and-the-next-method".bright_cyan(),
  "Iterator".bright_yellow().bold(),
  "trpl::Receiver".bright_yellow().bold(),
  "recv".bright_yellow().bold(),
  "next".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch17-04-streams.html#listing-17-21".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  The {1} trait defines a low-level interface that effectively combines the {2} and Future traits.\n\
  {3} supplies a higher-level set of APIs on top of {1}, including the {4} method as well as other utility methods similar to \
  those provided by the {2} trait.\n\
  {1} and {3} are not yet part of Rust's standard library, but most ecosystem crates use similar definitions.
  ",
  "The Stream and StreamExt traits".bright_magenta().bold(),
  "Stream".bright_yellow().bold(),
  "Iterator".bright_yellow().bold(),
  "StreamExt".bright_yellow().bold(),
  "next".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} A stream is like an asynchronous form of iteration.\n\
  {solid_disc} {1} short for extension, is a common pattern in the Rust community for extending one trait with another.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "Ext".bright_yellow().bold(),
  )
}