use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Section introduction", si_content),
    chapter::SubHeader::new("Defining a Trait", dt_content),
    chapter::SubHeader::new("Implementing a Trait on a Type", iatoat_content),
    chapter::SubHeader::new("Using Default Implementations", udi_content),
    chapter::SubHeader::new("Using Traits as Parameters", utap_content),
    chapter::SubHeader::new("Trait Bounds Syntax", tbs_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Section introduction");
  
  println!(
    "A {0} defines the functionality a particular type has and can share with other types.\n\
    We can use traits to define shared behavior in an abstract way.\n\
    We can use {1} to specify that a generic type can be any type that has \
    certain behavior.
  ",
    "trait".italic(),
    "trait bounds".italic()
  );

  println!(
    "{0} Traits are similar to a feature often called {1} in other languages, \
    although with some differences.\n\n\
    In languages like Java or C#, an Interface is a \"contract.\" It's a list of methods \
    that a class promises to have.\n\n\
    {solid_disc} {2}: Think of a wall outlet. Anything you plug into it (a lamp, a toaster, a vacuum) must have a specific plug shape. \
    The outlet doesn't care what the device is; it just cares that the device \"implements\" the plug interface.\n\
    {solid_disc} {3}: A Trait is essentially this contract. It defines the \"shape\" of the functionality.
  ",
    "Note:".bright_white().bold(),
    "interfaces".italic(),
    "The Analogdy".bright_white().bold(),
    "In Rust".bright_white().bold()
  );
}

// Header: Defining a Trait. Abbreviated as dt
fn dt_content() {
  menu::subheader_title("Defining a Trait");

  println!(
    "A type's behavior consists of the methods we can call on that type.\n\
    Different types share the same behavior if we can call the same methods on \
    all of those types.\n\
    Trait definitions are a way to group method signatures together to define a \
    set of behaviors necessary to accomplish some purpose.
  ");

  println!(
    "For example, let's say we have multiple structs that hold various kinds and amounts \
    of text: a {0} struct that holds a news story filed in a particular location \
    and a {1} that can have, at most, 280 characters along with metadata that \
    indicates whether it was a new post, a repost, or a reply to another post.\n\n\
    We want to make a media aggregator library crate named {2} that can display \
    summaries of data that might be stored in a {0} or {1} instance.\n\
    To do this, we need a summary from each type, and we'll request that summary by calling \
    a {3} method on an instance.\n\
    Listing 10-12 shows the definition of a public Summary trait that expresses this behavior.\n\n\
    See {4}, for code sample.
  ",
    "NewsArticle".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "aggregator".bright_yellow().bold(),
    "summarize".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-02-traits.html#listing-10-12".cyan(),

  );

  println!(
    "{0}\n\n\
    Here, we declare a trait using the {1} keyword and then the trait's name, which is \
    {2} in this case.\n\
    We also declare the trait as {3} so that crates depending on this crate can make use \
    of this trait too.\n\n\
    Inside the curly brackets, we declare the method signatures that describe the behaviors \
    of the types that implement this trait, which in this case is {4}\n\
    After the method signature, instead of providing an implementation within curly brackets, \
    we use a semicolon.\n\
    Each type implementing this trait must provide its own custom behavior for the body of \
    the method.\n\
    The compiler will enforce that any type that has the {2} trait will have the method \
    {5} defined with this signature exactly.\n\n\
    A trait can have multiple methods in its body: The method signatures are listed one per \
    line, and each line ends in a semicolon.
  ",
    "pub trait Summary {\n\
    \u{2003}\u{2003}fn summarize(&self) -> String;\n\
    }".bright_yellow().bold(),
    "trait".bright_yellow().bold(),
    "Summary".bright_yellow().bold(),
    "pub".bright_yellow().bold(),
    "fn summarize(&self) -> String".bright_yellow().bold(),
    "summarize".bright_yellow().bold()
  );
}

// Header: Implementing a Trait on a Type. Abbreviated as iatoat.
fn iatoat_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Implementing a Trait on a Type");

  println!(
    "Once we've defined a trait, we can implement it on a particular type.\n\
    Listing 10-13 shows an implementation of the {0} trait on the {1} struct \
    that uses the headline, the author, and the location to create the return value of {2}.\n\
    For the {3} struct, we define {2} as the username followed by the entire text \
    of the post, assuming that the post content is already limited to 280 characters.\n\n\
    See {4}, for code sample.
  ",
    "Summary".bright_yellow().bold(),
    "NewsArticle".bright_yellow().bold(),
    "summarize".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-02-traits.html#listing-10-13".cyan(),
  );

  println!(
    "{}\n\n\
    Implementing a trait on a type is similar to implementing regular methods.\n\
    The difference is that after {1}, we put the trait name we want to implement, \
    then use the {2} keyword, and then specify the name of the type we want to implement \
    the trait for.\n\
    Within the {1} block, we put the method signatures that the trait definition has defined.\n\
    Instead of adding a semicolon after each signature, we use curly brackets and fill in the \
    method body with the specific behavior that we want the methods of the trait to have for \
    the particular type.\n\n\
    i.e:\n\
    {3}
  ",
    "SYNTAX".bright_magenta().bold(),
    "impl".bright_yellow().bold(),
    "for".bright_yellow().bold(),
    "impl <trait_name> for <type_name> {\n\
    \u{2003}\u{2003}// method implementations go here\n\
    }".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    Now that the library has implemented the {1} trait on {2} and {3}, \
    users of the crate can call the trait methods on instances of {2} and {3} \
    in the same way we call regular methods.\n\
    {4}
  ",
    "TRAITS MUST BE BROUGHT INTO SCOPE".bright_magenta().bold(),
    "Summary".bright_yellow().bold(),
    "NewsArticle".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "The only difference is that the user must bring the trait into scope as \
    well as the types.".bright_white().bold()
  );

  println!(
    "{0}\n\n\
    Other crates that depend on the {1} crate can also bring the {2} trait into \
    scope to implement {2} on their own types.\n\
    One restriction to note is that we can implement a trait on a type only if either the \
    trait or the type, or both, are local to our crate.\n\
    For example, we can implement standard library traits like {3} on a custom type like \
    {4} as part of our {5} crate functionality because the type {4} is \
    local to our {5} crate.\n\
    We can also implement Summary on {6} in our {5} crate because the trait {2} is local \
    to our {5} crate.\n\n\
    But we can't implement external traits on external types.\n\
    For example, we can't implement the {3} trait on {6} within our {5} crate, \
    because {3} and {6} are both defined in the standard library and aren't local to \
    our {5} crate.
  ",
    "RESTRICTIONS ON TRAITS IMPLEMENTATIONS".bright_magenta().bold(),
    "aggregator".bright_yellow().bold(),
    "Summary".bright_yellow().bold(),
    "Display".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "aggregator".bright_yellow().bold(),
    "Vec<T>".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    This restriction is part of a property called {1}, and more specifically the \
    {2}, so named because the parent type is not present.\n\
    This rule ensures that other people's code can't break your code and vice versa.\n\
    {3}.
  ",
    "ORPHAN RULE".bright_magenta().bold(),
    "coherence".italic().bold(),
    "orphan rule".italic().bold(),
    "Without the rule, two crates could implement the same trait for the same type, and Rust \
    wouldn't know which implementation to use".red()
  );

  println!(
    "{}\n\n\
    {solid_disc} In order to use a trait on a type, it must be brought into scope.\n\
    {solid_disc} We can implement a trait on a type only if either the \
    trait or the type, or both, are local to our crate.\n\
  ",
    "REMEMBER".bright_white().bold()
  );
}

// Header: Using Default Implementations. Abrreviated as udi.
fn udi_content() {
  menu::subheader_title("Using Default Implementations");

  println!(
    "Sometimes it's useful to have default behavior for some or all of the methods in a trait \
    instead of requiring implementations for all methods on every type.\n\
    Then, as we implement the trait on a particular type, we can keep or override each \
    method's default behavior.
  ");

  println!(
    "{}\n\n\
    In Listing 10-14, we specify a default string for the {1} method of the {2} \
    trait instead of only defining the method signature.\n\n\
    See {3}, for code sample.\n\n\
    To use a default implementation to summarize instances of {4}, we specify an empty \
    {5} block with {6}.\n\n\
    Even though we're no longer defining the {1} method on {4} directly, we've \
    provided a default implementation and specified that {4} implements the {2} \
    trait.
  ",
    "SYNTAX".bright_magenta().bold(),
    "summarize".bright_yellow().bold(),
    "Summary".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-02-traits.html#listing-10-14".cyan(),
    "NewsArticle".bright_yellow().bold(),
    "impl".bright_yellow().bold(),
    "impl Summary for NewsArticle {}".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    Creating a default implementation doesn't require us to change anything about the \
    implementation of {1} on {2} in Listing 10-13: {3}.\n\
    The reason is that the syntax for overriding a default implementation is the same as the \
    syntax for implementing a trait method that doesn't have a default implementation.
  ",
    "OVERRIDING A DEFAULT IMPLEMENTATION".bright_magenta().bold(),
    "Summary".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-02-traits.html#listing-10-13".cyan()
  );

  println!(
    "{}\n\n\
    Default implementations can call other methods in the same trait, even if those other \
    methods don't have a default implementation.\n\
    In this way, a trait can provide a lot of useful functionality and only require \
    implementors to specify a small part of it.\n\
    For example, we could define the {1} trait to have a {2} method whose \
    implementation is required, and then define a {3} method that has a default \
    implementation that calls the {2} method:\n\n\
    {4}\n\n\
    To use this version of {1}, we only need to define {2} when we implement \
    the trait on a type:\n\n\
    {5}\n\n\
    After we define {2}, we can call {3} on instances of the {6} \
    struct, and the default implementation of {3} will call the definition of \
    {2} that we've provided.\n\
    Because we've implemented {2}, the {1} trait has given us the behavior \
    of the {3} method without requiring us to write any more code.\n\n\
    {7}.
  ",
    "CALLING OTHER METHODS IN THE SAME TRAIT".bright_magenta().bold(),
    "Summary".bright_yellow().bold(),
    "summarize_author".bright_yellow().bold(),
    "summarize".bright_yellow().bold(),
    "pub trait Summary {\n\
    \u{2003}\u{2003}fn summarize_author(&self) -> String;\n\n\
    \u{2003}\u{2003}fn summarize(&self) -> String {\n\
    \u{2003}\u{2003}\u{2003}\u{2003}format!(\"(Read more from {}...)\", self.summarize_author())\n\
    \u{2003}\u{2003}}\n\
    }".bright_yellow().bold(),
    "impl Summary for SocialPost {\n\
    \u{2003}\u{2003}fn summarize_author(&self) -> String {\n\
    \u{2003}\u{2003}\u{2003}\u{2003}format!(\"@{}\", self.username)\n\
    \u{2003}\u{2003}}\n\
    }".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "Note that it isn't possible to call the default implementation from an overriding \
    implementation of that same method".red()
  );
}

// Header: Using Traits as Parameters. Abbreviated as utap.
fn utap_content() {
  menu::subheader_title("Using Traits as Parameters");

  println!(
    "Let's explore how to use traits to define functions that accept many different types.\n\
    We'll use the {0} trait we implemented on the {1} and {2} types in \
    Listing 10-13 to define a {3} function that calls the {4} method on its {5} \
    parameter, which is of some type that implements the {0} trait.\n\
    To do this, we use the {6} syntax, like this:\n\n\
    {7}\n\n\
    Instead of a concrete type for the {5} parameter, we specify the {6} keyword and the \
    trait name.\n\
    This parameter accepts any type that implements the specified trait.\n\
    In the body of {3}, we can call any methods on {5} that come from the {0} trait, \
    such as {4}.\n\
    We can call notify and pass in any instance of {1} or {2}.\n\
    Code that calls the function with any other type, such as a {8} or an {9}, \
    won't compile, because those types don't implement {0}.
  ",
    "Summary".bright_yellow().bold(),
    "NewsArticle".bright_yellow().bold(),
    "SocialPost".bright_yellow().bold(),
    "notify".bright_yellow().bold(),
    "summarize".bright_yellow().bold(),
    "item".bright_yellow().bold(),
    "impl Trait".bright_yellow().bold(),
    "pub fn notify(item: &impl Summary) {\n\
    \u{2003}\u{2003}println!(\"Breaking news! {}\", item.summarize());\n\
    }".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "i32".bright_yellow().bold()

  )
}

// Header: Trait Bounds Syntax. Abbreviated as tbs.
fn tbs_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Trait Bounds Syntax");

  println!(
    "The {} syntax works for straightforward cases but is actually syntax sugar for a \
    longer form known as a trait bound; it looks like this:\n\n\
    {1}\n\n\
    This longer form is equivalent to {0} syntax but is more verbose.\n\
    We place trait bounds with the declaration of the generic type parameter after a colon and \
    inside angle brackets.\n\n\
    The {0} syntax is convenient and makes for more concise code in simple cases, \
    while the fuller trait bound syntax can express more complexity in other cases.\n\
    For example, we can have two parameters that implement {2}.\n\
    Doing so with the {0} syntax looks like this:\n\n\
    {3}\n\n\
    Using {0} is appropriate if we want this function to allow {4} and {5} to \
    have different types (as long as both types implement {2}).\n\
    If we want to force both parameters to have the same type, however, we must use a trait \
    bound, like this:\n\n\
    {6}\n\n\
    The generic type {7} specified as the type of the {4} and {5} parameters constrains the \
    function such that the concrete type of the value passed as an argument for {4} and {5} \
    must be the same.
  ",
    "impl Trait".bright_yellow().bold(),
    "pub fn notify<T: Summary>(item: &T) {\n\
    \u{2003}\u{2003}println!(\"Breaking news! {}\", item.summarize());\n\
    }".bright_yellow().bold(),
    "Summary".bright_yellow().bold(),
    "pub fn notify(item1: &impl Summary, item2: &impl Summary) {".bright_yellow().bold(),
    "item1".bright_yellow().bold(),
    "item2".bright_yellow().bold(),
    "pub fn notify<T: Summary>(item1: &T, item2: &T) {".bright_yellow().bold(),
    "T".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    {solid_disc} {1} syntax is convenient when we have only one parameter that uses a particular trait \
    and when we have multiple parameters of different types that all implement the same trait.\n\
    {solid_disc} Trait bounds are more appropriate when we need to specify that multiple \
    parameters must be of the same type.\n\
  ",
    "REMEMBER".bright_white().bold(),
    "impl Trait".bright_yellow().bold(),
  );
}