use owo_colors::OwoColorize;

use crate::{chapter, menu};

pub fn content(section_title: &str, section: &str) {
  let subheaders: [chapter::SubHeader; 5];
  subheaders = [
    chapter::SubHeader::new("Section Introduction", si_content),
    chapter::SubHeader::new("Type Safety and Abstraction with the Newtype Pattern", tsawtnp_content),
    chapter::SubHeader::new("Type Synonyms and Type Aliases", tsata_content),
    chapter::SubHeader::new("The Never Type That Never Returns", tntnr_content),
    chapter::SubHeader::new("Dynamically Sized Types and the Sized Trait", dstast_content),

  ];

  chapter::SubHeader::prompt_subheader(&subheaders, section_title, section);
}

// Subheaders content below.

// Header: Section Introduction. Abbreviated as si.
fn si_content() {
//   let solid_disc = "\u{2022}";

  menu::subheader_title("Section Introduction: Advanced Types");

  println!(
	"The Rust type system has some features that we've so far mentioned but haven't yet discussed.\n\
	We'll start by discussing newtypes in general as we examine why they are useful as types.\n\
	Then, we'll move on to type aliases, a feature similar to newtypes but with slightly different semantics.\n\
	We'll also discuss the {0} type and dynamically sized types.
  ",
	"!".bright_yellow().bold(),
	)
}

// Header: Type Safety and Abstraction with the Newtype Pattern. Abbreviated as tsawtnp.
fn tsawtnp_content() {
//   let solid_disc = "\u{2022}";

  menu::subheader_title("Type Safety and Abstraction with the Newtype Pattern");

	println!(
	"This section assumes you've read the earlier section “Implementing External Traits with the Newtype Pattern”:{0}.\n\
	The newtype pattern is also useful for tasks beyond those we've discussed so far, including statically enforcing that values are \
	never confused and indicating the units of a value.\n\
	You saw an example of using newtypes to indicate units in Listing 20-16: Recall that the {1} and {2} structs wrapped \
	{3} values in a newtype.\n\
	If we wrote a function with a parameter of type {1}, we wouldn't be able to compile a program that accidentally tried to \
	call that function with a value of type {2} or a plain {3}.\n\n\
	We can also use the newtype pattern to abstract away some implementation details of a type: The new type can expose a public \
	API that is different from the API of the private inner type.\n\n\
	Newtypes can also hide internal implementation.\n\
	For example, we could provide a {4} type to wrap a {5} that stores a person's ID associated with their name.\n\
	Code using {4} would only interact with the public API we provide, such as a method to add a name string to the {4} \
	collection; that code wouldn't need to know that we assign an {7} ID to names internally.\n\
	The newtype pattern is a lightweight way to achieve encapsulation to hide implementation details, which we discussed in the \
	“Encapsulation that Hides Implementation Details” section in Chapter 18:{6}.
	",
	"https://doc.rust-lang.org/book/ch20-02-advanced-traits.html#implementing-external-traits-with-the-newtype-pattern".bright_cyan(),
	"Millimeters".bright_yellow().bold(),
	"Meters".bright_yellow().bold(),
	"u32".bright_yellow().bold(),
	"People".bright_yellow().bold(),
	"HashMap<i32, String>".bright_yellow().bold(),
	"https://doc.rust-lang.org/book/ch18-01-what-is-oo.html#encapsulation-that-hides-implementation-details".bright_cyan(),
	"i32".bright_yellow().bold(),
	)
}

// Header: Type Synonyms and Type Aliases. Abbreviated as tsata.
fn tsata_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Type Synonyms and Type Aliases");

	println!(
	"Rust provides the ability to declare a type alias to give an existing type another name.\n\
	For this we use the {0} keyword.\n\
	For example, we can create the alias {1} to {2} like so:\n\n\
	See:{3}, for code samples and complete reading.
	",
	"type".bright_yellow().bold(),
	"Kilometers".bright_yellow().bold(),
	"i32".bright_yellow().bold(),
	"https://doc.rust-lang.org/book/ch20-03-advanced-types.html#type-synonyms-and-type-aliases".bright_cyan(),
	);

	println!(
	"{0}\n\n\
	{solid_disc} Type aliases don't create a new type; they just create a new name for an existing type.\n\
	{solid_disc} The main use case for type synonyms is to reduce repetition.\n\
	{solid_disc} Choosing a meaningful name for a type alias can help communicate your intent as well.\n\
	{solid_disc} Type aliases are also commonly used with the {1} type for reducing repetition.\n\
	{solid_disc} The type alias helps in two ways: It makes code easier to write and it gives us a consistent interface across \
	all of {2}. 
	",
	"REMEMBER".bright_white().bold(),
	"Result<T, E>".bright_yellow().bold(),
	"std::io".bright_yellow().bold(),
	)
}

// Header: The Never Type That Never Returns. Abbreviated as tntnr.
fn tntnr_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("The Never Type That Never Returns");

	println!(
	"Rust has a special type named {0} that's known in type theory lingo as the {1} type because it has no values.\n\
	We prefer to call it the {2} type because it stands in the place of the return type when a function will never return.\n\n\
	See:{3}, for complete reading.
	",
	"!".bright_yellow().bold(),
	"empty".italic().bold(),
	"never".italic().bold(),
	"https://doc.rust-lang.org/book/ch20-03-advanced-types.html#the-never-type-that-never-returns".bright_cyan(),
	);

	println!(
	"{0}\n\n\
	{solid_disc} Functions that return never are called {1} functions.
	",
	"REMEMBER".bright_white().bold(),
	"diverging".italic().bold(),
	)
}

// Header: Dynamically Sized Types and the Sized Trait. Abbreviated as dstast.
fn dstast_content() {
  let solid_disc = "\u{2022}";

  menu::subheader_title("Dynamically Sized Types and the Sized Trait");

	println!(
	"Rust needs to know certain details about its types, such as how much space to allocate for a value of a particular type.\n\
	This leaves one corner of its type system a little confusing at first: the concept of {0}.\n\
	Sometimes referred to as {1} or {2} types, these types let us write code using values whose size we can know only at \
	runtime.
	",
	"dynamically sized types".italic().bold(),
	"DSTs".italic().bold(),
	"unsized".italic().bold(),
	);

	println!(
	"{0}\n\n\
	Let's dig into the details of a dynamically sized type called {1}, which we've been using throughout the book.\n\
	That's right, not {2}, but {1} on its own, is a DST.\n\
	In many cases, such as when storing text entered by a user, we can't know how long the string is until runtime.\n\
	That means we can't create a variable of type {1}, nor can we take an argument of type {1}.\n\n\
	See:{3}, for code samples and complete reading.
	",
	"The DST str".bright_magenta().bold(),
	"str".bright_yellow().bold(),
	"&str".bright_yellow().bold(),
	"https://doc.rust-lang.org/book/ch20-03-advanced-types.html#dynamically-sized-types-and-the-sized-trait".bright_cyan(),
	);

	println!(
	"{0}\n\n\
	it's not possible to create a variable holding a dynamically sized type. So, what do we do? We put DSTs behind pointers!\n\
	Recall from the “String Slices” section in Chapter 4:{1}, that the slice data structure only stores the starting position and \
	the length of the slice.\n\
	So, although {2} is a single value that stores the memory address of where the {3} is located, a string slice is two values: \n\
	the address of the {4} and its length.\n\
	As such, we can know the size of a string slice value at compile time: It's twice the length of a {5}.\n\
	That is, we always know the size of a string slice, no matter how long the string it refers to is.\n\
	In general, this is the way in which dynamically sized types are used in Rust: They have an extra bit of metadata that stores \
	the size of the dynamic information.\n\
	The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind a pointer of \
	some kind.
	",
	"Putting DSTs behind pointers".bright_magenta().bold(),
	"https://doc.rust-lang.org/book/ch04-03-slices.html#string-slices".bright_cyan(),
	"&T".bright_yellow().bold(),
	"T".bright_yellow().bold(),
	"str".bright_yellow().bold(),
	"usize".bright_yellow().bold(),
	);

	println!(
	"{0}\n\n\
	We can combine {1} with all kinds of pointers: for example, {2} or {3}.\n\
	In fact, you've seen this before but with a different dynamically sized type: traits.\n\
	Every trait is a dynamically sized type we can refer to by using the name of the trait.\n\
	In the “Using Trait Objects to Abstract over Shared Behavior” section in Chapter 18:{4}, we mentioned that to use traits as \
	trait objects, we must put them behind a pointer, such as {5} or {6} ({7} would work too).
	",
	"Combining str with all kinds of pointers".bright_magenta().bold(),
	"str".bright_yellow().bold(),
	"Box<str>".bright_yellow().bold(),
	"Rc<str>".bright_yellow().bold(),
	"https://doc.rust-lang.org/book/ch18-02-trait-objects.html#using-trait-objects-to-abstract-over-shared-behavior".bright_cyan(),
	"&dyn Trait".bright_yellow().bold(),
	"Box<dyn Trait>".bright_yellow().bold(),
	"Rc<dyn Trait>".bright_yellow().bold(),
	);

	println!(
	"{0}\n\n\
	To work with DSTs, Rust provides the {1} trait to determine whether or not a type's size is known at compile time.\n\
	This trait is automatically implemented for everything whose size is known at compile time.\n\
	In addition, Rust implicitly adds a bound on {1} to every generic function.
	",
	"The Sized trait".bright_magenta().bold(),
	"Sized".bright_yellow().bold(),
	);

	println!(
	"{0}\n\n\
	By default, generic functions will work only on types that have a known size at compile time.\n\
	However, you can use the following special syntax to relax this restriction.\n\
	A trait bound on {1} means “{2} may or may not be {3},” and this notation overrides the default that generic types must have \
	a known size at compile time.\n\
	The {4} syntax with this meaning is only available for {3}, not any other traits.
	",
	"The ?Sized special syntax".bright_magenta().bold(),
	"?Sized".bright_yellow().bold(),
	"T".bright_yellow().bold(),
	"Sized".bright_yellow().bold(),
	"?Trait".bright_yellow().bold(),
	);

	println!(
	"{0}\n\n\
	{solid_disc} {1}, sometimes referred to as {2} or {3} types, these types let us write code using values whose size we can \
	know only at runtime.\n\
	{solid_disc} Rust needs to know how much memory to allocate for any value of a particular type, and all values of a type must \
	use the same amount of memory.\n\
	{solid_disc} The golden rule of dynamically sized types is that we must always put values of dynamically sized types behind \
	a pointer of some kind.\n\
	{solid_disc} Recall that the slice data structure only stores the starting position and the length of the slice, \
	so it's two values.\n\
	{solid_disc} The {4} trait is automatically implemented for everything whose size is known at compile time.\n\
	{solid_disc} By default, generic functions will work only on types that have a known size at compile time. \
	However, you can use the following special syntax to relax this restriction using the {5}
	",
	"REMEMBER".bright_white().bold(),
	"dynamically sized types".italic().bold(),
	"DSTs".italic().bold(),
	"unsized".italic().bold(),	
	"Sized".bright_yellow().bold(),
	"?Sized".bright_yellow().bold(),
	)
}


