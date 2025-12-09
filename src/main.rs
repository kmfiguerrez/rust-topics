// use owo_colors::OwoColorize;



fn main() {
    // let title = "Modules";
    // bold, bright effect using style chaining
    // println!("{}", title.bold().bright_blue());
    // println!("{}", "-".repeat(title.len()).bright_blue());

    // call the library crate's module function (crate name from Cargo.toml is `rust-topics`, which maps to `rust_topics`)
    // rust_topics::chapter_seven::seven_point_two::display_content();
    // rust_topics::chapter_seven::seven_point_three::display_content();
    // rust_topics::chapter_seven::seven_point_five::display_content();
    // rust_topics::chapter_four::four_point_one::display_contents();
    rust_topics::chapter_four::four_point_two::display_contents();
    // test_code();
    
}

fn test_code() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // let r2 = &mut s;
    // let r2 = &s;
    
    
    // let r2 = &s;

    // println!("{r1}, {r2}");
    println!("{r1}");
    // let r2 = &mut s;
    let r2 = &s;

}

fn test_code2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}");
    let r3 = &mut s; // BIG PROBLEM
}