use owo_colors::OwoColorize;

fn display_title() {
    println!("{} \n",
    "7.4 Bringing Paths into Scope with the use Keyword".bright_blue().bold(),
    );
}

pub fn display_content(){
    display_title();
    // bpiswuk_content();
    // ciup_content();
    // pnnwak_content();
    // renwpu_content();
    // uep_content();
    // unptculul_content();
    tgo_content();
}


// Subheaders content below.

// Bringing Paths into Scope with the use Keyword. Abbreviated as bpiswuk.
fn bpiswuk_content() {
    println!("\
        Having to write out the paths to call functions can feel inconvenient and repetitive. \n\
        Fortunately, there’s a way to simplify this process: we can create a shortcut to a path with the {} keyword once, \n\
        and then use the shorter name everywhere else in the scope. \n\n\

        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#bringing-paths-into-scope-with-the-use-keyword \n\
        for code example and read the explanation until the next subheader: Creating Idiomatic use Paths, begins.
    ",
    "use".green().bold()
    );

    println!("\
        Adding {0} and a path in a scope is similar to creating a symbolic link in the filesystem. \n\
        Paths brought into scope with {0} also check privacy, like any other paths.
    ",  
    "use".green().bold()
    );

    println!("\
        Note that {0} only creates the shortcut for the particular scope (module scope) in which the {0} occurs. \n\
        (A {0} statement only applies in the scope it’s in.)
    ",
    "use".green().bold()
    );


}


// Subheader: Creating Idiomatic use Paths. Abbreviated ciup.
fn ciup_content() {
    let solid_disc = "\u{2022}";

    // Subheader title.
    println!("{} \n", "Creating Idiomatic use Paths".bright_blue().bold());

    // println!("\
    //     See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#creating-idiomatic-use-paths \n\
    //     for code example.
    // ");

    println!("\
        Bringing the function’s parent module into scope with {0} means we have to specify the parent module when calling the function. \n\
        Specifying the parent module when calling the function makes it clear that the function isn’t locally defined while still minimizing \n\
        repetition of the full path. \
        The code in Listing 7-13 is unclear as to where {1} is defined. \n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-13
    ",
    "use".green().bold(),
    "add_to_waitlist".green().bold()
    );

    println!("\
        On the other hand, when bringing in structs, enums, and other items with {0}, it’s idiomatic \
        to specify the full path. \n\
        Listing 7-14 shows the idiomatic way to bring the standard library’s {1} struct into the scope of a binary crate. \n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-14,\n\
        for code example. \n\n\

        There’s no strong reason behind this idiom: it’s just the convention that has emerged, and folks have gotten used to reading and \n\
        writing Rust code this way. \n\n\

        The exception to this idiom is if we’re bringing two items with the same name into scope \
        with {0} statements, because Rust doesn’t allow that. \n\
        Listing 7-15 shows how to bring two {2} types into scope that have the same name but \
        different parent modules, and how to refer to them. \n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-15, \
        for code example. \n\n\

        As you can see, using the parent modules distinguishes the two {2} types. \n\
        If instead we specified {5} and {6}, \
        we’d have two {2} types in the same scope, \n\
        and Rust wouldn’t know which one we meant when we used {2}. \n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-15 \n\n\

        So remember: \n\
        {solid_disc} Specify the full path when bringing structs, enums and other items with {0} into scope. \n\
        {solid_disc} Don't specify the full path when bringing {3} and {4} \
        into scope.
    ",
    "use".green().bold(),
    "HashMap".green().bold(),
    "Result".green().bold(),
    "functions".bold(),
    "items with the same name but different parent modules".bold(),
    "use std::fmt::Result".green().bold(),
    "use std::io::Result".green().bold()
    );
}

// Subheader: Providing New Names with the as Keyword. Abbreviated as pnnwak.
fn pnnwak_content() {
    // Subheader title.
    println!("{} \n", "Providing New Names with the as Keyword".bright_blue().bold());

    println!("\
        There’s another solution to the problem of bringing two types of the same name into the same scope with \n\
        {0}: after the path, we can specify {1} and a new local name, \
        or {2}, for the type. \n\
        Listing 7-16 shows another way to write the code in Listing 7-15 by renaming one of the two {3} types using {1}. \n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-16, \n\
        for code example. \n\n\

        In the second {0} statement, we chose the new name {4} for the {5} type, which won’t conflict \n\
        with the {3} from {6} that we’ve also brought into scope. \n\
        Listing 7-15 and Listing 7-16 are considered idiomatic, so the choice is up to you!

    ",
    "use".green().bold(),
    "as".green().bold(),
    "alias".italic(),
    "Result".green().bold(),
    "IoResult".green().bold(),
    "std::io::Result".green().bold(),
    "std::fmt".green().bold(),

    );
}

// Subheader: Re-exporting Names with pub use. Abbreviated as renwpu.
fn renwpu_content() {
    // Subheader title.
    println!("{} \n", "Re-exporting Names with pub use".bright_blue().bold());

    println!("\
        When we bring a name into scope with the {0} keyword, the name is private to the scope \
        into which we imported it. \n\
        To enable code outside that scope to refer to that name as if it had been defined in \
        that scope, we can combine {1} and {0}. \n\
        This technique is called {2} because we’re bringing an item into scope \
        but also making that item available for others to bring into their scope. \n\n\
        See: https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-17, \n\
        for code example.
    ",
    "use".green().bold(),
    "pub".green().bold(),
    "re-exporting".italic()
    );

    println!("\
        Before this change, external code would have to call the {0} function by using the path \n\
        {1}, which also would have required the {2} module to be marked as {3}. \n\
        Now that this {4} has re-exported the {5} module from the root module, \
        external code can use the path {6} instead.
    ",
    "add_to_waitlist".green().bold(),
    "restaurant::front_of_house::hosting::add_to_waitlist()".green().bold(),
    "front_of_house".green().bold(),
    "pub".green().bold(),
    "pub use".green().bold(),
    "hosting".green().bold(),
    "restaurant::hosting::add_to_waitlist()".green().bold()
    );

}

// Subheader: Using External Packages. Abbreviated as uep.
fn uep_content() {
    // Subheader title.
    println!("{} \n", "Using External Packages".bright_blue().bold());

    println!("\
        To use an external package, we add the package to our project’s dependencies by adding \n\
        the package name and version to the [dependencies] section of {1}. \n\
        Then we bring the items we want to use into scope with {0} statements in our code. \n\n\
        See: {2}, \n\
        for code example.
    ",
    "use".green().bold(),
    "Cargo.toml".italic(),
    "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-external-packages".cyan()
    );

    println!("\
        Members of the Rust community have made many packages available at {0}, \n\
        and pulling any of them into your package involves these same \n\
        steps: listing them in your package’s {1} file and using use to bring items \
        from their crates into scope.
    ",
    "https://crates.io/".cyan(),
    "Cargo.toml".italic()
    );

    println!("\
        Note that the standard {0} library is also a crate that’s external to our package. \n\
        Because the standard library is shipped with the Rust language, we don’t need to change {1} to include {0}. \n\
        But we do need to refer to it with {2} to bring items from there into our package’s scope.
    ",
    "std".green().bold(),
    "Cargo.toml".italic(),
    "use".green().bold()
    );




}

// Subheader: Using Nested Paths to Clean Up Large use Lists. Abbreviated as unptculul.
fn unptculul_content() {
    // Subheader title.
    println!("{} \n", "Using Nested Paths to Clean Up Large use Lists".bright_blue().bold());

    println!("\
        When bringing in multiple items from the same parent module, we can use a nested path to group those items together \n\
        in one {0} statement rather than having separate {0} statements for each item. \n\
        Listing each item on its own line can take up a lot of vertical space in our files. \n\n\
        See: {1}, \n\
        for code example.
    ",
    "use".green().bold(),
    "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#using-nested-paths-to-clean-up-large-use-lists".cyan()
    );

    println!("\
        Instead, we can use nested paths to bring the same items into scope in one line. \n\
        We do this by specifying the common part of the path, followed by two colons, \n\
        and then curly brackets around a list of the parts of the paths that differ, \n\
        as shown in Listing 7-18. \n\
        see: {} \n\n\
        This syntax is especially useful when bringing in many items from the same parent module, \n\
        as it cuts down on repetition and makes it clear that the items all come from the same place. \n\n\
        In bigger programs, bringing many items into scope from the same crate or module using nested paths \n\
        can reduce the number of separate use statements needed by a lot!
    ",
    "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-18".cyan()
    );

    println!("\
        We can use a nested path at any level in a path, which is useful when combining two {} statements that share a subpath. \n\
        For example, Listing 7-19 shows two {0} statements: one that brings {1} into scope and one that brings {2} into scope. \n\
        See: {3}, for code example. \n\n\
        The common part of these two paths is {1}, and that’s the complete first path. To merge these two paths into one {0} statement, \n\
        we can use {4} in the nested path, as shown in Listing 7-20. \n\
        See: {5}, for code sample.
    ",
    "use".green().bold(),
    "std::io".green().bold(),
    "std::io::Write".green().bold(),
    "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-19".cyan(),
    "self".green().bold(),
    "https://doc.rust-lang.org/book/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html#listing-7-20".cyan()
    );

}

// Subheader: The Glob Operator. Abbreviated as  tgo.
fn tgo_content() {
    // Subheader title.
    println!("{} \n", "The Glob Operator".bright_blue().bold());

    println!("\
        If we want to bring all public items defined in a path into scope, we can specify that path followed by the {} glob \n\
        operator: {1} \n\n\
        This {2} statement brings all public items defined in {3} into the current scope. \n\n\
        {4} \n\
        Glob can make it harder to tell what names are in scope and where a name used in your program was defined. \n\
        Additionally, if the dependency changes its definitions, what you’ve imported changes as well, which may lead to \n\
        compiler errors when you upgrade the dependency if the dependency adds a definition with the same name as a definition \n\
        of yours in the same scope, for example. \n\n\
        The glob operator is often used when testing to bring everything under test into the {5} module.
    ",
    "*".green().bold(),
    "use std::collections::*;".green().bold(),
    "use".green().bold(),
    "std::collections".green().bold(),
    "Be careful when using the glob operator!".red(),
    "tests".green().bold()
    );
}
