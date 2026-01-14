use crate::{chapter, menu::{self, clear_screen}};
use owo_colors::OwoColorize;

pub fn display_contents() {
  let _four_point_one = chapter::Section::new("4.1 What is Ownership?");
  let subheaders: [chapter::SubHeader; 13];
  subheaders = [
    chapter::SubHeader::new("What is Owenership", wio_content),
    chapter::SubHeader::new("The Stack and the Heap", tsah_content),
    chapter::SubHeader::new("Ownership Rules", or_content),
    chapter::SubHeader::new("Variable Scope", vs_content),
    chapter::SubHeader::new("The String Type", tst_content),
    chapter::SubHeader::new("Memory and Allocation", mal_content),
    chapter::SubHeader::new("Variables and Data interacting with Move", vadiwm_content),
    chapter::SubHeader::new("Deep copy vs Shallow copy", dcvssc_content),
    chapter::SubHeader::new("Scope and Assignment", sas_content),
    chapter::SubHeader::new("Variables and Data interacting with Clones", vadiwc_content),
    chapter::SubHeader::new("Stack-Only Data", sod_content),
    chapter::SubHeader::new("Ownership and Functions", oaf_content),
    chapter::SubHeader::new("Return Values and Scope", rvas_content),
  ];

  loop {
    chapter_four_title();
    // println!("{}", subheaders.len());
    let mut i:u8 = 1;
    for subheader in &subheaders {
      println!("{}. {}",i, subheader.get_title());
      i+= 1;
    }
    println!();

    'prompting_header_loop: loop {
      let selected_number = menu::headers_prompt();
      let selected_number = match selected_number {
        Ok(num) => {
          // println!("You selected {num}");
          if num as usize > subheaders.len() {
            continue;
          }
          num
        }
        Err(menu::HeaderPromptError::Quit) => {
          println!("Exiting program safely...");
          std::process::exit(0);   
        }
        Err(menu::HeaderPromptError::Io(err)) => {
          eprintln!("I/O error: {err}");
          return;
        }
        Err(menu::HeaderPromptError::Parse(err)) => {
          eprintln!("Parse error: {err}");
          // println!("Select an integer!");
          continue;
        }
      };

    
      if selected_number == 1 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 2 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 3 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 4 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 5 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 6 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 7 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 8 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 9 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 10 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 11 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 12 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }
      else if selected_number == 13 {
        menu::clear_screen();
        subheaders[(selected_number as usize) - 1].display_content();
      }    

      loop {
        match menu::post_header_prompt() {
          Ok(menu::PostHeaderPromptAction::ListSubheaders) => {
            clear_screen();
            break 'prompting_header_loop;
          }
          Ok(menu::PostHeaderPromptAction::Quit) =>std::process::exit(0),
          Err(menu::PostHeaderPromptError::InvalidOption(_)) => continue,
          Err(menu::PostHeaderPromptError::Io(err)) => {
            eprintln!("I/O error: {err}");
            return;
          }
        }        
      }
    };
  }
}


fn chapter_four_title() {
  let title = "Understanding Ownership";
  println!("{}", title.bright_blue().bold());
  println!("{} \n", "-".repeat(title.len()).bright_blue());
}

// Subheaders content below.

// Subheader 4.1: What is Ownership?. Abbreviated as wio.
fn wio_content() {
  let subheader_title = "4.1 What is Ownership?";
  println!("{} \n", subheader_title.bright_blue().bold());

  println!("\
    {0} is Rust's most unique feature and it enables Rust to make memory safety guarantees without \
    needing a garbage collector. \n\
    Understanding ownership is key to becoming proficient in Rust programming. {1}.
  ",
  "Ownership".italic(),
  "So it's important to understand how ownership works".bold(),  
  );

  // Here are the basic rules of \
  //   ownership:
  //   1. Each value in Rust has a variable that’s called its owner.
  //   2. There can only be one owner at a time.
  //   3. When the owner goes out of scope, the value will be dropped.

  println!(
    "{} is a set of rules that governs how a Rust program manages memory. \n\
    All programs have to manage the way they use a computer's memory while running. \n\
    Some languages have a garbage collector that automatically looks for unused memory and frees it, \
    while other languages require the programmer to explicitly allocate and free memory. \n\
    Rust uses a third approach: memory is managed through a system of ownership with a set of rules that \
    the compiler checks at compile time. \n\
    If any of the rules are violated, the program won't compile. \n\
    None of the features of ownership will slow down your program while it's running. \n\n\
    When you understand ownership, you'll have a solid foundation for understanding the features that make Rust unique.
  ",
  "Ownership".italic(),
  );



}

// Subheader: The Stack and the Heap. Abbreviated as tsah.
fn tsah_content() {
  // Subheader title.
  println!("{} \n", "The Stack and the Heap".bright_blue().bold());

  println!(
    "To understand ownership, you also need to understand how Rust manages memory. \n\
    Both the stack and the heap are parts of memory available to your code to use at runtime, but they are structured in different ways. \n"
  );

  println!(
    "The stack is where values with a known, fixed size are stored. \n\
    The stack stores values in the order it gets them and removes the values in the opposite order. \n\
    The stack operates in a last-in, first-out manner, meaning that the last value pushed onto the stack is the first one to be popped off. \n\
    Adding or removing plates from the middle or bottom wouldn’t work as well! \n\
    Adding data is called {0}, and removing data is called {1}. \n\
    Values stored on the stack must have a known, fixed size at compile time so that the compiler can allocate the appropriate amount of space.
    ",
    "pushing onto the stack".bright_green().italic(),
    "popping off the stack".bright_green().italic()
  );

  println!(
    "The heap, on the other hand, is used for values with an unknown or dynamic size. \n\
    The heap is less organized: when you put data on the heap, you request a certain amount of space. \n\
    The {0} finds an empty spot in the heap that is big enough, marks it as being in use, and returns a {1}, \
    which is the address of that location. \n\
    This process is called {2} and is sometimes abbreviated as just {3} \
    ({4}). \n\
    Because the pointer to the heap is a known, fixed size, you can store the pointer on the stack, \
    but when you want the actual data, you must follow the pointer.    
  ",
  "memory allocator".bright_green().italic(),
  "pointer".bright_green().italic(),
  "allocating on the heap".bright_green().italic(),
  "allocating".bright_green().italic(),
  "pushing values onto the stack is not considered allocating".red()
  );

  println!(
    "{} because the allocator never \
    has to search for a place to store new data; that location is always at the top of the stack. \n\
    Comparatively, allocating space on the heap requires more work because the allocator must \
    first find a big enough space to hold the data and then perform bookkeeping to prepare for the next allocation.    
  ",
  "Pushing to the stack is faster than allocating on the heap".bright_white().bold()
  );

  println!(
    "Unlike the stack, values on the heap can grow and shrink in size as needed. \n\
    However, {} because it involves more complex memory management (because you have to follow a pointer to get there). \n\
    Contemporary processors are faster if they jump around less in memory. \n\
    A processor can usually do its job better if it works on data that’s close to other \
    data (as it is on the stack) rather than farther away (as it can be on the heap).
  ",
  "accessing data on the heap is generally slower than accessing data on the stack".bright_white().bold()
  );

  println!(
    "When your code calls a function, the values passed into the function (including, \
    potentially, pointers to data on the heap) and the function’s local variables get \
    pushed onto the stack. When the function is over, those values get popped off the stack.
  ");

  println!(
    "Keeping track of what parts of code are using what data on the heap, minimizing the \
    amount of duplicate data on the heap, and cleaning up unused data on the heap so you \
    don’t run out of space are all problems that ownership addresses. \n\
    Once you understand ownership, you won’t need to think about the stack and the heap \
    very often, but knowing that {} can \
    help explain why it works the way it does.    
  ",
  "the main purpose of ownership is to manage heap data".italic().bold()
  );

  println!(
    "Want to read more? \n\
    See: {}    
  ",
  "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#the-stack-and-the-heap".cyan()
  )

}

// Subheader: Ownership Rules. Abbreviated as or.
fn or_content() {
  // Subheader title.
  println!("{} \n", "Ownership Rules".bright_blue().bold());

  println!(
    "Ownership has three main rules that govern how it works: \n\
    1. Each value in Rust has a variable that’s called its owner. \n\
    2. There can only be one owner at a time. \n\
    3. When the owner goes out of scope, the value will be dropped. \n\n\
    These rules are checked at compile time, and if any of them are violated, \
    the program will not compile.
  ");
}

// Subheader: Variable Scope. Abbreviated as vs.
fn vs_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Variable Scope".bright_blue().bold());

  println!(
    "We'll look at the scope of some variables. A {0} is the range within a program for which an item is valid. \n\
    The variable is valid from the point at which it's declared until the end of the current {0}. \n\
    See: {1}, for code sample. \n\n\
    In other words, there are two important points in time here: \n\
    {two_spaces}{solid_disc} The point at which the variable comes into scope, which is when it is declared; \n\
    {two_spaces}{solid_disc} It remains valid until the end of the scope in which it is declared.
    
  ",
  "scope".bright_green().italic(),
  "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#listing-4-1".cyan()
  );
}

// Subheader: The String Type. Abbreviated as tst.
fn tst_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "The String Type".bright_blue().bold());

  println!(
    "Rust has multiple string types: \n\
    {two_spaces}{solid_disc} String literals - which are immutable fixed-length \
    strings stored in the binary's read-only memory section. \n\
    {two_spaces}{solid_disc} The String type - which is a growable, mutable, UTF-8 encoded string type. \n\n\
  ");
  
  println!(
    "String literals where a string value is hardcoded into our program (hardcoded into the text of our program). \n\
    String literals are convenient, but they aren't suitable for every situation in which we may want to use text. \n\
    One reason is that they're immutable. \n\
    Another is that not every string value can be known when we write our code: for example, what if we want to take user input and store it? \n\
    For these situations, Rust has a second string type, {0} which is part of the standard library. \n\
    This type manages data allocated on the heap and as such is able to store an amount of text that is unknown to us at compile time. \n\n\
    Code sample of a string literal: {1} \n\n\
    You can create a {0} from a string literal using the {2} function, like so: \n\
    {3} \n\n\
    The double colon {4} operator allows us to namespace this particular {2} function under the {0} type rather than using some sort of name \
    like {5}. 
  ",
  "String".bright_yellow().bold(),
  "let s: &str = \"hello world\";".bright_yellow(),
  "from".bright_yellow().bold(),
  "let mut s: String = String::from(\"hello world\");".bright_yellow(),
  "::".bright_yellow(),
  "string_from".bright_yellow().bold()
  )
}

// Subheader: Memory and Allocation. Abbreviated as mal.
fn mal_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Memory and Allocation".bright_blue().bold());

  println!(
    "So, what's the difference between string literals and String? \
    Why can {0} be mutated but literals cannot? \
    The difference is in how these two types deal with memory.
  ",
  "String".bright_yellow().bold()
  );

  println!(
    "In the case of a string literal, we know the contents at compile time, \
    so the text is hardcoded directly into the final executable. \n\
    This is why string literals are fast and efficient. \
    But these properties only come from the string literal's immutability. \n\
    {0}, we can't put a blob of memory into the binary for each piece \
    of text whose size is unknown at compile time and whose size might change while running the program.
  ",
  "Unfortunately".red()
  );

  println!(
    "With the {0} type, in order to support a mutable, growable piece of text, we need \
    to allocate an amount of memory on the heap, unknown at compile time, to hold the contents. \
    This means: \n\
    {two_spaces}{solid_disc} The memory must be requested from the memory allocator at runtime. \n\
    {two_spaces}{solid_disc} We need a way of returning this memory to the allocator when we're done with our {0}. \n\n\
    That first part is done by us: when we call {1}, {2}. \n\
    This is pretty much universal in programming languages.
  ",
  "String".bright_yellow().bold(),
  "String::from" .bright_yellow(),
  "its implementation requests the memory it needs".bright_white().bold()
  );

  println!(
    "The second part, returning the memory to the allocator, is where \
    Rust's ownership system comes into play. \n\
    When a variable goes out of scope, Rust calls a special function \
    called {0} for us. \n\
    This function is where we would put the code to return the memory \
    to the allocator. \n\
    Because Rust calls {0} automatically at the closing curly bracket \
    of a scope, we don't need to remember to free our memory ourselves, \
    and we never accidentally forget to free it. \n\n\
    {1}: In C++, this pattern of deallocating resources at the end of an \
    item's lifetime is sometimes called {2}. \n\
    The {0} function in Rust will be familiar to you if you've used RAII patterns. \n\n\
    If you want to read more, see: {3}
    ",
    "drop".bright_yellow().bold(),
    "Note".red(),
    "Resource Acquisition Is Initialization (RAII)".italic(),
    "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#memory-and-allocation".cyan()
    );

}

// Subheader: Variables and Data interacting with Move. Abbreviated as vadiwm.
fn vadiwm_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Variables and Data Interacting with Move".bright_blue().bold());

  println!(
    "Multiple variables can interact with the same data in different ways in Rust. \
    Let’s look at an example using an integer: \n\n\
    {0} \n\
    Listing 4-2: Assigning the integer value of variable {1} to {2}. \n\n\
    We can probably guess what this is doing: “bind the value {3} to {1}; \
    then make a copy of the value in {1} and bind it to {2}. \n\
    We now have two variables, {1} and {2}, and both equal {3}. \n\
    This is indeed what is happening, because integers are simple values with \
    a known, fixed size, and these two {3} values are pushed onto the stack.
  ",
    "let x = 5; \n\
    let y = x;".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "5".bright_yellow().bold()
  );

  println!(
    "Now let's look at the {0} version: \n\n\
    {1} \n\
    This looks very similar, so we might assume that the way it works would be the same: \
    that is, the second line would make a copy of the value in {2} and bind it to {3}. \n\
    But this isn’t quite what happens.
  ",
    "String".bright_yellow().bold(),
    "let s1 = String::from(\"hello\"); \n\
    let s2 = s1;
    ".bright_yellow().bold(),
    "s1".bright_yellow().bold(),
    "s2".bright_yellow().bold()
  );

  println!(
    "Let's see what is happening to {0} under the covers. \n\n\
    A {0} is made up of three parts: \n\
    {two_spaces}{solid_disc} A {1} to the memory that holds the contents of the string. \n\
    {two_spaces}{solid_disc} A {2} value that tells us how much memory, in bytes, the contents of the {0} are currently using. \n\
    {two_spaces}{solid_disc} A {3} value that tells us how much memory, in bytes, that the {0} has received from the allocator. \n\n\
    This group of data is stored on the stack. \n\
    When we assign {4} to {5}, the {0} data is copied, meaning we copy the pointer, \
    the length, and the capacity that are on the stack. \n\
    {6}.
  ",
    "String".bright_yellow().bold(),
    "pointer".bright_yellow().bold(),
    "length".bright_yellow().bold(),
    "capacity".bright_yellow().bold(),
    "s1".bright_yellow().bold(),
    "s2".bright_yellow().bold(),
    "We do not copy the data on the heap that the pointer refers to".bright_white().bold()
  );

  println!(
    "If Rust did copy the heap data, the operation {0} {1}.
  ",
    "s2 = s1".bright_yellow().bold(),
    "could be very expensive in terms \
    of runtime performance if the data on the heap were large".red()
  );

  println!(
    "{5} \n\n\
    Earlier, we said that when a variable goes out of scope, Rust automatically calls \
    the {0} function and cleans up the heap memory for that variable. \n\
    But both {1} and {2}'s pointers point to the same location on the heap. \n\
    {3}: when {2} and {1} go out of scope, they will both try to free the same memory. \n\
    This is known as a {4} and is one of the memory safety bugs.\n\
    Freeing memory twice can lead to memory corruption, which can potentially lead to security vulnerabilities.
  ",
    "drop".bright_yellow().bold(),
    "s1".bright_yellow().bold(),
    "s2".bright_yellow().bold(),
    "This is a problem".red(),
    "double free error".red(),
    "THE PROBLEM!".red()
  );

  println!(
    "{} \n\n\
    To ensure memory safety, after the line {1}, Rust considers {2} as no longer valid. \n\
    Therefore, Rust doesn’t need to free anything when {2} goes out of scope.
  ",
    "ENSURING MEMORY SAFETY".bright_green().bold(),
    "let s2 = s1;".bright_yellow().bold(),
    "s1".bright_yellow().bold()
  );

  println!(
    "{} \n\n\
    {two_spaces}{solid_disc} Data types with an unknown size at compile time implement the {1} trait \
    which means their ownership can be moved. \n\
    {two_spaces}  In other words, Rust is going to make the original owners invalid at the point where the assignment happens. \n\
    {two_spaces}{solid_disc} Data types with a known, fixed size at compile time implement the {2} trait, \
    which means original owners are still valid after assignment occurs (In other words, they do not transfer ownership!). \n\
  ",
    "REMEMBER".bright_white().bold(),
    "Move".bright_yellow().bold(),
    "Copy".bright_yellow().bold()
  );
}

// Subheader: Deep copy vs Shallow copy. Abbreviated as dcvssc.
// I extracted this content from the Variables and Data interacting with Move section
fn dcvssc_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Deep Copy vs Shallow Copy".bright_blue().bold());

  println!(
    "Let's use this code sample as a reference: \n\n\
    {0} \n\n\
    If you’ve heard the terms {1} and {2} while working with other \
    languages, the concept of copying the pointer, length, and capacity without copying \
    the data probably sounds like making a shallow copy. \n\
    But because But because Rust also invalidates the first variable, instead of being \
    called a shallow copy, it’s known as a {3}. \n\
    In this example, we would say that {4} was moved into {5}. \n\n\
    That's how Rust ensures memory safety! With only {5} valid, when it goes out of scope it alone will free the memory, and we’re done. \n\n\
    In addition, there’s a design choice that’s implied by this: {6}. Therefore, any automatic copying can be assumed to be inexpensive in terms of runtime performance. \n\n\
    {7} \n\n\
    {two_spaces}{solid_disc} With types with an unknown size at compile time, \
    {1} means copying the stack data but not the heap data. \n\
    {two_spaces}  {8} means copying both the stack data and the heap data. \n\
    {two_spaces}{solid_disc} With types with a known, fixed size at compile time, \
    {1} and {2} have no difference, they only copy the stack data. \n\
  ",
    "let s1 = String::from(\"hello\"); \n\
    let s2 = s1;".bright_yellow().bold(),
    "shallow copy".italic(),
    "deep copy".italic(),
    "move".italic(),
    "s1".bright_yellow().bold(),
    "s2".bright_yellow().bold(),
    "Rust will never automatically \
    create “deep” copies of your data".bright_white().bold(),
    "REMEMBER!".bright_white().bold(),
    "Deep copy".italic()
  )
  
}

// Subheader: Scope and Assignment. Abbreviated as sas.
fn sas_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Scope and Assignment".bright_blue().bold());

  println!(
    "When you assign a completely new value to an existing variable, Rust will call \
    {0} and free the original value’s memory immediately. \n\
    Consider this code, for example: \n\n\
    {1} \n\n\
    We initially declare a variable {2} and bind it to a {3} with the value {4}. \n\
    Then we immediately create a new {3} with the value {5} and assign it to {2}. \n\
    At this point, nothing is referring to the original value on the heap at all. \n\
    {6}. \n\
    To avoid a memory leak, Rust will automatically run the {0} on it and its \
    memory associated with the original value will be freed right away. \n\
    When we print the value at the end, it will be {5}. \n\n\
    {7} \n\n\
    {two_spaces}{solid_disc} The variable {2} is valid from the point it is first declared until the end of the scope. \n\
    {two_spaces}{solid_disc} When we assign a new value to {2}, the original value is dropped \
    and the new value takes its place. \n\
    {two_spaces}  Rust will do this at any point immediately when nothing is referring to the original value anymore,so, \n\
    {two_spaces}  like the value goes out of scope immediately!
  ",
    "drop".bright_yellow().bold(),
    "let s = String::from(\"hello\"); \n\
    s = String::from(\"world\"); \n\n\
    println!(\"{s}\")".bright_yellow().bold(),
    "s".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "\"hello\"".bright_yellow().bold(),
    "\"world\"".bright_yellow().bold(),
    "The original string thus immediately goes out of scope".bright_white().bold(),
    "REMEMBER!".bright_white().bold(),
  );
}

// Subheader: Variables and Data interacting with Clones. Abbreviated as vadiwc.
fn vadiwc_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Variables and Data Interacting with Clones".bright_blue().bold());

  println!(
    "If we do want to deeply copy the heap data of the {2}, not just the stack data, \
    we can use a common method called {0}.  \n\
    To do this, Rust has a {0} method that you can call to \
    explicitly create a deep copy of your data. \n\n\
    Here’s an example: \n\n\
    {1} \n\n\
    In this code, we create a {2} from a string literal and bind it to {3}. \n\
    Then we call the {4} method on {3} and bind the resulting deep \
    copy to {0}. \n\
    Now both {3} and {0} own their own separate copies of the \
    data on the heap. \n\
    We can prove this by printing both values, which will show \
    that they are both valid and contain the same data. \n\n\
    When you see a call to {0}, you know that some arbitrary code is being executed \
    and that code may be expensive. \n\
    It’s a visual indicator that something different is going on.
  ",
    "clone".bright_yellow().bold(),
    "let s1 = String::from(\"hello\"); \n\
    let s2 = s1.clone(); \n\n\
    println!(\"s1 = {}, s2 = {}\", s1, s2);".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "s1".bright_yellow().bold(),
    "s2".bright_yellow().bold()
  );
}

// Subheader: Stack-Only Data: Copy Trait. Abbreviated sod.
fn sod_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Stack-Only Data: Copy Trait".bright_blue().bold());

  println!(
    "Consider this code, for example: \n\n\
    {0} \n\n\
    Types such as integers that have a known size at compile time are stored entirely \
    on the stack, so copies of the actual values are quick to make. \n\
    That means there’s no reason we would want to prevent {1} from being valid after we \
    create the variable {2}. \n\
    In other words, there’s no difference between deep and shallow copying here, so \
    calling {3} wouldn’t do anything different from the ussual shallow copying, and \
    we can leave it out. \n\n\
    Rust has a special annotation called the {4} trait that we can place on types \
    that are stored on the stack, as integers. \n\
    If a type implements the {4} trait, variables that use it do not move, but rather \
    are trivially copied, making them still valid after assignment to another variable. \n\n\
    {14} with {4} if the type, or any of its parts, has \
    implemented the {5} trait. \n\
    If the type needs something special to happen when the value goes out of scope and \
    we add the {4} annotation to that type, we’ll get a compile-time error. \n\n\
    So, what types implement the {4} trait? \n\
    You can check the documentation for the given type to be sure, but as a general rule, \
    any group of simple scalar values can implement {4}, and nothing that requires \
    allocation or is some form of resource can implement {4}. \n\
    Here are some of the types that implement {4}: \n\n\
    {two_spaces}{solid_disc} All the integer types, such as {6}. \n\
    {two_spaces}{solid_disc} The Boolean type, {7}, with values {8} and {9}. \n\
    {two_spaces}{solid_disc} All the floating point types, such as {10}. \n\
    {two_spaces}{solid_disc} The char type, {11}. \n\
    {two_spaces}{solid_disc} Tuples, if they only contain types that also implement {4}. \n\
    {two_spaces}  For example, {12} implements Copy, but {13} does not. \n\n\
    {15}! \n\n\
    {two_spaces}{solid_disc} Types that are stored entirely on the stack implement the {4} trait \
    because copying stack data is not expensive and they don't transfer \
    ownership. \n\
    {two_spaces}{solid_disc} If a type implements the {4} trait, \
    we can make as many copies of that type as we want without \
    invalidating the original. \n\
    {two_spaces}{solid_disc} Types that are stored on the heap \
    generally do not implement the {4} trait. They implement the {16} trait meaning they transfer ownership \
    because copying heap data is expensive. \
    They also implement the {5} trait so their heap memory is cleaned up when they go out of scope. \
    And you can't annotate them with {4} trait. \n\
    {two_spaces}{solid_disc} If a type does not implement the {4} trait, \
    then when we assign it to another variable, the first variable \
    will no longer be valid.
  ",
    "let x = 5; \n\
    let y = x; \n\n\
    println!(\"x = {x}, y = {y}\");".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "clone".bright_yellow().bold(),
    "Copy".bright_yellow().bold(),
    "Drop".bright_yellow().bold(),
    "i32".bright_yellow().bold(),
    "bool".bright_yellow().bold(),
    "true".bright_yellow().bold(),
    "false".bright_yellow().bold(),
    "f64".bright_yellow().bold(),
    "char".bright_yellow().bold(),
    "(i32, i32)".bright_yellow().bold(),
    "(i32, String)".bright_yellow().bold(),
    "Rust won’t let us annotate a type".red(),
    "REMEMBER".bright_white().bold(),
    "Move".bright_yellow().bold(),
  )
}

// Subheader: Ownership and Functions. Abbreviated as oaf.
fn oaf_content() {
  let solid_disc = "\u{2022}";

  // Subheader title.
  println!("{} \n", "Ownership and Functions".bright_blue().bold());

  println!(
    "The mechanics of passing a value to a function are similar to those when assigning \
     a value to a variable. \n\
     {0}. \n\
     See: {1}, for code sample.
  ",
    "Passing a variable to a function will move or copy, just as assignment does".bright_white().bold(),
    "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#listing-4-3".cyan()
  );

  println!(
    "{0} \n\n\
    {solid_disc} Funtions will take ownership if the parameter type does not implement the {1} trait. \n\
    {solid_disc} Functions will make a copy of the value if the parameter type implements the {1} trait. \n\
    {solid_disc} When the function ends, parameters and locally defined variables go out \
    of scope, and if it they store data on the heap, they will be dropped unless they \
    are returned. \n\
    {solid_disc} If you want to return ownership of a value from a function, \
    you can return it as part of the function's return value.
  ",
    "REMEMBER!".bright_white().bold(),
    "Copy".bright_yellow().bold(),
  );
}

// Subheader: Return Values and Scope. Abbreviated as rvas.
fn rvas_content() {
  let solid_disc = "\u{2022}";
  // let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Return Values and Scope".bright_blue().bold());

  println!(
    "Returning values can also transfer ownership. \n\
    {0}. \n\
    See: {1}, for code sample. \n\n\
    The ownership of a variable follows the same pattern every time: assigning a value \
    to another variable moves it. \n\
    When a variable that includes data on the heap goes out of scope, the value will be \
    cleaned up by {2} unless ownership of the data has been moved to another variable. \n\n\
    While this works, {3}. \n\
    What if we want to let a function use a value but not take ownership? \n\
    {4}, in addition to any data resulting from the body of the function that \
    we might want to return as well. \n\n\
    Rust does let us return multiple values using a tuple, as shown in Listing 4-5. \n\
    See: {5}, for code sample. \n\n\
    Luckily for us, Rust has a feature for using a value without transferring ownership, called {6}.
  ",
    "When you return a value, you transfer ownership to the calling function".bright_white().bold(),
    "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#listing-4-4".cyan(),
    "drop".bright_yellow().bold(),
    "taking ownership and then returning ownership with every function is a bit tedious".red(),
    "It’s quite annoying that anything we pass in also needs to be passed back if we want \
    to use it again".red(),
    "https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html#listing-4-5".cyan(),
    "references".italic()
  );

  println!(
    "{0} \n\n\
    {solid_disc} Functions can take ownership also transfer and return ownership. \n\
    {solid_disc} Returning a value from a function will move or copy, just as assignment does. \n\
    {solid_disc} If you want to return multiple values, you can use a tuple. \n\
    {solid_disc} You can use a value on the heap without transferring ownership using {1} or {2}.
  ",
    "REMEMBER!".bright_white().bold(),
    "references".italic(),
    "borrowing".italic()
  );
}





