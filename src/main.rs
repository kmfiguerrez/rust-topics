// use owo_colors::OwoColorize;



fn main() {
    // let title = "Modules";
    // bold, bright effect using style chaining
    // println!("{}", title.bold().bright_blue());
    // println!("{}", "-".repeat(title.len()).bright_blue());

    rust_topics::chapter_nine::nine_point_three::display_contents();
    // lets_panic();
    // let res = test_code();
    // println!("Result from test_code: {:?}", res);
}


fn test_code() -> Option<u32> {
    return_option()?;
    Some(64)
}

fn return_option() -> Option<u32> {
    let i = Some(32);

    // i
    None
}

