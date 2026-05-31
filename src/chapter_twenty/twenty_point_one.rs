use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 13];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Performing Unsafe Superpowers", pus_content),
    chapter::SubHeader::new("Dereferencing a Raw Pointer", darp_content),
    chapter::SubHeader::new("Calling an Unsafe Function or Method", caufom_content),
    chapter::SubHeader::new("Creating a Safe Abstraction over Unsafe Code", casaou_content),
    chapter::SubHeader::new("Using extern Functions to Call External Code", ueftcec_content),
    chapter::SubHeader::new("Calling Rust Functions from Other Languages", crffol_content),
    chapter::SubHeader::new("Accessing or Modifying a Mutable Static Variable", aomamsv_content),
    chapter::SubHeader::new("Implementing an Unsafe Trait", iaut_content),
    chapter::SubHeader::new("Accessing Fields of a Union", afoau_content),
    chapter::SubHeader::new("Using Miri to Check Unsafe Code", umtcuc_content),
    chapter::SubHeader::new("Using Unsafe Code Correctly", uucc_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Advanced Features");

  println!(
  "We'll look at a few aspects of the language you might run into every once in a while but may not use every day.\n\
  You can use this chapter as a reference for when you encounter any unknowns.\n\
  The features covered here are useful in very specific situations.\n\
  Although you might not reach for them often, we want to make sure you have a grasp of all the features Rust has to offer.\n\n\
  In this chapter, we'll cover:\n\n\
  {solid_disc} Unsafe Rust: How to opt out of some of Rust's guarantees and take responsibility for manually upholding those \
  guarantees\n\
  {solid_disc} Advanced traits: Associated types, default type parameters, fully qualified syntax, supertraits, and the newtype \
  pattern in relation to traits\n\
  {solid_disc} Advanced types: More about the newtype pattern, type aliases, the never type, and dynamically sized types\n\
  {solid_disc} Advanced functions and closures: Function pointers and returning closures\n\
  {solid_disc} Macros: Ways to define code that defines more code at compile time\n\
  ")
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Unsafe Rust");

  println!(
  "All the code we've discussed so far has had Rust's memory safety guarantees enforced at compile time.\n\
  However, Rust has a second language hidden inside it that doesn't enforce these memory safety guarantees: It's called unsafe \
  Rust and works just like regular Rust but gives us extra superpowers.\n\n\
  Unsafe Rust exists because, by nature, static analysis is conservative.\n\
  When the compiler tries to determine whether or not code upholds the guarantees, it's better for it to reject some valid \
  programs than to accept some invalid programs.\n\
  Although the code might be okay, if the Rust compiler doesn't have enough information to be confident, it will reject the code.\n\
  In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I'm doing.”\n\
  Be warned, however, that you use unsafe Rust at your own risk: If you use unsafe code incorrectly, problems can occur due to \
  memory unsafety, such as null pointer dereferencing.\n\n\
  Another reason Rust has an unsafe alter ego is that the underlying computer hardware is inherently unsafe.\n\
  If Rust didn't let you do unsafe operations, you couldn't do certain tasks.\n\
  Rust needs to allow you to do low-level systems programming, such as directly interacting with the operating system or even \
  writing your own operating system.\n\
  Working with low-level systems programming is one of the goals of the language.\n\
  Let's explore what we can do with unsafe Rust and how to do it.
  ");

  println!(
  "{0}\n\n\
  {solid_disc} if the Rust compiler doesn't have enough information to be confident, it will reject the code. \
  In these cases, you can use unsafe code to tell the compiler, “Trust me, I know what I'm doing.”
  ",
  "REMEMBER".bright_yellow().bold(),
  )
}

// Header: Performing Unsafe Superpowers. Abbreviated as pus.
fn pus_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Performing Unsafe Superpowers");

  println!(
  "To switch to unsafe Rust, use the {0} keyword and then start a new block that holds the unsafe code.\n\
  You can take five actions in unsafe Rust that you can't in safe Rust, which we call unsafe superpowers.\n\
  Those superpowers include the ability to:\n\n\
  1. Dereference a raw pointer.\n\
  2. Call an unsafe function or method.\n\
  3. Access or modify a mutable static variable.\n\
  4. Implement an unsafe trait.\n\
  5. Access fields of unions.\n\n\
  It's important to understand that {0} doesn't turn off the borrow checker or disable any of Rust's other safety checks: \
  If you use a reference in unsafe code, it will still be checked.\n\
  The {0} keyword only gives you access to these five features that are then not checked by the compiler for memory safety.\n\
  You'll still get some degree of safety inside an unsafe block.\n\
  In addition, {0} does not mean the code inside the block is necessarily dangerous or that it will definitely have memory safety \
  problems: The intent is that as the programmer, you'll ensure that the code inside an unsafe block will access memory in a \
  valid way.\n\n\
  People are fallible and mistakes will happen, but by requiring these five unsafe operations to be inside blocks annotated with \
  {0}, you'll know that any errors related to memory safety must be within an unsafe block.\n\
  Keep {0} blocks small; you'll be thankful later when you investigate memory bugs.\n\n\
  To isolate unsafe code as much as possible, it's best to enclose such code within a safe abstraction and provide a safe API, \
  which we'll discuss later in the chapter when we examine unsafe functions and methods.\n\
  Parts of the standard library are implemented as safe abstractions over unsafe code that has been audited.\n
  Wrapping unsafe code in a safe abstraction prevents uses of {0} from leaking out into all the places that you or your users \
  might want to use the functionality implemented with {0} code, because using a safe abstraction is safe.\n\n\
  Let's look at each of the five unsafe superpowers in turn.\n\
  We'll also look at some abstractions that provide a safe interface to unsafe code.
  ",
  "unsafe".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} To switch to unsafe Rust, use the {1} keyword and then start a new block that holds the unsafe code.\n\
  {solid_disc} It's important to understand that {1} doesn't turn off the borrow checker or disable any of Rust's other \
  safety checks.\n\
  {solid_disc} In addition, {1} does not mean the code inside the block is necessarily dangerous or that it will definitely \
  have memory safety problems.\n\
  {solid_disc} Keep {1} blocks small; you'll be thankful later when you investigate memory bugs.\n\
  {solid_disc} To isolate unsafe code as much as possible, it's best to enclose such code within a safe abstraction and provide a \
  safe API

  ",
  "REMEMBER".bright_white().bold(),
  "unsafe".bright_yellow().bold(),
  )
}

// Header: Dereferencing a Raw Pointer. Abbreviated as darp.
fn darp_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Dereferencing a Raw Pointer");

  println!(
  "In Chapter 4, in the “Dangling References” section:{0}, we mentioned that the compiler ensures that references are always valid.\n\
  Unsafe Rust has two new types called {1} that are similar to references.\n\
  As with references, raw pointers can be immutable or mutable and are written as {2} and {3}, respectively.\n\
  The asterisk isn't the dereference operator; it's part of the type name.\n\
  In the context of raw pointers, immutable means that the pointer can't be directly assigned to after being dereferenced.\n\n\
  Different from references and smart pointers, raw pointers:\n\n\
  {solid_disc} Are allowed to ignore the borrowing rules by having both immutable and mutable pointers or multiple mutable pointers \
  to the same location\n\
  {solid_disc} Aren't guaranteed to point to valid memory\n\
  {solid_disc} Are allowed to be null\n\
  {solid_disc} Don't implement any automatic cleanup\n\n\
  By opting out of having Rust enforce these guarantees, you can give up guaranteed safety in exchange for greater performance or \
  the ability to interface with another language or hardware where Rust's guarantees don't apply.\n\n\
  Listing 20-1 shows how to create an immutable and a mutable raw pointer.\n\n\
  See Listing 21-1:{4}, for code sample.\n\n\
  In Listing 20-1, notice that we don;t include the {5} keyword in this code. We can create raw pointers in safe code; we just \
  can't dereference raw pointers outside an unsafe block, as you'll see in a bit.\n\n\
  {6}\n\n\
  We've created raw pointers by using the raw borrow operators: {7} creates a {8} immutable raw pointer, \
  and {9} creates a {10} mutable raw pointer.\n\
  Because we created them directly from a local variable, we know these particular raw pointers are valid, but we can’t make \
  that assumption about just any raw pointer.
  ",
  "https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html#dangling-references".bright_cyan(),
  "raw pointers".italic().bold(),
  "*const T".bright_yellow().bold(),
  "*mut T".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-1".bright_cyan(),
  "unsafe".bright_yellow().bold(),
  "The raw borrow operators".bright_magenta().bold(),
  "&raw const num".bright_yellow().bold(),
  "*const i32".bright_yellow().bold(),
  "&raw mut num".bright_yellow().bold(),
  "*mut i32".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  Next we'll create a raw pointer whose validity we can't be so certain of, using the keyword {1} to cast a value instead of using \
  the raw borrow operator.\n\
  Listing 20-2 shows how to create a raw pointer to an arbitrary location in memory.\n\
  Trying to use arbitrary memory is undefined: There might be data at that address or there might not, the compiler might optimize \
  the code so that there is no memory access, or the program might terminate with a segmentation fault.\n\
  Usually, there is no good reason to write code like this, especially in cases where you can use a raw borrow operator instead, \
  but it is possible.\n\n\
  See Listing 20-2:{2}, for code sample.\n\n\
  Recall that we can create raw pointers in safe code, but we can't dereference raw pointers and read the data being pointed to.\n\
  In Listing 20-3, we use the dereference operator {3} on a raw pointer that requires an {4} block.\n\n\
  See Listing 20-3:{5}, for code sample.\n\n\
  Creating a pointer does no harm; it's only when we try to access the value that it points at that we might end up dealing with \
  an invalid value.
  ",
  "Arbitrary memory location".bright_magenta().bold(),
  "as".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-2".bright_cyan(),
  "*".bright_yellow().bold(),
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-3".bright_cyan()
  );

  println!(
  "{0}\n\n\
  Note also that in Listings 20-1 and 20-3, we created {1} and {2} raw pointers that both pointed to the same memory location, \
  where {3} is stored.\n\
  If we instead tried to create an immutable and a mutable reference to {3}, the code would not have compiled because Rust's \
  ownership rules don't allow a mutable reference at the same time as any immutable references.\n\
  With raw pointers, we can create a mutable pointer and an immutable pointer to the same location and change data through the \
  mutable pointer, potentially creating a data race. Be careful!\n\n\
  {4}\n\n\
  With all of these dangers, why would you ever use raw pointers?\n\
  One major use case is when interfacing with C code, as you'll see in the next section.\n\
  Another case is when building up safe abstractions that the borrow checker doesn't understand.\n\
  We'll introduce unsafe functions and then look at an example of a safe abstraction that uses unsafe code.
  ",
  "Ignoring borrowing rules".bright_magenta().bold(),
  "*const i32".bright_yellow().bold(),
  "*mut i32".bright_yellow().bold(),
  "num".bright_yellow().bold(),
  "Why use raw pointers".bright_magenta().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Unsafe Rust has two new types called {1} that are similar to references.\n\
  {solid_disc} As with references, raw pointers can be immutable or mutable and are written as {2} and {3}, respectively. \
  The asterisk isn't the dereference operator; it's part of the type name.\n\
  {solid_disc} In the context of raw pointers, {4} means that the pointer can't be directly assigned to after being \
  dereferenced.\n\
  {solid_disc} We can create raw pointers in safe code; we just can't dereference raw pointers outside an unsafe block.\n\
  {solid_disc} Creating a pointer does no harm; it's only when we try to access the value that it points at that we might \
  end up dealing with an invalid value.
  {solid_disc} Raw pointers ({2} and {3}) are fundamentally just memory addresses—under the hood, they are just \
  integers (like {5}).  
  ",
  "REMEMBER".bright_white().bold(),
  "raw pointers".italic().bold(),
  "*const T".bright_yellow().bold(),
  "*mut T".bright_yellow().bold(),
  "immutable".italic().bold(),
  "usize".bright_yellow().bold()
  )
}

// Header: Calling an Unsafe Function or Method. Abbreviated caufom.
fn caufom_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Calling an Unsafe Function or Method");

  println!(
  "The second type of operation you can perform in an unsafe block is calling unsafe functions.\n\
  Unsafe functions and methods look exactly like regular functions and methods, but they have an extra {0} before the rest of \
  the definition.\n\
  The {0} keyword in this context indicates the function has requirements we need to uphold when we call this function, \
  because Rust can't guarantee we've met these requirements.\n\
  By calling an unsafe function within an {0} block, we're saying that we've read this function's documentation and we take \
  responsibility for upholding the function's contracts.\n\n\
  See: {1}, for code sample and output.\n\n\
  To perform unsafe operations in the body of an {0} function, you still need to use an {0} block, just as within a \
  regular function, and the compiler will warn you if you forget.\n\
  This helps us keep unsafe blocks as small as possible, as unsafe operations may not be needed across the whole function body.
  ",
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#calling-an-unsafe-function-or-method".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Unsafe functions and methods are defined by using the {1} keyword before the function signature.\n\
  {solid_disc} We can only call unsafe functions within an {1} block, and we must ensure that the contracts of the unsafe \
  function are upheld when we call it.\n\
  {solid_disc} To perform unsafe operations in the body of an {1} function, you still need to use an {1} block, \
  just as within a regular function, and the compiler will warn you if you forget.
  {solid_disc} When we know code is okay, but Rust doesn't, it's time to reach for unsafe code.
  ",
  "REMEMBER".bright_white().bold(),
  "unsafe".bright_yellow().bold()
  )
}

// Header: Creating a Safe Abstraction over Unsafe Code. Abbreviated casaou.
fn casaou_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Creating a Safe Abstraction over Unsafe Code");

  println!(
  "Just because a function contains unsafe code doesn't mean we need to mark the entire function as unsafe.\n\
  In fact, wrapping unsafe code in a safe function is a common abstraction.\n\
  As an example, let's study the {0} function from the standard library, which requires some unsafe code.\n\
  We'll explore how we might implement it.\n\
  This safe method is defined on mutable slices: It takes one slice and makes it two by splitting the slice at the index given as \
  an argument.\n\n\
  Listing 20-4 shows how to use {0}.\n\n\
  See Listing 20-4:{1}, for code sample and complete reading.\n\n\
  In Listing 20-5:{2},\n\
  Rust's borrow checker can't understand that we're borrowing different parts of the slice; it only knows that we're borrowing \
  from the same slice twice.\n\
  Borrowing different parts of a slice is fundamentally okay because the two slices aren't overlapping, but Rust isn't smart \
  enough to know this.\n\
  When we know code is okay, but Rust doesn't, it's time to reach for unsafe code.\n\n\
  Listing 20-6 shows how to use an {3} block, a raw pointer, and some calls to unsafe functions to make the implementation of \
  {0} work.\n\n\
  See Listing 20-6:{4}, for code sample and complete reading.\n\n\
  ",
  "split_at_mut".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-4".bright_cyan(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-5".bright_cyan(),
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-6".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Just because a function contains unsafe code doesn't mean we need to mark the entire function as unsafe. \
  In fact, wrapping unsafe code in a safe function is a common abstraction.\n\
  {solid_disc} When we know code is okay, but Rust doesn't, it's time to reach for unsafe code.\n\
  {solid_disc} Functions and methods that take raw pointers as arguments or return raw pointers are unsafe because the \
  caller must ensure that the raw pointer is valid.\n\
  {solid_disc} Again unsafe code must be used within an unsafe block, even if it's within an unsafe function, to keep \
  unsafe code contained and make it easier to review and audit.
  {solid_disc} Raw pointers ({1} and {2}) are fundamentally just memory addresses—under the hood, they are just \
  integers (like {3}).
  ",
  "REMEMBER".bright_white().bold(),
  "*const T".bright_yellow().bold(),
  "*mut T".bright_yellow().bold(),
  "usize".bright_yellow().bold(),
)
}

// Header: Using extern Functions to Call External Code. Abbreviated ueftcec.
fn ueftcec_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using extern Functions to Call External Code");

  println!(
  "Sometimes your Rust code might need to interact with code written in another language.\n\
  For this, Rust has the keyword extern that facilitates the creation and use of a Foreign Function Interface (FFI), \
  which is a way for a programming language to define functions and enable a different (foreign) programming language to \
  call those functions.\n\n\
  {0}\n\n\
  Listing 20-8 demonstrates how to set up an integration with the {1} function from the C standard library.\n\
  Functions declared within {2} blocks are generally unsafe to call from Rust code, so {2} blocks must also be marked {3}.\n\
  The reason is that other languages don't enforce Rust's rules and guarantees, and Rust can't check them, so responsibility \
  falls on the programmer to ensure safety.\n\n\
  See Listing 20-8:{4}, for code sample and complete reading.
  ",
  "Codes from other languages are unsafe to Rust".bright_magenta().bold(),
  "abs".bright_yellow().bold(),
  "extern".bright_yellow().bold(),
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-8".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  Within the {1} block, we list the names and signatures of external functions from another language we want to call.\n\
  The {2} part defines which {3} the external function uses: The ABI defines how to call the function at the assembly level.\n\
  The {2} ABI is the most common and follows the C programming language's ABI.\n\
  Information about all the ABIs Rust supports is available in the Rust Reference: {4}.
  ",
  "Application Binary Interface (ABI)".bright_magenta().bold(),
  "unsafe extern \"C\"".bright_yellow().bold(),
  "\"C\"".bright_yellow().bold(),
  "application binary interface (ABI)".italic().bold(),
  "https://doc.rust-lang.org/reference/items/external-blocks.html#abi".bright_cyan()
  );

  println!(
  "{0}\n\n\
  Every item declared within an {1} block is implicitly unsafe.\n\
  However, some FFI functions are safe to call.\n\
  For example, the {2} function from C's standard library does not have any memory safety considerations, and we know it can be \
  called with any i32.\n\
  In cases like this, we can use the {3} keyword to say that this specific function is safe to call even though it is in an \
  {1} block.\n\
  Once we make that change, calling it no longer requires an unsafe block, as shown in Listing 20-9.\n\n\
  See Listing 20-9:{4}, for code sample.\n\n\
  Marking a function as {3} does not inherently make it safe! Instead, it is like a promise you are making to Rust that \
  it is safe.\n\
  It is still your responsibility to make sure that promise is kept!
  ",
  "The safe keyword".bright_magenta().bold(),
  "unsafe extern".bright_yellow().bold(),
  "abs".bright_yellow().bold(),
  "safe".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-9".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} Rust has the keyword {2} that facilitates the creation and use of a {1}.\n\
  {solid_disc} {1} is a way for a programming language to define functions and enable a \
  different (foreign) programming language to call those functions.\n\
  {solid_disc} Functions declared within {2} blocks are generally unsafe to call from Rust code, so {2} blocks must also be \
  marked {3}.\n\
  {solid_disc} Some FFI functions are safe to call. We can use them in safe Rust by marking them with the {4} keyword to say \
  that this specific function is safe to call even though it is in an {2} block.\n\
  {solid_disc} The ABI defines how to call the function at the assembly level.
  ",
  "REMEMBER".bright_white().bold(),
  "Foreign Function Interface (FFI)".italic().bold(),
  "extern".bright_yellow().bold(),
  "unsafe".bright_yellow().bold(),
  "safe".bright_yellow().bold()
  )
}

// Header: Calling Rust Functions from Other Languages. Abbreviated crffol.
fn crffol_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Calling Rust Functions from Other Languages");

  println!(
  "We can also use {0} to create an interface that allows other languages to call Rust functions.\n\
  Instead of creating a whole {0} block, we add the {0} keyword and specify the ABI to use just before the {1} keyword for the \
  relevant function.\n\
  We also need to add an {2} annotation to tell the Rust compiler not to mangle the name of this function.\n\
  Mangling is when a compiler changes the name we've given a function to a different name that contains more information for other \
  parts of the compilation process to consume but is less human readable.\n\
  Every programming language compiler mangles names slightly differently, so for a Rust function to be nameable by other languages, \
  we must disable the Rust compiler's name mangling.\n\
  This is unsafe because there might be name collisions across libraries without the built-in mangling, so it is our \
  responsibility to make sure the name we choose is safe to export without mangling.\n\n\
  In the following example, we make the {3} function accessible from C code, after it's compiled to a shared library and linked \
  from C:\n\n\
  See: {4}, for code sample.\n\n\
  This usage of {0} requires {5} only in the attribute, not on the {0} block.
  ",
  "extern".bright_yellow().bold(),
  "fn".bright_yellow().bold(),
  "#[unsafe(no_mangle)]".bright_yellow().bold(),
  "call_from_c".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#calling-rust-functions-from-other-languages".bright_cyan(),
  "unsafe".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} We can also use {1} to create an interface that allows other languages to call Rust functions.\n\
  {solid_disc} Instead of creating a whole {1} block, we add the {1} keyword and specify the ABI to use just before the {1} \
  keyword for the relevant function.\n\
  {solid_disc} {2} is when a compiler changes the name we've given a function to a different name that contains more information \
  for other parts of the compilation process to consume but is less human readable.\n\
  {solid_disc} {2} is unsafe because there might be name collisions across libraries without the built-in mangling, so it is our \
  responsibility to make sure the name we choose is safe to export without mangling.
  ",
  "REMEMBER".bright_white().bold(),
  "extern".bright_yellow().bold(),
  "Mangling".bright_yellow().bold(),
  )
}

// Header: Accessing or Modifying a Mutable Static Variable. Abbreviated aomamsv.
fn aomamsv_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Accessing or Modifying a Mutable Static Variable");

  println!(
  "In this book, we've not yet talked about global variables, which Rust does support but which can be problematic with Rust's \
  ownership rules.\n\
  If two threads are accessing the same mutable global variable, it can cause a data race.\n\n\
  In Rust, global variables are called static variables.\n\
  Listing 20-10 shows an example declaration and use of a static variable with a string slice as a value.\n\n\
  See Listing 20-10:{0}, for code sample and complete reading.\n\n\
  Static variables are similar to constants, which we discussed in the “Declaring Constants”: {1} section in Chapter 3.\n\
  The names of static variables are in {2} by convention.\n\
  Static variables can only store references with the {3} lifetime, which means the Rust compiler can figure out the lifetime \
  and we aren't required to annotate it explicitly.\n\
  Accessing an immutable static variable is safe.
  ",
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-10".bright_cyan(),
  "https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html#declaring-constants".bright_cyan(),
  "SCREAMING_SNAKE_CASE".bright_yellow().bold(),
  "'static".bright_yellow().bold()
  );

  println!(
  "{0}\n\n\
  A subtle difference between constants and immutable static variables is that values in a static variable have a fixed address \
  in memory.\n\
  Using the value will always access the same data. Constants, on the other hand, are allowed to duplicate their data whenever \
  they're used.\n\
  Another difference is that static variables can be mutable.\n\
  Accessing and modifying mutable static variables is unsafe.\n\
  Listing 20-11 shows how to declare, access, and modify a mutable static variable named {1}.\n\n\
  See Listing 20-11:{2}, for code sample and complete reading.
  ",
  "Difference between constants and immutable static variables".bright_magenta().bold(),
  "COUNTER".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-11".bright_cyan(),
  );

  println!(
  "{0}\n\n\
  Whenever we write an unsafe function, it is idiomatic to write a comment starting with {1} and explaining what the caller \
  needs to do to call the function safely.\n\
  Likewise, whenever we perform an unsafe operation, it is idiomatic to write a comment starting with {1} to explain how the \
  safety rules are upheld.  
  ",
  "Comment unsafe operations".bright_magenta().bold(),
  "SAFETY".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  With mutable data that is globally accessible, it's difficult to ensure that there are no data races, which is why Rust \
  considers mutable static variables to be unsafe.\n\
  Where possible, it's preferable to use the concurrency techniques and thread-safe smart pointers we discussed in Chapter 16 so \
  that the compiler checks that data access from different threads is done safely.
  ",
  "Workaround over static variables".bright_magenta().bold(),
  );  

  println!(
  "{0}\n\n\
  {solid_disc} In Rust, global variables are called static variables.\n\
  {solid_disc} If two threads are accessing the same mutable global variable, it can cause a data race.\n\
  {solid_disc} Static variables can only store references with the {1} lifetime, which means the Rust compiler can figure out \
  the lifetime and we aren't required to annotate it explicitly.\n\
  {solid_disc} Accessing an immutable static variable is safe.\n\
  {solid_disc} Accessing and modifying mutable static variables is unsafe.\n\
  {solid_disc} Any code that reads or writes from static variables must be within an {2} block.
  ",
  "REMEMBER".bright_white().bold(),
  "'static".bright_yellow().bold(),
  "unsafe".bright_yellow().bold()
  )
}

// Header: Implementing an Unsafe Trait. Abbreviated iaut.
fn iaut_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing an Unsafe Trait");

  println!(
  "We can use {0} to implement an unsafe trait.\n\
  A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify.\n\
  We declare that a trait is {0} by adding the {0} keyword before {1} and marking the implementation of the trait as \
  {0} too, as shown in Listing 20-12.\n\n\
  See Listing 20-12:{2}, for code sample and complete reading.\n\n\
  By using {6}, we're promising that we'll uphold the invariants that the compiler can't verify.\n\n\
  As an example, recall the {3} and {4} marker traits we discussed in the “Extensible Concurrency with Send and Sync” section in \
  Chapter 16: {5}, The compiler implements these traits automatically if our types are composed entirely of other types that \
  implement {3} and {4}.\n\
  If we implement a type that contains a type that does not implement {3} or {4}, such as raw pointers, and we want to mark \
  that type as {3} or {4}, we must use {0}.\n\
  Rust can't verify that our type upholds the guarantees that it can be safely sent across threads or accessed from multiple \
  threads; therefore, we need to do those checks manually and indicate as such with {0}.
  ",
  "unsafe".bright_yellow().bold(),
  "trait".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#listing-20-12".bright_cyan(),
  "Send".bright_yellow().bold(),
  "Sync".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch16-04-extensible-concurrency-sync-and-send.html".bright_cyan(),
  "unsafe impl".bright_yellow().bold(),
  );

  println!(
  "{0}\n\n\
  {solid_disc} A trait is unsafe when at least one of its methods has some invariant that the compiler can't verify
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Accessing Fields of a Union. Abbreviated afoau.
fn afoau_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Accessing Fields of a Union");

  println!(
  "The final action that works only with {0} is accessing fields of a union.\n\
  A union is similar to a {1}, but only one declared field is used in a particular instance at one time.\n\
  Unions are primarily used to interface with unions in C code.\n\
  Accessing union fields is unsafe because Rust can't guarantee the type of the data currently being stored in the union instance.\n\
  You can learn more about unions in the Rust Reference: {2}.
  ",
  "unsafe".bright_yellow().bold(),
  "struct".bright_yellow().bold(),
  "https://doc.rust-lang.org/reference/items/unions.html".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} A union is similar to a struct, but only one declared field is used in a particular instance at one time.\n\
  {solid_disc} Accessing union fields is unsafe because Rust can't guarantee the type of the data currently being stored in \
  the union instance.\n\
  {solid_disc} The size of a union is determined by the size of its largest field.

  ",
  "REMEMBER".bright_white().bold(),

  )
}

// Header: Using Miri to Check Unsafe Code. Abbreviated umtcuc.
fn umtcuc_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Using Miri to Check Unsafe Code");

  println!(
  "When writing unsafe code, you might want to check that what you have written actually is safe and correct.\n\
  One of the best ways to do that is to use Miri, an official Rust tool for detecting undefined behavior.\n\
  Whereas the borrow checker is a static tool that works at compile time, Miri is a dynamic tool that works at runtime.\n\
  It checks your code by running your program, or its test suite, and detecting when you violate the rules it understands about how \
  Rust should work.\n\n\
  See: {0}, for code sample and complete reading.\n\n\
  Miri doesn't catch everything you might get wrong when writing unsafe code.\n\
  Miri is a dynamic analysis tool, so it only catches problems with code that actually gets run.\n\
  That means you will need to use it in conjunction with good testing techniques to increase your confidence about the unsafe \
  code you have written.\n\
  Miri also does not cover every possible way your code can be unsound.\n\n\
  Put another way: If Miri does catch a problem, you know there's a bug, but just because Miri doesn't catch a bug doesn't mean \
  there isn't a problem.\n\
  It can catch a lot, though.\n\
  Try running it on the other examples of unsafe code in this chapter and see what it says!  
  ",
  "https://doc.rust-lang.org/book/ch20-01-unsafe-rust.html#using-miri-to-check-unsafe-code".bright_cyan()
  );

  println!(
  "{0}\n\n\
  {solid_disc} The borrow checker is a static tool that works at compile time, Miri is a dynamic tool that works at runtime.
  ",
  "REMEMBER".bright_white().bold()
  )
}

// Header: Using Unsafe Code Correctly. Abbreviated uucc.
fn uucc_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Using Unsafe Code Correctly");

  println!(
  "Using {0} to use one of the five superpowers just discussed isn't wrong or even frowned upon, but it is trickier to get {0} code \
  correct because the compiler can't help uphold memory safety.\n\
  When you have a reason to use {0} code, you can do so, and having the explicit {0} annotation makes it easier to track \
  down the source of problems when they occur.\n\
  Whenever you write unsafe code, you can use Miri to help you be more confident that the code you have written upholds Rust's \
  rules.\n\n\
  For a much deeper exploration of how to work effectively with unsafe Rust, read Rust's official guide for {0}, \
  The Rustonomicon: {1}.
  ",
  "unsafe".bright_yellow().bold(),
  "https://doc.rust-lang.org/nomicon/".bright_cyan()
  )
}


