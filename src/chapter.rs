pub struct Chapter<'a> {
  title: &'a str,
  sections: Vec<Section<'a>>
}

impl<'a> Chapter<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_content(&self) {
    for section in &self.sections {
      println!("{}", section.get_title());
    }
  }

  // Associate Functions.
  pub fn new(title: &'a str, sections: Vec<Section<'a>>) -> Self {
    Self {
      title,
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