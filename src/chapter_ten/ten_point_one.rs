use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 6];
  subheaders = [
    chapter::SubHeader::new("Introduction", introduction_content),
    chapter::SubHeader::new("Generics in Function Definitions", gifd_content),
    chapter::SubHeader::new("Generics in Struct Definitions", gisd_content),
    chapter::SubHeader::new("Generics in Enum Definitions", gied_content),
    chapter::SubHeader::new("Generics in Method Definitions", gimd_content),
    chapter::SubHeader::new("Performance of Code Using Generics", pcug_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Introduction. Abbreviated as i.
fn introduction_content() {
  let solid_disc = "\u{2022}";

  // header title.
  menu::subheader_title("Introduction");


  // header content.
  println!(
    "Every programming language has tools for effectively handling the duplication of concepts. \
    In Rust, one such tool is {}: abstract stand-ins for concrete types or other properties. \n\
    We can express the behavior of generics or how they relate to other generics without \
    knowing what will be in their place when compiling and running the code.
  ",
    "generics".italic()
  );

  println!(
    "Functions can take parameters of some generic type, instead of a concrete type like \
    {0} or {1}, in the same way they take parameters with unknown values to run the same \
    code on multiple concrete values.
  ",
    "i32".bright_yellow().bold(),
    "String".bright_yellow().bold(),
  );

  println!(
    "You can combine traits with generic types to constrain a generic type to accept only \
    those types that have a particular behavior, as opposed to just any type. \n\n\
    We'll also use generics with {}: a variety of generics that give the compiler \
    information about how references relate to each other. \n\
    Lifetimes allow us to give the compiler enough information about borrowed values so \
    that it can ensure that references will be valid in more situations than it could without \
    our help.
  ",
    "lifetimes".italic()
  );

  println!(
    "{}\n\n\
    {solid_disc} Generics allow us to replace specific types with a placeholder that represents \
    multiple types to remove code duplication.
  ",
    "REMEMBER".bright_white().bold()
  )
}

// Header: Generics in Function Definitions. Abbreviated as gifd.
fn gifd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Generics in Function Definitions");

  println!(
    "When defining a function that uses generics, we place the generics in the signature \
    of the function where we would usually specify the data types of the parameters and \
    return value. \n\
    Doing so makes our code more flexible and provides more functionality to callers of \
    our function while preventing code duplication.\n\n\
    See Listing 10-4: {}, for code samples
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-4".cyan()
  );

  println!(
    "In Listing 10-4, the {0} and {1} function have the same code, \
    so let's eliminate the duplication by introducing a {2} in a \
    single function.
  ",
    "largest_i32".bright_yellow().bold(),
    "largest_char".bright_yellow().bold(),
    "generic type parameter".italic()
  );

  println!(
    "{}\n\n\
    To parameterize the types in a new single function, we need to name the {1}, \
    just as we do for the {2} to a function. \n\
    You can use any identifier as a type parameter name. \n\
    But we'll use {3} because, by convention, type parameter names in Rust are short, often just \
    one letter, and Rust's type-naming convention is UpperCamelCase. Short for {3}, {4} is the \
    default choice of most Rust programmers.
  ",
    "NAMING GENERIC TYPE PARAMETERS".bright_magenta().bold(),
    "type parameter".italic(),
    "value parameters".italic(),
    "T".bright_yellow().bold(),
    "type".italic()
  );

  println!(
    "{}\n\n\
    When we use a parameter in the body of the function, we have to declare the parameter \
    name in the signature so that the compiler knows what that name means.\n\
    Similarly, when we use a type parameter name in a function signature, we have to declare \
    the type parameter name before we use it.\n\
    To define the generic {1} function, we place type name declarations inside angle \
    brackets, {2}, between the name of the function and the parameter list, like this:\n\n\
    {3}\n\n\
    We read this definition as “The function {1} is generic over some type {4}.”\n\
    This function has one parameter named {5}, which is a slice of values of type {4}.\n\
    The {1} function will return a reference to a value of the same type {4}.
  ",
    "DECLARING GENERIC TYPE PARAMETERS".bright_magenta().bold(),
    "largest".bright_yellow().bold(),
    "<>".bright_yellow().bold(),
    "fn largest<T>(list: &[T]) -> &T {".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "list".bright_yellow().bold(),
  );

  println!(
    "Listing 10-5: {}, shows the combined {1} function definition using the generic data \
    type in its signature.\n\
    The listing also shows how we can call the function with either a slice of {2} values \
    or {3} values.\n\
    {6}. The error below listing 10-5 states that the body \
    of the {1} function won't work for all possible types that {4} could be.\n\
    Because we want to compare values of type {4} in the body, we can only use types whose \
    values can be ordered.\n\
    To enable comparisons, the standard library has the {5} trait that you \
    can implement on types.\n\
    To fix Listing 10-5, we can follow the help text's suggestion and restrict the types \
    valid for {4} to only those that implement {5}.
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-5".cyan(),
    "largest".bright_yellow().bold(),
    "i32".bright_yellow().bold(),
    "char".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "std::cmp::PartialOrd".bright_yellow().bold(),
    "Note that this code won't compile yet".red()
  );

  println!(
    "{}\n\n\
    {solid_disc} By convention, type parameter names in Rust are short, often just one letter. \
    Short for type, {1} is the default choice of most Rust programmers.\n\
    {solid_disc} Rust’s type-naming convention is UpperCamelCase.\n\
    {solid_disc} To use a generic type parameter name in a function signature, we have to \
    declare the type parameter name inside angle brackets, {2}, between the name of the function and the parameter list.

  ",
    "REMEMBER".bright_white().bold(),
    "T".bright_yellow().bold(),
    "<>".bright_yellow().bold()
  )
}

// Header: Generics in Struct Definitions. Abbreviated as gisd.
fn gisd_content() {
  menu::subheader_title("Generics in Struct Definitions");

  println!(
    "We can also define structs to use a generic type parameter in one or more fields using \
    the {0} syntax. \n\
    Listing 10-6 defines a {1} struct to hold {2} and {3} coordinate values of any type. \n\n\
    See Listing 10-6: {4}, for code samples
  ",
    "<>".bright_yellow().bold(),
    "Point<T>".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-6".cyan()
  );

  println!(
    "{}\n\n\
    The syntax for using generics in struct definitions is similar to that used in function definitions.\n\
    First, we declare the name of the type parameter inside angle brackets just after the name of the struct.\n\
    Then, we use the generic type in the struct definition where we would otherwise specify concrete data types.
  ",
    "SYNTAX".bright_magenta().bold()
  );

  println!(
    "{}\n\n\
    Note that because we've used only one generic type to define {1}, this definition \
    says that the {1} struct is generic over some type {2}, and the fields {3} and {4} are \
    both that same type, whatever that type may be.\n\n\
    If we create an instance of a {1} that has values of different types, as in Listing 10-7, \
    {6}.\n\n\
    See Listing 10-7: {5}, for code samples
  ",
    "WHAT DOES THE SYNTAX SAY?".bright_magenta().bold(),
    "Point<T>".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-7".cyan(),
    "our code won't compile".red()
  );

  println!(
    "{}\n\n\
    In the code sample in listing 10-7, when we assign the integer value {1} to {2}, we let the \
    compiler know that the generic type {3} will be an integer for this instance of {4}.\n\
    Then, when we specify {5} for {6}, which we've defined to have the same type as {2}, {7}.
  ",
    "THE FIRST ASSIGNMENT DETERMINES THE TYPE".bright_magenta().bold(),
    "5".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "Point<T>".bright_yellow().bold(),
    "4.0".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "we'll get a type mismatch error".red()
  );

  println!(
    "{}\n\n\
    To define a struct that can hold fields of different types, we can use multiple \
    generic type parameters.\n\
    Listing 10-8 shows how to define a {1} struct that uses two generic type parameters, \
    {2} and {3}, to allow the {4} and {5} fields to have different types.\n\n\
    See Listing 10-8: {6}, for code samples
  ",
    "MULTIPLE GENERIC TYPE PARAMETERS".bright_magenta().bold(),
    "Point<T, U>".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "U".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-8".cyan()
  );

  println!(
    "{}\n\n\
    Now all the instances of {1} shown in listing 10-8 are allowed!\n\
    You can use as many generic type parameters in a definition as you want, but {2}.\n\
    If you're finding you need lots of generic types in your code, it could indicate that\
    your code needs restructuring into smaller pieces.
  ",
    "CAUTION TO USING MULTIPLE GENERICS".bright_magenta().bold(),
    "Point".bright_yellow().bold(),
    "using more than a few makes your code hard to read".red()
  )
}

// Header: Generics in Enum Definitions. Abbreviated as gied.
fn gied_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Generics in Enum Definitions");

  println!(
    "We can define enums to hold generic data types in their variants.\n\
    For example, look at the {0} enum in the standard library, which is defined as follows:\n\n\
    {1}\n\n\
    This definition should now make more sense to you.\n\
    As you can see, the {0} enum is generic over type {2} and has two variants: \
    {3}, which holds one value of type {2}, and a {4} variant that doesn't hold any value.\n\
    By using the {0} enum, we can express the abstract concept of an optional value, \
    and because {0} is generic, we can use this abstraction no matter what the type \
    of the optional value is.
  ",
    "Option<T>".bright_yellow().bold(),
    "enum Option<T> {\n\
    \u{2003}\u{2003}None,\n\
    \u{2003}\u{2003}Some(T),\n\
    }".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "Some".bright_yellow().bold(),
    "None".bright_yellow().bold()
  );

  println!(
    "Enums can use multiple generic types as well.\n\
    For example, the {0} enum in the standard library is defined as follows:\n\n\
    {1}\n\n\
    The {0} enum is generic over two types, {2} and {3}, and has two variants: \
    {4}, which holds a value of type {2}, and {5}, which holds a value of type {3}.\n\
    This definition makes it convenient to use the {0} enum anywhere we have an operation \
    that might succeed (return a value of some type {2}) or fail (return an error of some type {3}).
  ",
    "Result".bright_yellow().bold(),
    "enum Result<T, E> {\n\
    \u{2003}\u{2003}Ok(T),\n\
    \u{2003}\u{2003}Err(E),\n\
    }".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "E".bright_yellow().bold(),
    "Ok".bright_yellow().bold(),
    "Err".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    {solid_disc} We can define enums to hold generic data types in their variants.\n\
    {solid_disc} When you recognize situations in your code with multiple struct or enum \
    definitions that differ only in the types of the values they hold, you can avoid \
    duplication by using generic types instead.
  ",
    "REMEMBER".bright_white().bold()
  )

}

// Header: Generics in Method Definitions. Abbreviated as gimd.
fn gimd_content() {
  menu::subheader_title("Generics in Method Definitions");

  println!(
    "We can implement methods on structs and enums and use generic types in their definitions too.\n\
    See Listing 10-9: {}, for code samples\n\n\
    In Listing 10-9, we've defined a method named {1} on {2} that returns a reference to the data in the field {1}. \n\n\
    Note that we have to declare {3} just after {4} so that we can use {3} to specify that \
    we're implementing methods on the type {2}.\n\
    By declaring {3} as a generic type after {4}, Rust can identify that the type in the \
    angle brackets in {5} is a generic type rather than a concrete type. \n\
    We could have chosen a different name for this generic parameter than the generic \
    parameter declared in the struct definition, but using the same name is conventional.\n\
    If you write a method within an {4} that declares a generic type, that method will be \
    defined on any instance of the type, no matter what concrete type ends up substituting \
    for the generic type.
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-9".cyan(),
    "x".bright_yellow().bold(),
    "Point<T>".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "impl".bright_yellow().bold(),
    "Point".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    We can also specify constraints on generic types when defining methods on the type.\n\
    We could, for example, implement methods only on {1} instances rather than on \
    {2} instances with any generic type.\n\
    To do so, we specify the concrete type in the impl block rather than the generic type.\n\
    In Listing 10-10, we use the concrete type {3}, meaning we don't declare any types after {4}.\n\n\
    See Listing 10-10: {7}, for code samples.\n\n\
    This code means the type {1} will have a {5} method; \
    other instances of {2} where {6} is not of type {3} will not have this method defined.
  ",
    "SPECIFYING CONSTRAINTS ON GENERIC TYPES IN METHODS".bright_magenta().bold(),
    "Point<f32>".bright_yellow().bold(),
    "Point<T>".bright_yellow().bold(),
    "f32".bright_yellow().bold(),
    "impl".bright_yellow().bold(),
    "distance_from_origin".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-10".cyan()
  );

  println!(
    "{}\n\n\
    Generic type parameters in a struct definition aren't always the same as those you use \
    in that same struct's method signatures.\n\
    Listing 10-11 uses the generic types {3} and {4} for the {8} struct and {5} and {6} for \
    the {7} method signature to make the example clearer.\n\n\
    See Listing 10-11: {1}, for code samples.\n\n\
    The purpose of example in Listing 10-11 is to demonstrate a situation in which some generic parameters \
    are declared with {2} and some are declared with the method definition. \n\
    Here, the generic parameters {3} and {4} are declared after {2} because they go with the struct definition.\n\
    The generic parameters {5} and {6} are declared after {7} because they're only relevant to the method.
  ",
    "GENERICS DECLARED IN A STRUCT VS IN METHOD DEFINITIONS".bright_magenta().bold(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-11".cyan(),
    "impl".bright_yellow().bold(),
    "X1".bright_yellow().bold(),
    "Y1".bright_yellow().bold(),
    "X2".bright_yellow().bold(),
    "Y2".bright_yellow().bold(),
    "fn mixup".bright_yellow().bold(),
    "Point".bright_yellow().bold()
  )

}

// Header: Performance of Code Using Generics. Abbreviated as pcug.
fn pcug_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Performance of Code Using Generics");

  println!(
    "You might be wondering whether there is a runtime cost when using generic type parameters. \n\
    The good news is that using generic types won't make your program run any slower than it would with concrete types.
  ");

  println!(
    "Rust accomplishes this by performing monomorphization of the code using generics at compile time.\n\
    {} is the process of turning generic code into specific code by filling in the concrete types that are used when compiled. \n\
    In this process, the compiler does the opposite of the steps we used to create the generic function in Listing 10-5: {}: \
    The compiler looks at all the places where generic code is called and generates code for the concrete types the generic code is called with.
  ",
    "Monomorphization".italic(),
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-5".cyan()
  );

  println!(
    "Let's look at how this works by using the standard library's generic {0} enum:\n\n\
    {1}\n\n\
    When Rust compiles this code, it performs monomorphization. \n\
    During that process, the compiler reads the values that have been used in {0} instances and identifies two kinds of {0}:\n\
    {solid_disc} One is {2}\n\
    {solid_disc} the other is {3}\n\n\
    As such, it expands the generic definition of {0} into two definitions specialized to {2} and {3}, thereby replacing the generic \
    definition with the specific ones.
  ",
    "Option<T>".bright_yellow().bold(),
    "let integer = Some(5);\n\
    let float = Some(5.0);".bright_yellow().bold(),
    "i32".bright_yellow().bold(),
    "f64".bright_yellow().bold()
  );

  println!(
    "The monomorphized version of the code looks similar to the following (the compiler uses different names than what we're using here for illustration)::\n\n\
    {0}\n\n\
    The generic {1} is replaced with the specific definitions created by the compiler. \n\
    Because Rust compiles generic code into code that specifies the type in each instance, we pay no runtime cost for using generics. \n\
    When the code runs, it performs just as it would if we had duplicated each definition by hand. \
    The process of monomorphization makes Rust's generics extremely efficient at runtime.
  ",
    "enum Option_i32 {\n\
    \u{2003}\u{2003}Some(i32),\n\
    \u{2003}\u{2003}None,\n\
    }\n\n\
    enum Option_f64 {\n\
    \u{2003}\u{2003}Some(f64),\n\
    \u{2003}\u{2003}None,\n\
    }\n\n\
    fn main() {\n\
    \u{2003}\u{2003}let integer = Option_i32::Some(5);\n\
    \u{2003}\u{2003}let float = Option_f64::Some(5.0);\n\
    }".bright_yellow().bold(),
    "Option<T>".bright_yellow().bold()
  );
}