use owo_colors::OwoColorize;

pub fn display_contents() {
  chapter_four_title();
  // wio_content();
  tsah_content();
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
  println!("{} \n", subheader_title.blue());

  println!("\
    {0} is Rust's most unique feature and it enables Rust to make memory safety guarantees without \
    needing a garbage collector. \n\
    Understanding ownership is key to becoming proficient in Rust programming. {1}.
  ",
  "Ownership".italic(),
  "So it’s important to understand how ownership works".bold(),  
  );

  // Here are the basic rules of \
  //   ownership:
  //   1. Each value in Rust has a variable that’s called its owner.
  //   2. There can only be one owner at a time.
  //   3. When the owner goes out of scope, the value will be dropped.

  println!(
    "{} is a set of rules that governs how a Rust program manages memory. \n\
    All programs have to manage the way they use a computer’s memory while running. \n\
    Some languages have a garbage collector that automatically looks for unused memory and frees it, \
    while other languages require the programmer to explicitly allocate and free memory. \n\
    Rust uses a third approach: memory is managed through a system of ownership with a set of rules that \
    the compiler checks at compile time. \n\
    If any of the rules are violated, the program won’t compile. \n\
    None of the features of ownership will slow down your program while it’s running. \n\n\
    When you understand ownership, you’ll have a solid foundation for understanding the features that make Rust unique.
  ",
  "Ownership".italic(),
  );



}

// Subheader: The Stack and the Heap. Abbreviated as tsah.
fn tsah_content() {
  // Subheader title.
  println!("{} \n", "The Stack and the Heap".blue());

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


}


