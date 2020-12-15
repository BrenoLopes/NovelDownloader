use easy_scraper::Pattern;

use crate::novels::providers::NovelProvider;

use regex::Regex;

#[derive(Copy, Clone)]
pub struct Boxnovel {}

impl Boxnovel {
  pub fn new() -> Boxnovel { return Boxnovel{}; }
}

impl NovelProvider for Boxnovel
{
  fn supports_url(&self, url: &str) -> bool {
    let re = Regex::new(r"^(https?://)?([\s\S]*\.)?boxnovel.com(/[\s\S]*)?$")
      .expect("Could not parse and load the regex engine.");

    return re.is_match(url);
  }

  fn get_name(&self) -> String {
    return "boxnovel.com".to_string();
  }

  fn get_release_links(&self, html: &str) -> Result<Vec<String>, &str> {
    let mut novel_links: Vec<String> = Vec::with_capacity(300);

    let pattern = match Pattern::new(r#"
    <ul class="main version-chap">
      <li class="wp-manga-chapter">
        <a href="{{url}}">{{text}}</a>
      </li>
    </ul>
    "#){
      Ok(p) => p,
      Err(_) => return Err("Could not load the pattern.".as_ref())
    };

    let all_matches = pattern.matches(html);

    for item in all_matches.iter() {
      let link = match item.get("url") {
        Some(l) => l,
        None => return Err("The pattern has not matched any chapter link"),
      };

      novel_links.push(link.to_string());
    }

    // Remove empty links
    novel_links.retain( |e| ![""].contains(&e.as_str()) );
    // Invert the chapter list
    novel_links.reverse();

    return Ok(novel_links);
  }

  fn get_chapter(&self, html: & str) -> Option<String> {
    let pattern = Pattern::new(r#"
      <div class="entry-content_wrap">
        <div class="read-container">
          <div class="reading-content">
            <div class="text-left">
              {{data:*}}
            </div>
          </div>
        </div>
      </div>
    "#).expect("Could not load the pattern.");

    let matches = pattern.matches(html);

    let data = match matches[0].get("data") {
      None => None,
      Some(data) => Some(data.to_string())
    };

    return data;
  }
}

#[cfg(test)]
mod tests {
  use crate::novels::providers::{NovelProvider};
  use crate::novels::boxnovel::Boxnovel;

  #[test]
  fn test_supports_url_simple() {
    let url = "https://boxnovel.com";

    let expected = true;
    let received = Boxnovel::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_with_domain() {
    let url = "https://test.boxnovel.com";

    let expected = true;
    let received = Boxnovel::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_long() {
    let url = "https://boxnovel.com/novel/i-might-be-a-fake-cultivator/";

    let expected = true;
    let received = Boxnovel::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_not_supports_url_nonsense() {
    let url = "https://boxnovel.com.nonsence/novel/i-might-be-a-fake-cultivator/";

    let expected = false;
    let received = Boxnovel::new().supports_url(url);

    assert_eq!(expected, received, "It should not support this url.");
  }

  #[test]
  fn test_provider_get_name_from_thread() {
    use std::thread;

    let thread = thread::spawn(move || {
      let provider = Boxnovel::new();

      let expected = "boxnovel.com".to_string();
      let received: String = provider.get_name();

      assert_eq!(expected, received, "The provider should be accessible from multiple threads.")
    });
    thread.join()
      .unwrap();
  }


  #[test]
  fn test_boxnovel_get_name() {
    let guinea_pig = Boxnovel::new();

    let expected = "boxnovel.com".to_string();
    let received = guinea_pig.get_name();

    assert_eq!(expected, received, "The names must match!")
  }

  #[test]
  fn test_boxnovel_get_release_link() {
    let guinea_pig = Boxnovel::new();

    let html = r#"
  <div id="oh">
    <ul class="main version-chap">
      <li class="wp-manga-chapter">
        <!--Box Novel put it's latest chapter at the top of the page instead of the bottom-->
        <a href="https://test.com/5">Teste1</a>
        <a href="https://test.com/4">Teste2</a>
        <a href="https://test.com/3">Teste3</a>
        <a href="https://test.com/2">Teste4</a>
        <a href="https://test.com/1">Teste5</a>
      </li>
    </ul>
  </div>
  "#;

    let expected = vec![
      "https://test.com/1".to_string(),
      "https://test.com/2".to_string(),
      "https://test.com/3".to_string(),
      "https://test.com/4".to_string(),
      "https://test.com/5".to_string(),
    ];

    let received = guinea_pig.get_release_links(&html)
      .expect("Could not parse the html to get the release links");

    assert_eq!(expected, received, "The links in the list should match");
  }

  #[test]
  fn test_boxnovel_get_release_link_un_reversed() {
    let guinea_pig = Boxnovel::new();

    let html = r#"
  <div id="oh-no-strip-me">
    <ul class="main version-chap">
      <li class="wp-manga-chapter">
        <!--Box Novel put it's latest chapter at the top of the page instead of the bottom-->
        <a href="https://test.com/1">Teste1</a>
        <a href="https://test.com/2">Teste2</a>
        <a href="https://test.com/3">Teste3</a>
        <a href="https://test.com/4">Teste4</a>
        <a href="https://test.com/5">Teste5</a>
      </li>
    </ul>
  </div>
  "#;

    let expected = vec![
      "https://test.com/1".to_string(),
      "https://test.com/2".to_string(),
      "https://test.com/3".to_string(),
      "https://test.com/4".to_string(),
      "https://test.com/5".to_string(),
    ];

    let received = guinea_pig.get_release_links(&html)
      .expect("Could not parse the html to get the release links");

    assert_ne!(expected, received, "The links in the list should not match because it's not \
    reversed");
  }

  #[test]
  fn test_boxnovel_get_chapter() {
    let guinea_pig = Boxnovel::new();

    let html = r#"
    <html>
      <div class="random">
        <h1>RANDOM</h1>
        <div class="entry-content_wrap">
          <div class="read-container">
            <div class="reading-content">
              <div class="text-left">
                <!-- {{data:*}} -->
                <p>Paragrath 01</p>
                <p>Paragrath 02</p>
                <p>Paragrath 03</p>
                <p>Paragrath 04</p>
                <p>Paragrath 05</p>
              </div>
            </div>
          </div>
        </div>
      </div>
    </html>
  "#;

    let expected =
      "<p>Paragrath 01</p><p>Paragrath 02</p><p>Paragrath 03</p><p>Paragrath 04</p><p>Paragrath 05</p>";

    let received = guinea_pig.get_chapter(&html)
      .expect("Should not have any errors in the parsing!");

    assert_eq!(expected, received, "This test should pass because both results must return true");
  }
}