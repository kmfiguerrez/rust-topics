use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Chapter Introduction", ci_content),
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Objects Contain Data and Behavior", ocdab_content),
    chapter::SubHeader::new("Encapsulation That Hides Implementation Details", ethid_content),
    chapter::SubHeader::new("Inheritance as a Type System and as Code Sharing", iatsacs_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Chapter Introduction. Abbreviated as ci.
fn ci_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Chapter Introduction: Object-Oriented Programming Features");

  println!(
  "Object-oriented programming (OOP) is a way of modeling programs. Objects as a programmatic concept were introduced in the \
  programming language Simula in the 1960s.\n\
  Those objects influenced Alan Kay's programming architecture in which objects pass messages to each other.\n\
  To describe this architecture, he coined the term object-oriented programming in 1967.\n\
  Many competing definitions describe what OOP is, and by some of these definitions Rust is object oriented but by others it is not.\n\
  In this chapter, we'll explore certain characteristics that are commonly considered object oriented and how those \
  characteristics translate to idiomatic Rust.\n\
  We'll then show you how to implement an object-oriented design pattern in Rust and discuss the trade-offs of doing so versus \
  implementing a solution using some of Rust's strengths instead.
  ")
}

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Characteristics of Object-Oriented Languages");

  println!(
  "There is no consensus in the programming community about what features a language must have to be considered object oriented.\n\
  Rust is influenced by many programming paradigms, including OOP; for example, we explored the features that came from functional \
  programming in Chapter 13.\n\
  Arguably, OOP languages share certain common characteristics—namely, objects, encapsulation, and inheritance.\n\
  Let's look at what each of those characteristics means and whether Rust supports it.
  ")
}

// Header: Objects Contain Data and Behavior. Abbreviated as ocdab.
fn ocdab_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Objects Contain Data and Behavior");

  println!(
  "The book Design Patterns: Elements of Reusable Object-Oriented Software by Erich Gamma, Richard Helm, Ralph Johnson, and John \
  Vlissides (Addison-Wesley, 1994), colloquially referred to as The Gang of Four book, is a catalog of object-oriented design \
  patterns. It defines OOP in this way:\n\n\
  Object-oriented programs are made up of objects.\n\
  An {0} packages both data and the procedures that operate on that data.\n\
  The procedures are typically called {1} or {2}.\n\n\
  Using this definition, Rust is object oriented: Structs and enums have data, and {3} blocks provide methods on structs and enums.\n\
  Even though structs and enums with methods aren't called objects, they provide the same functionality, according to the \
  Gang of Four's definition of objects.
  ",
  "object".bold(),
  "methods".bold(),
  "operations".bold(),
  "impl".bright_yellow().bold(),
  )
}

// Header: Encapsulation That Hides Implementation Details. Abbreviated as ethid.
fn ethid_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Encapsulation That Hides Implementation Details");

  println!(
  "Another aspect commonly associated with OOP is the idea of {0}, which means that the implementation details of an object \
  aren't accessible to code using that object.\n\
  Therefore, the only way to interact with an object is through its public API; code using the object shouldn't be able to reach \
  into the object's internals and change data or behavior directly.\n\
  This enables the programmer to change and refactor an object's internals without needing to change the code that uses the object.\n\n\
  We discussed how to control encapsulation in Chapter 7: We can use the {1} keyword to decide which modules, types, functions, \
  and methods in our code should be public, and by default everything else is private.\n\
  For example, we can define a struct {2} that has a field containing a vector of i32 values.\n\
  The struct can also have a field that contains the average of the values in the vector, meaning the average doesn't have to be \
  computed on demand whenever anyone needs it.\n\
  In other words, {2} will cache the calculated average for us.\n\
  Listing 18-1 has the definition of the {2} struct.\n\n\
  See Listing 18-1: {3}, for code sample and complete reading.
  ",
  "encapsulation".italic().bold(),
  "pub".bright_yellow().bold(),
  "AveragedCollection".bright_yellow().bold(),
  "https://doc.rust-lang.org/book/ch18-01-what-is-oo.html#listing-18-1".bright_cyan(),
);

println!(
"{0}\n\n\
 Because we've encapsulated the implementation details of the struct {1}, we can easily change aspects, such as the data structure, \
in the future.\n\
For instance, we could use a {2} instead of a {3} for the {4} field.\n\
As long as the signatures of the {5}, {6}, and {7} public methods stayed the same, code using {1} wouldn't need to change.\n\
If we made list public instead, this wouldn't necessarily be the case: {2} and {3} have different methods for \
adding and removing items, so the external code would likely have to change if it were modifying {4} directly.
",
"Benefits of encapsulation".bright_magenta().bold(),
"AveragedCollection".bright_yellow().bold(),
"HashSet<i32>".bright_yellow().bold(),
"Vec<i32>".bright_yellow().bold(),
"list".bright_yellow().bold(),
"add".bright_yellow().bold(),
"remove".bright_yellow().bold(),
"average".bright_yellow().bold(),
);

println!(
"{0}\n\n\
If encapsulation is a required aspect for a language to be considered object oriented, then Rust meets that requirement.\n\
The option to use {1} or not for different parts of code enables encapsulation of implementation details.
",
"Rust meets OOP's encapsulation requirement".bright_magenta().bold(),
"pub".bright_yellow().bold(),
)
}

// Header: Inheritance as a Type System and as Code Sharing. Abbreviated as iatsacs.
fn iatsacs_content() {
  // let solid_disc = "\u{2022}";

  menu::subheader_title("Inheritance as a Type System and as Code Sharing");

  println!(
  "{0} is a mechanism whereby an object can inherit elements from another object's definition, thus gaining the parent object's \
  data and behavior without you having to define them again.\n\n\
  If a language must have inheritance to be object oriented, then Rust is not such a language.\n\
  There is no way to define a struct that inherits the parent struct's fields and method implementations without using a macro.\n\n\
  However, if you're used to having inheritance in your programming toolbox, you can use other solutions in Rust, depending on \
  your reason for reaching for inheritance in the first place.
  ",
  "Inheritance".italic().bold(),
  );

  println!(
  "{0}\n\n\
  You would choose inheritance for two main reasons.\n\
  One is for reuse of code: You can implement particular behavior for one type, and inheritance enables you to reuse that \
  implementation for a different type.\n\
  You can do this in a limited way in Rust code using default trait method implementations, which you saw in Listing 10-14 when we \
  added a default implementation of the {1} method on the {2} trait.\n\
  Any type implementing the {2} trait would have the {1} method available on it without any further code.\n\
  This is similar to a parent class having an implementation of a method and an inheriting child class also having the \
  implementation of the method.\n\
  We can also override the default implementation of the {1} method when we implement the {2} trait, which is similar to a child \
  class overriding the implementation of a method inherited from a parent class.\n\n\
  The other reason to use inheritance relates to the type system: to enable a child type to be used in the same places as the \
  parent type.\n\
  This is also called {3}, which means that you can substitute multiple objects for each other at runtime if they share certain \
  characteristics.
  ",
  "Reasons for inheritance".bright_magenta().bold(),
  "summarize".bright_yellow().bold(),
  "Summary".bright_yellow().bold(),
  "polymorphism".italic().bold(),
  );

  println!(
  "{0}\n\n\
  To many people, polymorphism is synonymous with inheritance.\n\
  But it's actually a more general concept that refers to code that can work with data of multiple types.\n\
  For inheritance, those types are generally subclasses.\n\n\
  Rust instead uses generics to abstract over different possible types and trait bounds to impose constraints on what those types \
  must provide.\n\
  This is sometimes called {1}.
  ",
  "Polymorphism".bright_magenta().bold(),
  "bounded parametric polymorphism".italic().bold()
  );

  println!(
  "{0}\n\n\
  Rust has chosen a different set of trade-offs by not offering inheritance.\n\
  Inheritance is often at risk of sharing more code than necessary.\n\
  Subclasses shouldn't always share all characteristics of their parent class but will do so with inheritance.\n\
  This can make a program's design less flexible.\n\
  It also introduces the possibility of calling methods on subclasses that don't make sense or that cause errors because the \
  methods don't apply to the subclass.\n\
  In addition, some languages will only allow single inheritance (meaning a subclass can only inherit from one class), further \
  restricting the flexibility of a program's design.\n\n\
  For these reasons, Rust takes the different approach of using trait objects instead of inheritance to achieve polymorphism \
  at runtime.\n\
  ",
  "Rust uses trait objects instead of inheritance".bright_magenta().bold(),
  );  
}


