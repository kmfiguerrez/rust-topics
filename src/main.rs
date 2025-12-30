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
    // rust_topics::chapter_four::four_point_three::display_contents();
    rust_topics::chapter_six::six_point_three::display_contents();
    // test_code();
    // test_code3();
    // enum_test1();
    
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
    // let r2 = &s;

}

fn test_code2() {
    let mut s = String::from("hello");

    let r1 = &s; // no problem
    let r2 = &s; // no problem
    println!("{r1}, {r2}");
    let r3 = &mut s; // BIG PROBLEM
}

fn test_code3() {
  let s = "whatever";
  let b = s.as_bytes();
  let i = b.iter();
  let e = i.enumerate();
  println!("{:?}", b);
  println!("{:?}", e);
  // println!("{:?}", i.next());
}

fn test_code4() {
  let s = String::from("Yo");
  let sr: &str = &s;
  let sr1 = &s;

  let sl = &s[0..5];
  let sl = "Hello, world!";
  let sl1 = &sl[0..5];
  take_str("");
}

fn take_str(s: &str) {
  println!("{s}");
}

fn enum_test() {
  enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
  }

  impl Message {
      fn tawag(&self) {
      }
  }

  let m = Message::Write(String::from("hello"));
}

fn enum_test1() {
  enum Coin {
      Penny,
      Nickel,
      Dime,
      Quarter,
  }

  let c = Coin::Penny;

  match c {
      Coin::Dime => 10,
      idk => {
        println!("idk nigga!");
        5
      }
  };

}