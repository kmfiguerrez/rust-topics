use owo_colors::OwoColorize;

pub fn display_contents() {
  chapter_six_title();
  // dae_content();
  // ev_content();
  // oaoeos_content();
  // dem_content();
  // toe_content();
  bon_content();
}

fn chapter_six_title() {
  let title = "Enums and Pattern Matching";
  println!("{}", title.bright_blue().bold());
  println!("{} \n", "-".repeat(title.len()).bright_blue());
}

// Subheaders content below.
// Subheader: Defining an Enum. Abbreviated as dae.
fn dae_content() {
  let solid_disc = "\u{2022}";
  let two_spaces = "\u{2003}\u{2003}";

  // Subheader title.
  println!("{} \n", "Defining an Enum".bright_blue().bold());

  println!(
    "Where structs give you a way of grouping together related fields and data, like a \
    {0} with its {1} and {2}, {3}. \n\
    For example, we may want to say that {0} is one of a set of possible shapes \
    that also includes {4} and {5}. \n\
    To do this, Rust allows us to encode these possibilities as an enum.
  ",
    "Rectangle".bright_yellow().bold(),
    "width".bright_yellow().bold(),
    "height".bright_yellow().bold(),
    "enums give you a way of saying a value is one \
    of a possible set of values".bright_white().bold(),
    "Circle".bright_yellow().bold(),
    "Triangle".bright_yellow().bold(),
  );

  println!(
    "Let’s look at a situation we might want to express in code and see why enums are \
    useful and more appropriate than structs in this case. \n\
    Say we need to work with IP addresses. Currently, two major standards are used for \
    IP addresses: version four and version six. \n\
    Because these are the only possibilities for an IP address that our program will come \
    across, we can {0} all possible variants, which is where enumeration gets its \
    name.
  ",
    "enumerate".italic()
  );

  println!(
    "Any IP address can be either a version four or a version six address, but not both \
    at the same time. \n\
    That property of IP addresses makes the enum data structure appropriate {0}. \n\
    Both version four and version six addresses are still fundamentally IP addresses, \
    so they should be treated as the same type when the code is handling situations that \
    apply to any kind of IP address.
  ",
    "because an enum value can only be one of its variants".bright_white().bold()
  );

  println!(
    "We can express this concept in code by defining an {} enumeration and \
    listing the possible kinds an IP address can be, V4 and V6. These are the variants \
    of the enum: \n\
    {} \n\
    {0} is now a custom data type that we can use elsewhere in our code.
  ",
    "IpAddrKind".bright_yellow().bold(),
    "enum IpAddrKind { \n\
    \tV4, \n\
    \tV6, \n\
    }
    ".bright_yellow().bold()
  );

  println!(
    "You can think of the main reason of Rust's enum is to model real-world states. \n\
    For example: A network connection can be: \n\
    {two_spaces}{solid_disc} Disconnected \n\
    {two_spaces}{solid_disc} Connecting \n\
    {two_spaces}{solid_disc} Connected (with an IP address) \n\n\
    {0} \n\
    Now: \n\
    {solid_disc} Impossible to represent invalid states \n\
    {solid_disc} Data is tied to the state that needs it \n\
    {solid_disc} Compiler enforces correctness \n\n\
    Let's show an example without enums ({1}) \n\
    We'll use 0-Disconnected, 2-Connecting and 1-Connected for state. \n\n\
    {2} \n\
    Problems: \n\
    {solid_disc} Invalid states possible \n\
    {solid_disc} state == 1 but ip == None \n\
    {solid_disc} Compiler can't help you
  ",
    "enum Connection { \n\
    \tDisconnected, \n\
    \tConnecting, \n\
    \tConnected(String), \n\
    }
    ".bright_yellow().bold(),
    "bad/unsafe".red(),
    "struct Connection { \n\
    \tstate: u8, \n\
    \tip: Option<String> \n\
    }
    ".bright_yellow().bold()
  );

  println!(
    "\n{} \n\n\
    {solid_disc} A Rust enum lets you define a type that can be one of several variants. \n\
    {solid_disc} The main reason of Rust enum is to model real-world states.
  ",
    "REMEMBER".bright_white().bold()
  );

  
}

// Subheader: Enum Values. Abbreviated as ev.
fn ev_content() {
  // Subheader title
  println!("{} \n", "Enum Values".bright_blue().bold());

  println!(
    "We can create instances of each of the two variants of IpAddrKing like this: \n\n\
    {} \n\n\
    Note that the variants of the enum are namespaced under its identifier, and we use \
    a double colon to separate the two. \n\
    This is useful because now both values {1} and {2} are of the same type: {3}. \n\
    We can then, for instance, define a function that takes any {3}: \n\n\
    {4} \n\n\
    And we can call this function with either variant: \n\n\
    {5}
  ",
    "enum IpAddrKind { \n\
    \tV4, \n\
    \tV6, \n\
    } \n\n\
    let four = IpAddrKind::V4; \n\
    let six = IpAddrKind::V6;
    ".bright_yellow().bold(),
    "IpAddrKind::V4".bright_yellow().bold(),
    "IpAddrKind::V6".bright_yellow().bold(),
    "IpAddrKind".bright_yellow().bold(),
    "fn route(ip_kind: IpAddrKind) {}".bright_yellow().bold(),
    "route(IpAddrKind::V4); \n\
    route(IpAddrKind::V6);
    ".bright_yellow().bold(),
  );

  println!(
    "We know that each variant can carry data. So, we don't need an enum inside a struct \
    like in listing 6-1: {0} \n\
    Now let's put data directly into each enum variant by having a new definition of \
    the {1} enum that says both {2} and {3} variants will have associated {4} values: \n\n\
    {5}
  ",
    "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#listing-6-1".cyan(),
    "IpAddr".bright_yellow().bold(),
    "V4".bright_yellow().bold(),
    "V6".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "enum IpAddr { \n\
    \tV4(String), \n\
    \tV6(String), \n\
    } \n\n\
    let home = IpAddr::V4(String::from(\"127.0.0.1\")); \n\
    let loopback = IpAddr::V6(String::from(\"::1\"));
    ".bright_yellow().bold(),
  );

  println!(
    "We attach data to each variant of the enum directly, so there is no need for an extra \
    struct. \n\
    Here, it’s also easier to see another detail of how enums work: The name of each enum \
    variant that we define also becomes a function that constructs an instance of the enum. \n\
    That is, {} is a function call that takes a {1} argument and returns an instance of the {2} type. \n\
    We automatically get this constructor function defined as a result of defining the enum.
  ",
    "IpAddr::V4()".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "IpAddr".bright_yellow().bold(),
  );

  println!(
    "Read more? See: {}
  ",
    "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#enum-values".cyan()
  );
}

// Subheader: Other advantages of enums over struct. Abbreviated as oaoeos.
// I extracted this content from the Enum Values section.
fn oaoeos_content() {
  // Subheader title.
  println!("{} \n", "Other advantages of enums over struct".bright_blue().bold());

  println!(
    "There’s another advantage to using an enum rather than a struct: Each variant can \
    have different types and amounts of associated data. \n\
    For example: Version four IP addresses will always have four numeric components that \
    will have values between 0 and 255. \n\
    If we wanted to store {} addresses as four {} values but still express {2} addresses \
    as one {3} value, we wouldn’t be able to with a struct. \n\
    Enums handle this case with ease: \n\n\
    {4}
  ",
    "V4".bright_yellow().bold(),
    "u8".bright_yellow().bold(),
    "V6".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "enum IpAddr { \n\
    \tV4(u8, u8, u8, u8), \n\
    \tV6(String), \n\
    } \n\n\
    let home = IpAddr::V4(127, 0, 0, 1); \n\
    let loopback = IpAddr::V6(String::from(\"::1\"));
    ".bright_yellow().bold()
  );  

  println!(
    "As you can see, you can put any kind of data inside an enum variant: \
    strings, numeric types, structs and even enum! \n\
    Also, standard library types are often not much more complicated than what \
    you might come up with.
  ");

  println!(
    "Let’s look at another example of an enum in Listing 6-2: {} \n\
    Defining an enum with variants such as the ones in Listing 6-2 is similar to \
    defining different kinds of struct definitions, except the enum doesn’t use the \
    {} keyword and all the variants are grouped together under the {} type. \n\n\
    But if we used the different structs, each of which has its own type, we couldn’t as \
    easily define a function to take any of these kinds of messages as we could with the \
    {2} enum defined in Listing 6-2, which is a single type.
  ",
    "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#listing-6-2".cyan(),
    "struct".bright_yellow().bold(),
    "Message".bright_yellow().bold()
  );
}

// Subheader: Defining Enum methods. Abbreviated as dem.
// I extracted this part from the Other advantages of enums over struct section.
fn dem_content() {
  // Subheader title.
  println!("{} \n", "Defining Enum methods".bright_blue().bold());

  println!(
    "There is one more similarity between enums and structs: Just as we’re able \
    to define methods on structs using {0}, we’re also able to define methods on enums. \n\
    Here’s a method named {1} that we could define on our {2} enum: \n\n\
    {3}
  ",
    "impl".bright_yellow().bold(),
    "call".bright_yellow().bold(),
    "Message".bright_yellow().bold(),
    "impl Message { \n\
    \tfn call(&self) { \n\
    \t// method body would be defined here \n\
    \t} \n\
    }\n\
    let m = Message::Write(String::from(\"hello\")); \n\
    m.call();
    ".bright_yellow().bold()
  );

  println!(
    "The body of the method would use {0} to get the value that we called the method on. \n\
    In this example, we’ve created a variable {1} that has the value {2}, and that is \
    what {0} will be in the body of the {3} method when {4} runs.
  ",
    "self".bright_yellow().bold(),
    "m".bright_yellow().bold(),
    "Message::Write(String::from(\"hello\"))".bright_yellow().bold(),
    "call".bright_yellow().bold(),
    "m.call()".bright_yellow().bold()
  );
}

// Subheader: The Option Enum. Abbreviated as toe.
fn toe_content() {
  // Subheader title.
  println!("{} \n", "The Option Enum".bright_blue().bold());

  println!(
    "This section explores a case study of {0}, which is another enum defined by the \
    standard library. \n\
    The {0} type encodes the very common scenario in which a value \
    could be something, or it could be nothing.
  ",
    "Option".bright_yellow().bold()
  );

  println!(
    "For example, if you request the first item in a non-empty list, you would get a \
    value. If you request the first item in an empty list, you would get nothing. \n\
    Expressing this concept in terms of the type system means the compiler can check \
    whether you’ve handled all the cases you should be handling; \
    this functionality can prevent bugs that are extremely common in other programming \
    languages.
  ");

  println!(
    "Rust doesn’t have the null feature that many other languages have. \n\
    Null is a value that means there is no value there. \n\
    {}
  ",
    "In languages with null, variables can always be in one of two states: \
    null or not-null.".red()
  );

  println!(
    "The problem with null values is that if you try to use a null value as a not-null \
    value, you’ll get an error of some kind. \n\
    Because this null or not-null property is pervasive, it’s extremely easy to make \
    this kind of error. \n\
  ");

  println!(
    "However, the concept that null is trying to express is still a useful one: A null \
    is a value that is currently invalid or absent for some reason. \n\
    The problem isn’t really with the concept but with the particular implementation.
  ");

  println!(
    "As such, Rust does not have nulls, but it does have an enum that can encode the \
    concept of a value being present or absent. \n\
    This enum is {0}, and it is defined by the standard library as follows: \n\n\
    {1} \n\
    The {0} enum is so useful that it’s even included in the prelude; you don’t \
    need to bring it into scope explicitly. \n\
    Its variants are also included in the prelude: \
    You can use {2} and {3} directly without the {4} prefix. \n\
    The {0} enum is still just a regular enum, and {5} and {3} are still \
    variants of type {0}.
  ",
    "Option<T>".bright_yellow().bold(),
    "enum Option<T> { \n\
    \tNone, \n\
    \tSome(T), \n\
    }
    ".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "None".bright_yellow().bold(),
    "Option::".bright_yellow().bold(),
    "Some(T)".bright_yellow().bold(),
  );

  println!(
    "The {0} syntax is a feature of Rust, It’s a generic type parameter. \n\
    For now, all you need to know is that {0} means that the {1} variant of the \
    {2} enum can hold one piece of data of any type, and that each concrete type that \
    gets used in place of {3} makes the overall {4} type a different type. \n\n\
    Here are some examples of using {2} values to hold number types and char types: \n\n\
    {5}
  ",
    "<T>".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold(),
    "let some_number = Some(5); \n\
    let some_char = Some('e'); \n\
    let absent_number: Option<i32> = None;
    ".bright_yellow().bold()
  );

  println!(
    "The type of {0} is {1}. \n\
    The type of {2} is {3}, which is a different type. \n\
    Rust can infer these types because we’ve specified a value inside the {4} variant. \n\
    For {5}, Rust requires us to annotate the overall {6} type: \
    The compiler can’t infer the type that the corresponding {4} variant will hold by \
    looking only at a {7} value. \n\
    In the code above, we tell Rust that we mean for {8} to be of type {1}.

  ",
    "some_number".bright_yellow().bold(),
    "Option<i32>".bright_yellow().bold(),
    "some_char".bright_yellow().bold(),
    "Option<char>".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "absent_number".bright_yellow().bold(),
    "Option".bright_yellow().bold(),
    "None".bright_yellow().bold(),
    "absent_number".bright_yellow().bold(),
  );
}

// Subheader: Benefits of Option<T> over null. Abbreviated as bon.
// I extracted this part from the The Option Enum section.
fn bon_content() {
  // Subheader title.
  println!("{} \n", "Benefits of Option<T> over null".bright_blue().bold());

  println!(
    "When we have a {} value, we know that a value is present, and the value is held within the {0}. \n\
    When we have a {1} value, in some sense it means the same thing as null: We don’t have a valid value. \n\
    So, why is having {2} any better than having null?
  ",
    "Some".bright_yellow().bold(),
    "None".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold(),
  );

  println!(
    "In short, because {0} and {1} (where {1} can be any type) are different types, \
    the compiler won’t let us use an {0} value as if it were definitely a valid value. \n\
    For example, this code won’t compile, because it’s trying to add an {2} to an {3}: \n\n\
    {4}",
    "Option<T>".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "i8".bright_yellow().bold(),
    "Option<i8>".bright_yellow().bold(),
    "let x: i8 = 5; \n\
    let y: Option<i8> = Some(5); \n\
    let sum = x + y;
    ".bright_yellow().bold(),
  );

  println!(
    "See the error output under the \"The Option Enum section\": {0} \n\
    The error ouput shows that Rust doesn’t understand how to add an {1} and an {2}, \
    because they’re different types. \n\
    When we have a value of a type like {1} in Rust, the compiler will ensure that we \
    always have a valid value. \n\
    We can proceed confidently without having to check for null before using that value. \n\
    Only when we have an {2} (or whatever type of value in the angle bracket we’re working \
    with) do we have to worry about possibly not having a value, and the compiler will \
    make sure we handle that case before using the value.
  ",
    "https://doc.rust-lang.org/book/ch06-01-defining-an-enum.html#the-option-enum".cyan(),
    "i8".bright_yellow().bold(),
    "Option<i8>".bright_yellow().bold()
  );

  println!(
    "In other words, you have to convert an {0} to a {1} before you can perform {1} \
    operations with it. \n\
    Generally, this helps catch one of the most common issues with null: assuming that \
    something isn’t null when it actually is.
  ",
    "Option<T>".bright_yellow().bold(),
    "T".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    Eliminating the risk of incorrectly assuming a not-null value helps you be more \
    confident in your code. \n\
    In order to have a value that can possibly be null, you must explicitly opt in by \
    making the type of that value {1}. \n\
    Then, when you use that value, you are required to explicitly handle the case when \
    the value is null. \n\
    Everywhere that a value has a type that isn’t an {1}, you can safely assume that \
    the value isn’t null. \n\
    This was a deliberate design decision for Rust to limit null’s pervasiveness and \
    increase the safety of Rust code.
  ",
    "MAKING THE VALUE POSSIBLY NULL".bright_magenta().bold(),
    "Option<T>".bright_yellow().bold()
  );

  println!(
    "{4} \n\n\
    So how do you get the {0} value out of a {1} variant when you have a value of type \
    {2} so that you can use that value? \n\
    The {2} enum has a large number of methods that are useful in a variety of \
    situations; you can check them out in its documentation: {3}. \n\
    Becoming familiar with the methods on {2} will be extremely useful in your journey \
    with Rust.
  ",
    "T".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold(),
    "https://doc.rust-lang.org/std/option/enum.Option.html".cyan(),
    "USING THE T VALUE THAT THE SOME VARIANT HOLDS".bright_magenta().bold()
  );

  println!(
    "{} \n\n\
    In general, in order to use an {1} value, you want to have code that will \
    handle each variant. \n\
    You want some code that will run only when you have a {2} value, and this code \
    is allowed to use the inner {3}. \n\
    You want some other code to run only if you have a {4} value, and that code doesn’t \
    have a {3} value available. \n\
    The {5} expression is a control flow construct that does just this when used with \
    enums: It will run different code depending on which variant of the enum it has, and \
    that code can use the data inside the matching value.
  ",
    "THE MATCH EXPRESSION".bright_magenta().bold(),
    "Option<T>".bright_yellow().bold(),
    "Some(T)".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "None".bright_yellow().bold(),
    "match".bright_yellow().bold(),
  );
}