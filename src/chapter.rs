pub mod content;

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

  pub fn get_chapter(&self) -> &'a str {
    self.chapter
  }  

  pub fn display_sections(&self) {
    let mut i: u8 = 1;
    for section in &self.sections {
      println!("{}. {}", i, section.get_title());
      i+= 1;
    }
  }

  pub fn get_section(&self, selected_section: u8) -> Option<&Section<'a>> {
    if selected_section == 0 {
      None
    } else {
      self.sections.get((selected_section - 1) as usize)
    }
  }

  // Associate functions.
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
  content: fn(&str)
}

impl<'a> Section<'a> {
  // Methods.
  pub fn get_title(&self) -> &'a str {
    self.title
  }

  pub fn display_content(&self) {
    (self.content)(self.get_title());
  }

  // Associate Functions.
  pub fn new(title: &'a str, content: fn(&str)) -> Self {
    Self {
      title,
      content
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