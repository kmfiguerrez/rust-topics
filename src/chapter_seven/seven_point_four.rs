use owo_colors::OwoColorize;

fn display_title() {
    println!("{} \n",
    "Bringing Paths into Scope with the use Keyword".bright_blue().bold(),
    );
}

pub fn display_content(){
    display_title();
    bpiswuk_content();
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