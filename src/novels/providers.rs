use regex::Regex;
use super::boxnovel::Boxnovel;

pub trait NovelProvider {
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
  let provider_regex = Regex::new(r"^(https?://)?([\s\S]*\.)?boxnovel.com/?[\s\S]*$")
    .expect("Could not parse and load the regex engine.");

  if provider_regex.is_match(url) {
    return Ok(
      Box::new(Boxnovel::new())
    );
  }

  let err = format!("There is no supported provider for url \"{}\"", url);
  Err(err)
}

#[test]
fn test_provider_simple() {
  let expected = "https://boxnovel.com".to_string();
  let provider = load(&expected)
    .expect("This test should pass.");

  assert_eq!("boxnovel.com".to_string(), provider.get_name())
}

#[test]
fn test_provider_with_domain() {
  let expected = "https://test.boxnovel.com".to_string();
  let provider = load(&expected)
    .expect("This test should pass.");

  assert_eq!("boxnovel.com".to_string(), provider.get_name())
}

#[test]
fn test_provider_long() {
  let expected = "https://boxnovel.com/novel/i-might-be-a-fake-cultivator/".to_string();
  let provider = load(&expected)
    .expect("This test should pass.");

  assert_eq!("boxnovel.com".to_string(), provider.get_name())
}

#[test]
fn test_provider_get_name_from_thread() {
  use std::thread;

  let url = "https://boxnovel.com".to_string();

  let thread = thread::spawn(move || {
    let provider = load(&url)
      .expect("Failed to load the provider");

    let expected = "boxnovel.com".to_string();
    let received: String = provider.get_name();

    assert_eq!(expected, received, "The provider should be accessible from multiple threads.")
  });
  thread.join()
    .unwrap();
}