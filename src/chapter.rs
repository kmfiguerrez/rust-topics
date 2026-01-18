pub struct Chapter<'a> {
  title: &'a str,
  chapter: &'a str,
  sections: Vec<Section<'a>>
}

impl<'a> Chapter<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_sections(&self) {
    let mut i: u8 = 1;
    for section in &self.sections {
      println!("{}. {}", i, section.get_title());
      i+= 1;
    }
  }

  // Associate Functions.
  pub fn new(title: &'a str, chapter: &'a str, sections: Vec<Section<'a>>) -> Self {
    Self {
      title,
      chapter,
      sections
    }
  }
}

pub struct Section<'a> {
  title: &'a str,
}

impl<'a> Section<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  // Associate Functions.
  pub fn new(title: &'a str) -> Self {
    Self {
      title,
    }
  }
}


pub struct SubHeader<'a> {
  title: &'a str,
  content: fn()
}

impl<'a> SubHeader<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_content(&self) {
    (self.content)();
  }

  // Associate Functions.
  pub fn new(title: &'a str, content: fn()) -> Self {
    Self {
      title,
      content
    }
  }
}