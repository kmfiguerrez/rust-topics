use owo_colors::OwoColorize;
use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 12];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Dangling References", dr_content),
    chapter::SubHeader::new("The Borrow Checker", tbc_content),
    chapter::SubHeader::new("Generic Lifetimes in Functions", glif_content),
    chapter::SubHeader::new("Lifetime Annotation Syntax", las_content),
    chapter::SubHeader::new("Generic Lifetimes In Function Signatures", glifs_content),
    chapter::SubHeader::new("Relationships", rel_content),
    chapter::SubHeader::new("Generic lifetimes In Struct Definitions", glisd_content),
    chapter::SubHeader::new("Lifetime Elision", le_content),
    chapter::SubHeader::new("Generics In Method Definitions", gimd_content),
    chapter::SubHeader::new("The Static Lifetime", tsl_content),
    chapter::SubHeader::new("Generic Type Parameters, Trait Bounds, and Lifetimes", gtptbal_content),
  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}


// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
  menu::subheader_title("Section Introduction");

  println!(
    "Lifetimes are another kind of generic that we've already been using.\n\
    Rather than ensuring that a type has the behavior we want, lifetimes ensure that \
    references are valid as long as we need them to be.\n\n\
    {}.\n\
    Most of the time, lifetimes are implicit and inferred, just like most of the time, \
    types are inferred.\n\
    We are only required to annotate types when multiple types are possible.\n\
    In a similar way, we must annotate lifetimes when the lifetimes of references could be \
    related in a few different ways.\n\
    Rust requires us to annotate the relationships using generic lifetime parameters to ensure \
    that the actual references used at runtime will definitely be valid.\n\n\
    Annotating lifetimes is not even a concept most other programming languages have.
  ",
    "Every reference in Rust has a lifetime, which is the scope for which that reference \
    is valid".bright_white().bold()
  );
}

// Header: Dangling References. Abbreviated as dr.
fn dr_content() {
  menu::subheader_title("Dangling References");

  println!(
    "The main aim of lifetimes is to prevent dangling references, which, if they were allowed \
    to exist, would cause a program to reference data other than the data it's intended to \
    reference.\n\
    Consider the program in Listing 10-16, which has an outer scope and an inner scope.\n\n\
    See: {0}, for code sample.\n\n\
    The outer scope declares a variable named {1} with no initial value, and the inner scope \
    declares a variable named {2} with the initial value of {3}.\n\
    Inside the inner scope, we attempt to set the value of {1} as a reference to {2}.\n\
    Then, the inner scope ends, and we attempt to print the value in {1}.\n\
    This code won't compile, because the value that {1} is referring to has gone out of scope before we try to use \
    it.\n\n\
    See: {4} for the error sample.\n\n\
    The error message says that the variable {2} “does not live long enough.”\n\
    The reason is that {2} will be out of scope when the inner scope ends on line 7.\n\
    But {1} is still valid for the outer scope; because its scope is larger, we say that it “lives longer.”\n\
    If Rust allowed this code to work, {1} would be referencing memory that was deallocated \
    when {2} went out of scope, and anything we tried to do with {1} wouldn't work correctly.\n\
    Rust uses borrow checker to determine at compile time whether any references are valid.
  ",
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-16".cyan(),
    "r".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "5".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#dangling-references".cyan()
  );

  println!(
    "{}: The examples in Listings 10-16, 10-17, and 10-23 declare variables without giving \
    them an initial value, so the variable name exists in the outer scope.\n\
    At first glance, this might appear to be in conflict with Rust having no null values.\n\
    However, if we try to use a variable before giving it a value, we'll get a compile-time \
    error, which shows that indeed Rust does not allow null values.
  ",
    "NOTE".bright_white().bold()
  );

}

// Header: The Borrow Checker. Abbreviated as tbc.
fn tbc_content() {
  menu::subheader_title("The Borrow Checker");

  println!(
    "The Rust compiler has a borrow checker that compares scopes to determine whether all \
    borrows are valid.\n\n\
    See: Listing 10-17: {0} for the borrow checker in action.\n\n\
    Here, we've annotated the lifetime of {1} with {2} and the lifetime of {3} with {4}.\n\
    As you can see, the inner {4} block is much smaller than the outer {2} lifetime block.\n\
    At compile time, Rust compares the size of the two lifetimes and sees that {1} has a \
    lifetime of {2} but that it refers to memory with a lifetime of {4}.\n\
    The program is rejected because {4} is shorter than {2}: The subject of the reference \
    doesn't live as long as the reference.\n\n\
    Listing 10-18 fixes the code so that it doesn't have a dangling reference and it compiles \
    without any errors.\n\n\
    See Listing 10-18: {5} for the fixed code sample.\n\n\
    Here, {3} has the lifetime {4}, which in this case is larger than {2}.\n\
    This means {1} can reference {3} because Rust knows that the reference in {1} will always \
    be valid while {3} is valid.
  ",
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-17".cyan(),
    "r".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "'b".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-18".cyan()
  );

}

// Header: Generic Lifetimes in Functions. Abbreviated as glif.
fn glif_content() {
  menu::subheader_title("Generic Lifetimes in Functions");

  println!(
    "Let's look at a situation where we would need to use generic lifetimes in a function.\n\n\
    See Listing 10-19: {0}, and \n\
    Listing 10-20: {1} for the code samples.\n\n\
    When you run the code in Listing 10-19, you'll get an error because Rust can't \
    determine whether the reference being returned refers to {2} or {3}.\n\
    Actually, we don't know either, because the {4} block in the body of this function returns \
    a reference to {2} and the {5} block returns a reference to {3}!\n\n\
    When we're defining this function, we don't know the concrete values that will be passed \
    into this function, so we don't know whether the {4} case or the {5} case will execute.\n\
    We also don't know the concrete lifetimes of the references that will be passed in, so we \
    can't look at the scopes.\n\
    The borrow checker can't determine this either, because it doesn't know how the lifetimes \
    of {2} and {3} relate to the lifetime of the return value.\n\
    To fix this error, we'll add generic lifetime parameters that define the relationship \
    between the references so that the borrow checker can perform its analysis.
  ",
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-19".cyan(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-20".cyan(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "if".bright_yellow().bold(),
    "else".bright_yellow().bold()
  );
}

// Header: Lifetime Annotation Syntax. Abbreviated as las.
fn las_content() {
  menu::subheader_title("Lifetime Annotation Syntax");

  println!(
    "Lifetime annotations don't change how long any of the references live.\n\
    Rather, they describe the relationships of the lifetimes of multiple references to each \
    other without affecting the lifetimes.\n\
    Just as functions can accept any type when the signature specifies a generic type \
    parameter, functions can accept references with any lifetime by specifying a generic \
    lifetime parameter.
  ");

  println!(
    "Lifetime annotations have a slightly unusual syntax: The names of lifetime parameters \
    must start with an apostrophe ({0}) and are usually all lowercase and very short, like \
    generic types.\n\
    Most people use the name {1} for the first lifetime annotation.\n\
    We place lifetime parameter annotations after the {2} of a reference, using a space to \
    separate the annotation from the reference's type.
  ",
    "'".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "&".bright_yellow().bold()
  );

  println!(
    "Here are some examples—a reference to an {0} without a lifetime parameter, a reference to \
    an {0} that has a lifetime parameter named {1}, and a mutable reference to an {0} that also \
    has the lifetime {1}:\n\n\
    {2}\n\n\
    One lifetime annotation by itself doesn't have much meaning, because the annotations are \
    meant to tell Rust how generic lifetime parameters of multiple references relate to each \
    other. 
  ",
    "i32".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "&i32 //a reference\n\
    &'a i32 // a reference with an explicit lifetime\n\
    &'a mut i32// a mutable reference with an explicit lifetime".bright_yellow().bold()
  );
}

// Header: Generic Lifetimes In Function Signatures. Abbreviated as glifs.
fn glifs_content() {
  menu::subheader_title("Generic Lifetimes In Function Signatures");

  println!(
    "To use lifetime annotations in function signatures, we need to declare the generic \
    lifetime parameters inside angle brackets between the function name and the parameter \
    list, just as we did with generic type parameters.\n\n\
    {0}.\n\
    This is the relationship between lifetimes of the parameters and the return value.\n\
    We'll name the lifetime {1} and then add it to each reference, as shown in Listing 10-21.\n\n\
    See Listing 10-21: {2} for the code sample.\n\n\
    The function signature now tells Rust that for some lifetime {1}, the function takes two \
    parameters, both of which are string slices that live at least as long as lifetime {1}.\n\
    The function signature also tells Rust that the string slice returned from the function \
    will live at least as long as lifetime {1}.\n\
    In practice, it means that the lifetime of the reference returned by the {3} function \
    is the same as the smaller of the lifetimes of the values referred to by the function \
    arguments.\n\
    These relationships are what we want Rust to use when analyzing this code.\n\n\
    {4}, when we specify the lifetime parameters in this function signature, we're not \
    changing the lifetimes of any values passed in or returned.\n\
    Rather, we're specifying that the borrow checker should reject any values that don't \
    adhere to these constraints.\n\
    Note that the {3} function doesn't need to know exactly how long {5} and {6} will live, \
    only that some scope can be substituted for {1} that will satisfy this signature.
  ",
    "We want the signature to express the following constraint: The returned reference will be \
    valid as long as both of the parameters are valid".bright_white().bold(),
    "'a".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-21".cyan(),
    "longest".bright_yellow().bold(),
    "Remember".bright_white().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold()
  );

  println!(
    "When annotating lifetimes in functions, the annotations go in the function signature, \
    not in the function body.\n\
    The lifetime annotations become part of the contract of the function, much like the types \
    in the signature.\n\
    Having function signatures contain the lifetime contract means the analysis the Rust \
    compiler does can be simpler.\n\
    {0}.\n\
    {1}.
  ",
    "If there's a problem with the way a function is annotated or the way it is called, the \
    compiler errors can point to the part of our code and the constraints more precisely".green(),
    "If, instead, the Rust compiler made more inferences about what we intended the \
    relationships of the lifetimes to be, the compiler might only be able to point to a use \
    of our code many steps away from the cause of the problem".red()
  );

  println!(
    "When we pass concrete references to {0}, the concrete lifetime that is substituted \
    for {1} is the part of the scope of {2} that overlaps with the scope of {3}.\n\
    In other words, the generic lifetime {1} will get the concrete lifetime that is equal to \
    the smaller of the lifetimes of {2} and {3}.\n\
    Because we've annotated the returned reference with the same lifetime parameter {1}, the \
    returned reference will also be valid for the length of the smaller of the \
    lifetimes of {2} and {3}.\n\n\
    See Listing 10-22: {4} for the code sample. and\n\
    Listing 10-23: {5}.
  ",
    "longest".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "x".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-22".cyan(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-23".cyan()
  );


}

// Header: Relationships. Abbreviated as rel.
fn rel_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Relationships");

  println!(
    "The way in which you need to specify lifetime parameters depends on what your function \
    is doing.\n\
    For example, if we changed the implementation of the {0} function to always return the \
    first parameter rather than the longest string slice, we wouldn't need to specify a \
    lifetime on the {1} parameter.\n\
    The following code will compile:\n\n\
    {2}.\n\n\
    We've specified a lifetime parameter {3} for the parameter {4} and the return type, but not \
    for the parameter {1}, because the lifetime of {1} does not have any relationship with the \
    lifetime of {4} or the return value.
  ",
    "longest".bright_yellow().bold(),
    "y".bright_yellow().bold(),
    "fn longest<'a>(x: &'a str, y: &str) -> &'a str {\n\
    \u{2003}\u{2003}x\n\
    }".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "x".bright_yellow().bold()
  );

  println!(
    "When returning a reference from a function, the lifetime parameter for the return type \
    needs to match the lifetime parameter for one of the parameters.\n\
    If the reference returned does not refer to one of the parameters, it must refer to a \
    value created within this function.\n\
    However, this would be a dangling reference because the value will go out of scope at \
    the end of the function.\n\
    Consider this attempted implementation of the {0} function that won't compile:\n\
    {1}.\n\n\
    Here, even though we've specified a lifetime parameter {2} for the return type, this \
    implementation will fail to compile because the return value lifetime is not related \
    to the lifetime of the parameters at all.\n\
    Here is the error message we get: {3}\n\n\
    The problem is that {4} goes out of scope and gets cleaned up at the end of the {0} \
    function.\n\
    We're also trying to return a reference to {4} from the function.\n\
    There is no way we can specify lifetime parameters that would change the dangling \
    reference, and Rust won't let us create a dangling reference.\n\
    In this case, the best fix would be to return an owned data type rather than a reference \
    so that the calling function is then responsible for cleaning up the value.\n\n\
    Ultimately, lifetime syntax is about connecting the lifetimes of various parameters and \
    return values of functions.\n\
    Once they're connected, Rust has enough information to allow memory-safe operations and \
    disallow operations that would create dangling pointers or otherwise violate memory safety.
  ",
    "longest".bright_yellow().bold(),
    "fn longest<'a>(x: &str, y: &str) -> &'a str {\n\
    \u{2003}\u{2003}let result = String::from(\"really long string\");\n\
    \u{2003}\u{2003}result.as_str()\n\
     }".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "cannot return value referencing local variable 'result'".bright_yellow().bold(),
    "result".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    {solid_disc} When returning a reference from a function, the lifetime parameter for the \
    return type needs to match the lifetime parameter for one of the parameters.\n\
    {solid_disc} Lifetime syntax is about connecting the lifetimes of various parameters and \
    return values of functions
  ",
    "REMEMBER".bright_white().bold()
  );

}

// Header: Generic lifetimes In Struct Definitions. Abbreviated as glisd.
fn glisd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Generic lifetimes In Struct Definitions");

  println!(
    "So far, the structs we've defined all hold owned types.\n\
    We can define structs to hold references, but in that case, we would need to add a \
    lifetime annotation on every reference in the struct's definition.\n\
    Listing 10-24 has a struct named {0} that holds a string slice.\n\n\
    See Listing 10-24: {1}, for code sample.\n\n\
    This struct has the single field {2} that holds a string slice, which is a \
    reference.\n\
    As with generic data types, we declare the name of the generic lifetime parameter \
    inside angle brackets after the name of the struct so that we can use the lifetime \
    parameter in the body of the struct definition.\n\
    This annotation means an instance of {0} can't outlive the reference it \
    holds in its {2} field.\n\n\
    The {3} function here creates an instance of the {0} struct that holds \
    a reference to the first sentence of the {4} owned by the variable {5}.\n\
    The data in {5} exists before the {0} instance is created.\n\
    In addition, {5} doesn't go out of scope until after the {0} goes out \
    of scope, so the reference in the {0} instance is valid.
  ",
    "ImportantExcerpt".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-24".cyan(),
    "part".bright_yellow().bold(),
    "main".bright_yellow().bold(),
    "String".bright_yellow().bold(),
    "novel".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} We declare the name of the generic lifetime parameter inside angle \
    brackets after the name of the struct.\n\
    {solid_disc} Generic lifetimes in Struct make sure that an instance of a Struct do not \
    outlive the reference its keys hold.
  ",
    "REMEMBER".bright_white().bold()
  )

}

// Header: Lifetime Elision. Abbreviated as le.
fn le_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Lifetime Elision");

  println!(
    "You've learned that every reference has a lifetime and that you need to specify \
    lifetime parameters for functions or structs that use references.\n\
    In Listing 10-25, we have a function that compiled without lifetime annotations.\n\n\
    See Listing 10-25: {}, for code sample.\n\n\
    The reason this function compiles without lifetime annotations is historical: In early \
    versions (pre-1.0) of Rust, this code wouldn't have compiled, because every reference \
    needed an explicit lifetime.\n\
    At that time, the function signature would have been written like this:\n\
    {1}
  ",
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-25".cyan(),
    "fn first_word<'a>(s: &'a str) -> &'a str {".bright_yellow().bold()
  );

  println!(
    "After writing a lot of Rust code, the Rust team found that Rust programmers were \
    entering the same lifetime annotations over and over in particular situations.\n\
    These situations were predictable and followed a few deterministic patterns.\n\
    The developers programmed these patterns into the compiler's code so that the borrow \
    checker could infer the lifetimes in these situations and wouldn't need explicit \
    annotations.\n\n\
    This piece of Rust history is relevant because it's possible that more deterministic \
    patterns will emerge and be added to the compiler.\n\
    In the future, even fewer lifetime annotations might be required.
  ");

  println!(
    "{}\n\n\
    The patterns programmed into Rust's analysis of references are called the {1}.\n\
    These aren't rules for programmers to follow; they're a set of particular cases that \
    the compiler will consider, and if your code fits these cases, you don't need to write \
    the lifetimes explicitly.\n\n\
    The elision rules don't provide full inference.\n\
    If there is still ambiguity about what lifetimes the references have after Rust applies \
    the rules, the compiler won't guess what the lifetime of the remaining references \
    should be.\n\
    Instead of guessing, the compiler will give you an error that you can resolve by \
    adding the lifetime annotations.\n\n\
    Lifetimes on function or method parameters are called {2}, and lifetimes \
    on return values are called {3}.
  ",
    "LIFETIME ELISION RULES".bright_magenta().bold(),
    "lifetime elision rules".italic(),
    "input lifetimes".italic(),
    "output lifetimes".italic()
  );

  println!(
    "{}\n\n\
    The compiler uses three rules to figure out the lifetimes of the references when there \
    aren't explicit annotations.\n\
    The first rule applies to input lifetimes, and the second and third rules apply to \
    output lifetimes.\n\
    If the compiler gets to the end of the three rules and there are still references for \
    which it can't figure out lifetimes, the compiler will stop with an error.\n\
    These rules apply to {1} definitions as well as {2} blocks.
  ",
    "THREE RULES OF LIFETIME ELISION".bright_magenta().bold(),
    "fn".bright_yellow().bold(),
    "impl".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    The first rule is that the compiler assigns a lifetime parameter to each parameter \
    that's a reference.\n\
    In other words, a function with one parameter gets one lifetime parameter: {1} \
    a function with two parameters gets two separate lifetime parameters: {2} and so on.
  ",
    "FIRST RULE".bright_magenta().bold(),
    "fn foo<'a>(x: &'a i32);".bright_yellow().bold(),
    "fn foo<'a, 'b>(x: &'a i32, y: &'b i32);".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    The second rule is that, if there is exactly one input lifetime parameter, that lifetime \
    is assigned to all output lifetime parameters: {1}.
  ",
    "SECOND RULE".bright_magenta().bold(),
    "fn foo<'a>(x: &'a i32) -> &'a i32".bright_yellow().bold()
  );

  println!(
    "{}\n\n\
    The third rule is that, if there are multiple input lifetime parameters, but one of \
    them is {1} or {2} because this is a method, the lifetime of {3} is assigned \
    to all output lifetime parameters.\n\
    This third rule makes methods much nicer to read and write because fewer symbols \
    are necessary.\n\n\
    See: {4}, and look for the part: Let's pretend we're the compiler. Where the \
    application of the three rules are applied.
  ",
    "THIRD RULE".bright_magenta().bold(),
    "&self".bright_yellow().bold(),
    "&mut self".bright_yellow().bold(),
    "self".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#lifetime-elision".cyan()
  );

  println!(
    "{}\n\n\
    {solid_disc} Every reference has a lifetime and that you need to specify lifetime \
    parameters for functions or structs that use references.\n\
    {solid_disc} There are particular situations where explicit annotations are not \
    needed because these situations follow a few deterministic patterns called \
    lifetime elision rules.\n\
    {solid_disc} The third rule really only applies in method signatures.\n\
    {solid_disc} Lifetimes on function or method parameters are called input lifetimes, \
    and lifetimes on return values are called output lifetimes.

  ",
    "REMEMBER".bright_white().bold()
  )
}

// Header: Generics In Method Definitions. Abrreviated as gimd.
fn gimd_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Generics In Method Definitions");

  println!(
    "When we implement methods on a struct with lifetimes, we use the same syntax as that of \
    generic type parameters, as shown in Listing 10-11: {}.\n\
    Where we declare and use the lifetime parameters depends on whether they're related to \
    the struct fields or the method parameters and return values.\n\n\
    Lifetime names for struct fields always need to be declared after the {1} keyword and \
    then used after the struct's name because those lifetimes are part of the struct's type.
  ",
    "https://doc.rust-lang.org/book/ch10-01-syntax.html#listing-10-11".cyan(),
    "impl".bright_yellow().bold()
  );

  println!(
    "In method signatures inside the {} block, references might be tied to the lifetime \
    of references in the struct's fields, or they might be independent.\n\
    In addition, the lifetime elision rules often make it so that lifetime annotations \
    aren't necessary in method signatures.\n\
    Let's look at some examples using the struct named {1} that we defined in \
    Listing 10-24: {2}.\n\n\
    First, we'll use a method named {3} whose only parameter is a reference to {4} and \
    whose return value is an {5}, which is not a reference to anything:\n\n\
    {6}\n\n\
    The lifetime parameter declaration after {0} and its use after the type name are \
    required, but because of the first elision rule, we're not required to annotate the \
    lifetime of the reference to {4}.
  ",
    "impl".bright_yellow().bold(),
    "ImportantExcerpt".bright_yellow().bold(),
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#listing-10-24".cyan(),
    "level".bright_yellow().bold(),
    "self".bright_yellow().bold(),
    "i32".bright_yellow().bold(),
    "impl<'a> ImportantExcerpt<'a> {\n\
    \u{2003}\u{2003}fn level(&self) -> i32 {\n\
    \u{2003}\u{2003}\u{2003}\u{2003}3\n\
    \u{2003}\u{2003}}\n\
    }".bright_yellow().bold()
  );

  println!(
    "Here is an example where the third lifetime elision rule applies:\n\n\
    {0}\n\n\
    There are two input lifetimes, so Rust applies the first lifetime elision rule and \
    gives both {1} and {2} their own lifetimes.\n\
    Then, because one of the parameters is {1}, the return type gets the lifetime of \
    {1}, and all lifetimes have been accounted for.
  ",
    "impl<'a> ImportantExcerpt<'a> {\n\
    \u{2003}\u{2003}fn announce_and_return_part(&self, announcement: &str) -> &str {\n\
    \u{2003}\u{2003}\u{2003}\u{2003}println!(\"Attention please: {announcement}\");\n\
    \u{2003}\u{2003}\u{2003}\u{2003}self.part\n\
    \u{2003}\u{2003}}\n\
    }".bright_yellow().bold(),
    "&self".bright_yellow().bold(),
    "announcement".bright_yellow().bold(),
  );

  println!(
    "{}\n\n\
    {solid_disc} Where we declare and use the lifetime parameters depends on whether \
    they're related to the struct fields or the method parameters and return values.\n\
    {solid_disc} Lifetime names for struct fields always need to be declared after the {1} \
    keyword and then used after the struct's name because those lifetimes are part of the \
    struct's type.\n\
    {solid_disc} The lifetime parameter declaration after {1} and its use after the type \
    name are required.
  ",
    "REMEMBER".bright_white().bold(),
    "impl".bright_yellow().bold()
  )  
}

// Header: The Static Lifetime. Abbreviated as tsl.
fn tsl_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The Static Lifetime");

  println!(
    "One special lifetime we need to discuss is {0}, which denotes that the affected reference can live for the entire duration of the program.\n\
    All string literals have the {0} lifetime, which we can annotate as follows:\n\n\
    {1}\n\n\
    The text of this string is stored directly in the program's binary, which is always available. Therefore, the lifetime of all string literals is {0}.\n\
    You might see suggestions in error messages to use the {0} lifetime.\n\
    But before specifying {0} as the lifetime for a reference, think about whether or not the reference you have actually lives the entire lifetime of \
    your program, and whether you want it to.\n\
    Most of the time, an error message suggesting the {0} lifetime results from attempting to create a dangling reference or a mismatch of the available \
    lifetimes.\n\
    In such cases, the solution is to fix those problems, not to specify the {0} lifetime.
  ",
    "'static".bright_yellow().bold(),
    "let s: &'static str = \"I have a static lifetime.\";".bright_yellow().bold(),
  );

  println!(
    "{0}\n\n\
    {solid_disc} All string literals have the {1} lifetime.\n\
    {solid_disc} The text of string literal is stored directly in the program's binary, which is always available.\
    Therefore, the lifetime of all string literals is {1}.\n\
    {solid_disc} Before specifying {1} as the lifetime for a reference, think about whether or not the reference you have actually lives the entire lifetime \
    of your program, and whether you want it to.
  ",
    "REMEMBER".bright_white().bold(),
    "'static".bright_yellow().bold(),

  )

}

// Header: Generic Type Parameters, Trait Bounds, and Lifetimes. Abbreviated as gtptbal.
fn gtptbal_content() {
  menu::subheader_title("Generic Type Parameters, Trait Bounds, and Lifetimes");

  println!(
    "Let's briefly look at the syntax of specifying generic type parameters, trait bounds, and lifetimes all in one function!\n\n\
    See: {}, for code sample.\n\n\
    In the code sample, the {1} function from Listing 10-21 is shown.\n\
    But now it has an extra parameter named {2} of the generic type {3}, which can be filled in by any type that implements the {4} trait as specified by \
    the {5} clause.\n\
    This extra parameter will be printed using {7}, which is why the {4} trait bound is necessary.\n\
    Because lifetimes are a type of generic, the declarations of the lifetime parameter {6} and the generic type parameter {3} go in the same list inside the \
    angle brackets after the function name.
  ",
    "https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html#generic-type-parameters-trait-bounds-and-lifetimes".cyan(),
    "longest".bright_yellow().bold(),
    "ann".bright_yellow().bold(),
    "T".bright_yellow().bold(),
    "Display".bright_yellow().bold(),
    "where".bright_yellow().bold(),
    "'a".bright_yellow().bold(),
    "{}".bright_yellow().bold(),
  )
}

