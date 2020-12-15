use super::boxnovel::Boxnovel;
use crate::novels::eatapplepies::EatApplePies;

pub trait NovelProvider {
  /// Check if this provider support the url taken as parameter.
  fn supports_url(&self, url: &str) -> bool;

  /// This function just print the name of the provider that is implementing this on runtime
  fn get_name(&self) -> String;

  /// This function receive the html in a string and return all the chapter's links in a
  /// vector.
  fn get_release_links(&self, html: &str) -> Result<Vec<String>, & str>;

  /// This function receives the html in a string and return the chapter's data from the page.
  /// The html will later be striped by calibre, for cleaning and stripping all the html was inside
  /// the div container.
  fn get_chapter(&self, html: &str) -> Option<String>;
}

pub fn load<'a>(url: &str) -> Result<Box<dyn NovelProvider>, String> {
  let providers = get_list_of_providers();

  for provider in providers {
    if provider.supports_url(url) {
      return Ok(provider);
    }
  }

  let err = format!("There is no supported provider for url \"{}\"", url);
  Err(err)
}

pub fn print_supported_websites() {
  let providers = get_list_of_providers();

  print!("Supported websites: \n");
  for provider in providers {
    print!("{}\n", provider.get_name());
  }
}

fn get_list_of_providers() -> Vec<Box<dyn NovelProvider>> {
  return vec![
    Box::new(Boxnovel::new()),
    Box::new(EatApplePies::new())
  ];
}