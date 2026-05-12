use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 7];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Controlling Access with Mutexes", cawm_content),
    chapter::SubHeader::new("The API of Mutex<T>", taom_content),
    chapter::SubHeader::new("Shared Access to Mutex<T>", satm_content),
    chapter::SubHeader::new("Multiple Ownership with Multiple Threads", mowmt_content),
    chapter::SubHeader::new("Atomic Reference Counting with Arc<T>", arcwa_content),
    chapter::SubHeader::new("Comparing RefCell<T>/Rc<T> and Mutex<T>/Arc<T>", cram_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Shared-State Concurrency");

  println!(
  "Message passing is a fine way to handle concurrency, but it's not the only way.\n\
  Another method would be for multiple threads to access the same shared data.\n\
  Consider this part of the slogan from the Go language documentation again: “Do not communicate by sharing memory.”\n\n\
  What would communicating by sharing memory look like? In addition, why would message-passing enthusiasts caution not \
  to use memory sharing?\n\n\
  In a way, channels in any programming language are similar to single ownership because once you transfer a value down \
  a channel, you should no longer use that value.\n\
  Shared-memory concurrency is like multiple ownership: Multiple threads can access the same memory location at the same time.\n\
  As you saw in Chapter 15, where smart pointers made multiple ownership possible, multiple ownership can add complexity \
  because these different owners need managing.\n\
  Rust's type system and ownership rules greatly assist in getting this management correct.\n\
  For an example, let's look at mutexes, one of the more common concurrency primitives for shared memory.
  ");

  println!(
  "{0}\n\n\
  {solid_disc} In a way, channels in any programming language are similar to single ownership because once you transfer a \
  value down a channel, you should no longer use that value.\n\
  {solid_disc} Shared-memory concurrency is like multiple ownership: Multiple threads can access the same memory location \
  at the same time.
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Controlling Access with Mutexes. Abbreviated as cawm.
fn cawm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Controlling Access with Mutexes");

  println!(
  "{0} is an abbreviation for {1}, as in a mutex allows only one thread to access some data at any given time.\n\
  To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the mutex's lock.\n\
  The {2} is a data structure that is part of the mutex that keeps track of who currently has exclusive access to the data.\n\
  Therefore, the mutex is described as {3} the data it holds via the locking system.\n\n\
  Mutexes have a reputation for being difficult to use because you have to remember two rules:\n\n\
  1. You must attempt to acquire the lock before using the data.\n\
  2. When you're done with the data that the mutex guards, you must unlock the data so that other threads can acquire the lock.\n\n\
  For a real-world metaphor for a mutex, imagine a panel discussion at a conference with only one microphone.\n\
  Before a panelist can speak, they have to ask or signal that they want to use the microphone.\n\
  When they get the microphone, they can talk for as long as they want to and then hand the microphone to the next panelist \
  who requests to speak.\n\
  If a panelist forgets to hand the microphone off when they're finished with it, no one else is able to speak.\n\
  If management of the shared microphone goes wrong, the panel won't work as planned!\n\n\
  Management of mutexes can be incredibly tricky to get right, which is why so many people are enthusiastic about channels.\n\
  However, thanks to Rust's type system and ownership rules, you can't get locking and unlocking wrong.
  ",
  "Mutex".italic().bold(),
  "mutual exclusion".italic().bold(),
  "lock".italic().bold(),
  "guarding".italic().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} {1} is an abbreviation for {2}, as in a mutex allows only one thread to access some data at any given time.\n\
  {solid_disc} To access the data in a mutex, a thread must first signal that it wants access by asking to acquire the \
  mutex's lock.\n\
  {solid_disc} The {3} is a data structure that is part of the mutex that keeps track of who currently has exclusive access \
  to the data.\n\
  {solid_disc} The mutex is described as {4} the data it holds via the locking system.
  ",
  "REMEMBER".bright_white().bold(),
  "Mutex".italic().bold(),
  "mutual exclusion".italic().bold(),
  "lock".italic().bold(),
  "guarding".italic().bold(),
  )
}

// Header: The API of Mutex<T>. Abbreviated as taom.
// Extracted from the Controlling Access with Mutexes header.
fn taom_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The API of Mutex<T>");

  println!(
  "As an example of how to use a mutex, let's start by using a mutex in a single-threaded context, as shown in Listing 16-12.\n\n\
  See Listing 16-12: {0}, for code sample.\n\n\
  As with many types, we create a {1} using the associated function {2}.\n\
  To access the data inside the mutex, we use the {3} method to acquire the lock.\n\
  This call will block the current thread so that it can't do any work until it's our turn to have the lock.\n\n\
  The call to {3} would fail if another thread holding the lock panicked.\n\
  In that case, no one would ever be able to get the lock, so we've chosen to {4} and have this thread panic if we're in \
  that situation.\n\n\
  After we've acquired the lock, we can treat the return value, named {5} in this case, as a mutable reference to the data inside.\n\
  The type system ensures that we acquire a lock before using the value in {6}.\n\
  The type of {6} is {7}, not {8}, so we must call lock to be able to use the {8} value.\n\
  We can't forget; the type system won't let us access the inner {8} otherwise.\n\n\
  The call to {3} returns a type called {9}, wrapped in a {10} that we handled with the call to {4}.\n\
  The {9} type implements {11} to point at our inner data; the type also has a {12} implementation that releases the lock \
  automatically when a {9} goes out of scope, which happens at the end of the inner scope.\n\
  As a result, we don't risk forgetting to release the lock and blocking the mutex from being used by other threads because \
  the lock release happens automatically.\n\n\
  After dropping the lock, we can print the mutex value and see that we were able to change the inner {8} to 6.
  ",
  "https://doc.rust-lang.org/book/ch16-03-shared-state.html#listing-16-12".bright_cyan(),
  "Mutex<T>".bright_yellow().bold(),
  "new".bright_yellow().bold(),
  "lock".bright_yellow().bold(),
  "unwrap".bright_yellow().bold(),
  "num".bright_yellow().bold(),
  "m".bright_yellow().bold(),
  "Mutex<i32>".bright_yellow().bold(),
  "i32".bright_yellow().bold(),
  "MutexGuard".bright_yellow().bold(),
  "LockResult".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  {solid_disc} To access the data inside the mutex, we use the {1} method to acquire the lock. \
  This call will block the current thread. The type system ensures that we acquire a lock before using the value.\n\
  {solid_disc} The call to {1} would fail if another thread holding the lock panicked.\n\
  {solid_disc} The call to {1} returns a type called {2}, wrapped in a {3} that we handled with the call to {4}.\n\
  {solid_disc} The {2} type implements the {5} to point at our inner data; the type also has a {6} implementation that \
  releases the lock automatically when a {2} goes out of scope.\n\
  ",
  "REMEMBER".bright_white().bold(),
  "lock".bright_yellow().bold(),
  "MutexGuard".bright_yellow().bold(),
  "LockResult".bright_yellow().bold(),
  "unwrap".bright_yellow().bold(),
  "Deref".bright_yellow().bold(),
  "Drop".bright_yellow().bold(),
  )  
}

// Header: Shared Access to Mutex<T>. Abbreviated as satm.
// Extracted from the Controlling Access with Mutexes header.
fn satm_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Shared Access to Mutex<T>");

  println!(
  "Now let's try to share a value between multiple threads using {0}.\n\
  We'll spin up 10 threads and have them each increment a counter value by 1, so the counter goes from 0 to 10.\n\
  The example in Listing 16-13 will have a compiler error, and we'll use that error to learn more about using {0} and how Rust \
  helps us use it correctly.\n\n\
  See Listing 16-13: {1}, for code sample.\n\n\
  ",
  "Mutex<T>".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-03-shared-state.html#listing-16-13".bright_cyan()
  )
}

// Header: Multiple Ownership with Multiple Threads. Abbreviated as mowmt.
// Extracted from the Controlling Access with Mutexes header.
fn mowmt_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Multiple Ownership with Multiple Threads");

  println!(
  "In Chapter 15, we gave a value to multiple owners by using the smart pointer {0} to create a reference-counted value.\n\
  Let's do the same here and see what happens.\n\
  We'll wrap the {1} in {0} in Listing 16-14 and clone the {0} before moving ownership to the thread.\n\n\
  See Listing 16-14: {2}, for code sample, output and complete reading.\n\n\
  Unfortunately, {0} is not safe to share across threads.\n\
  When {0} manages the reference count, it adds to the count for each call to {3} and subtracts from the count when each clone \
  is dropped.\n\
  But it doesn't use any concurrency primitives to make sure that changes to the count can't be interrupted by another thread.\n\
  This could lead to wrong counts—subtle bugs that could in turn lead to memory leaks or a value being dropped before we're done \
  with it.\n\
  What we need is a type that is exactly like {0}, but that makes changes to the reference count in a thread-safe way.
  ",
  "Rc<T>".bright_yellow().bold(),
  "Mutex<T>".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-03-shared-state.html#listing-16-14".bright_cyan(),
  "clone".bright_yellow().bold(),
  )
}

// Header: Atomic Reference Counting with Arc<T>. Abbreviated as arcwa.
// Extracted from the Controlling Access with Mutexes header.
fn arcwa_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Atomic Reference Counting with Arc<T>");

  println!(
  "Fortunately, {0} is a type like {1} that is safe to use in concurrent situations.\n\
  The a stands for atomic, meaning it's an atomically reference-counted type.\n\
  Atomics are an additional kind of concurrency primitive that we won't cover in detail here: See the standard library \
  documentation for {2}: {3}, for more details.\n\
  At this point, you just need to know that atomics work like primitive types but are safe to share across threads.\n\n\
  You might then wonder why all primitive types aren't atomic and why standard library types aren't implemented to use \
  {0} by default.\n\n\
  {4}\n\n\
  The reason is that thread safety comes with a performance penalty that you only want to pay when you really need to.\n\
  If you're just performing operations on values within a single thread, your code can run faster if it doesn't have to enforce \
  the guarantees atomics provide.\n\n\
  Let's return to our example: {0} and {1} have the same API, so we fix our program by changing the use line, the call to new, \
  and the call to clone.\n\
  The code in Listing 16-15 will finally compile and run.\n\n\
  See Listing 16-15: {5}, for code sample, output and complete reading.\n\n\
  Note that if you are doing simple numerical operations, there are types simpler than {6} types provided by the {7} module \
  of the standard library.\n\
  These types provide safe, concurrent, atomic access to primitive types.\n\
  We chose to use {6} with a primitive type for this example so that we could concentrate on how {6} works.
  ",
  "Arc<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "std::sync::atomic".bright_cyan(),
  "https://doc.rust-lang.org/std/sync/atomic/index.html".bright_cyan(),
  "THREAD SAFETY COST".bright_magenta().bold(),
  "https://doc.rust-lang.org/book/ch16-03-shared-state.html#listing-16-15".bright_cyan(),
  "Mutex<T>".bright_yellow().bold(),
  "std::sync::atomic".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Atomic types provide primitive shared-memory communication between threads, and are the building blocks of \
  other concurrent types.\n\
  {solid_disc} There area atomic versions of a select number of primitive types, including  AtomicBool, AtomicIsize, AtomicUsize, \
  AtomicI8, AtomicU16, etc.\n\
  {solid_disc} Primitive types aren't atomic by default because thread safety comes with a performance penalty.
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Comparing RefCell<T>/Rc<T> and Mutex<T>/Arc<T>. Abbreviated as cram.
fn cram_content() {
  menu::subheader_title("Comparing RefCell<T>/Rc<T> and Mutex<T>/Arc<T>");

  println!(
  "You might have noticed that {0} is immutable but that we could get a mutable reference to the value inside it; this means \
  {1} provides interior mutability, as the {2} family does.\n\
  In the same way we used {3} in Chapter 15 to allow us to mutate contents inside an {4}, we use {1} to mutate contents inside an \
  {5}.\n\n\
  Another detail to note is that Rust can't protect you from all kinds of logic errors when you use {6}.\n\
  Recall from Chapter 15 that using {4} came with the risk of creating reference cycles, where two {4} values refer to each other, \
  causing memory leaks.\n\
  Similarly, {6} comes with the risk of creating {7}.\n\
  These occur when an operation needs to lock two resources and two threads have each acquired one of the locks, causing them to \
  wait for each other forever.\n\
  If you're interested in deadlocks, try creating a Rust program that has a deadlock; then, research deadlock mitigation strategies \
  for mutexes in any language and have a go at implementing them in Rust.\n\
  The standard library API documentation for {6} and {8} offers useful information.
  ",
  "counter".bright_yellow().bold(),
  "Mutex<T>".bright_yellow().bold(),
  "Cell".bright_yellow().bold(),
  "RefCell<T>".bright_yellow().bold(),
  "Rc<T>".bright_yellow().bold(),
  "Arc<T>".bright_yellow().bold(),
  "Mutex<T>".bright_yellow().bold(),
  "deadlocks".italic().bold(),
  "MutexGuard".bright_yellow().bold()
  )
}





