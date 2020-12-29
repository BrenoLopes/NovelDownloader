use easy_scraper::Pattern;

use crate::novels::providers::NovelProvider;

use regex::Regex;

#[derive(Copy, Clone)]
pub struct WuxiaWorldCo {}

impl WuxiaWorldCo {
  pub fn new() -> WuxiaWorldCo { return WuxiaWorldCo{}; }
}

impl NovelProvider for WuxiaWorldCo
{
  fn supports_url(&self, url: &str) -> bool {
    let re = Regex::new(r"^(https?://)?([\s\S]*\.)?wuxiaworld.co(/[\s\S]*)?$")
      .expect("Could not parse and load the regex engine.");

    return re.is_match(url);
  }

  fn get_name(&self) -> String {
    return "wuxiaworld.co".to_string();
  }

  fn get_release_links(&self, html: &str) -> Result<Vec<String>, &str> {
    let mut novel_links: Vec<String> = Vec::with_capacity(300);

    let pattern = match Pattern::new(r#"
      <ul class="chapter-list clearfix">
        <a class="chapter-item" href="{{url}}">
          {{text}}
        </a>
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

      let mut new_link = String::from("https://www.wuxiaworld.co");
      new_link.push_str(link);

      novel_links.push(new_link);
    }

    // Remove empty links
    novel_links.retain( |e| ![""].contains(&e.as_str()) );

    return Ok(novel_links);
  }

  fn get_chapter(&self, html: & str) -> Option<String> {
    let pattern1 = Pattern::new(r#"
      <section class="section">
        <h1 class="chapter-title">{{title}}</h1>
      </section>
    "#).expect("Could not load the pattern one.");

    let matches = pattern1.matches(html);

    if matches.len() < 1 {
      return None;
    }
    let title = match matches[0].get("title") {
      None => return None,
      Some(t) => t
    };

    let pattern2 = Pattern::new(r#"
      <section class="section">
        <div class="chapter-entity" id="chapter-entity">
          {{data:*}}
        </div>
      </section>
    "#).expect("Could not load the pattern one.");

    let matches = pattern2.matches(html);

    if matches.len() < 1 {
      return None;
    }
    let data = match matches[0].get("data") {
      None => return None,
      Some(d) => {
        let mut tmp = d.replace(title, "");
        tmp.push('\n');

        tmp
      }
    };

    let title = format!("<h1>{}</h1>", title);
    let mut chapter = String::from(title);
    chapter.push('\n');
    chapter.push_str(&data);

    return Some(chapter);
  }
}

#[cfg(test)]
mod tests {
  use crate::novels::providers::{NovelProvider};
  use crate::novels::wuxiaworldco::WuxiaWorldCo;

  #[test]
  fn test_supports_url_simple() {
    let url = "https://www.wuxiaworld.co";

    let expected = true;
    let received = WuxiaWorldCo::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_with_domain() {
    let url = "https://test.wuxiaworld.co";

    let expected = true;
    let received = WuxiaWorldCo::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_long() {
    let url = "https://wuxiaworld.co/novel/i-might-be-a-fake-cultivator/";

    let expected = true;
    let received = WuxiaWorldCo::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_not_supports_url_nonsense() {
    let url = "https://wuxiaworld.co.nonsence/novel/i-might-be-a-fake-cultivator/";

    let expected = false;
    let received = WuxiaWorldCo::new().supports_url(url);

    assert_eq!(expected, received, "It should not support this url.");
  }

  #[test]
  fn test_provider_get_name_from_thread() {
    use std::thread;

    let thread = thread::spawn(move || {
      let provider = WuxiaWorldCo::new();

      let expected = "wuxiaworld.co".to_string();
      let received: String = provider.get_name();

      assert_eq!(expected, received, "The provider should be accessible from multiple threads.")
    });
    thread.join()
      .unwrap();
  }

  #[test]
  fn test_wuxiaworldco_get_name() {
    let guinea_pig = WuxiaWorldCo::new();

    let expected = "wuxiaworld.co".to_string();
    let received = guinea_pig.get_name();

    assert_eq!(expected, received, "The names must match!")
  }

  #[test]
  fn test_boxnovel_get_release_link() {
    let guinea_pig = WuxiaWorldCo::new();

    let html = r#"
      <div id="oh">
        <ul class="chapter-list clearfix">
          <a class='chapter-item' href="/Rebirth-of-the-Thief-Who-Roamed-the-World/1020876.html">
            <div class='chapter-info'>
              <p class='chapter-name'>Chapter 1 – Rebirth</p>
            </div>
          </a>
          <a class='chapter-item' href="/Rebirth-of-the-Thief-Who-Roamed-the-World/1020877.html">
            <div class='chapter-info'>
              <p class='chapter-name'>Chapter 2 – To Meet Once Again</p>
            </div>
          </a>
          <a class='chapter-item' href="/Rebirth-of-the-Thief-Who-Roamed-the-World/1020878.html">
            <div class='chapter-info'>
              <p class='chapter-name'>Chapter 3 – Haggling</p>
            </div>
          </a>
      </div>
      "#;

    let expected = vec![
      "https://www.wuxiaworld.co/Rebirth-of-the-Thief-Who-Roamed-the-World/1020876.html".to_string(),
      "https://www.wuxiaworld.co/Rebirth-of-the-Thief-Who-Roamed-the-World/1020877.html".to_string(),
      "https://www.wuxiaworld.co/Rebirth-of-the-Thief-Who-Roamed-the-World/1020878.html".to_string(),
    ];

    let received = guinea_pig.get_release_links(&html)
      .expect("Could not parse the html to get the release links");

    assert_eq!(expected, received, "The links in the list should match");
  }

  #[test]
  fn test_boxnovel_get_chapter() {
    let guinea_pig = WuxiaWorldCo::new();

    let html = r#"
    <html>
      <div class="section-list" id="section-list-wp">
        <section class="section">
          <h1 class="chapter-title" style="margin-bottom: 10px;">Chapter 1: I'm Bai Xiaochun</h1>
            <div class="chapter-entity" style="margin-top: 0px;" id="chapter-entity">
              Mount Hood lay in the Eastwood Mountain Range, and at its base was a quaint little village. The villagers there lived off the land, and didn’t have much to do with the outside world.<br><br>
            </div>
        </section>
      </div>
    </html>
  "#;

    let expected =
      "Chapter 1: I\'m Bai Xiaochun\nMount Hood lay in the Eastwood Mountain Range, and at its base was a quaint little village. The villagers there lived off the land, and didn’t have much to do with the outside world.<br><br>";

    let received = guinea_pig.get_chapter(&html)
      .expect("Should not have any errors in the parsing!");

    assert_eq!(expected, received, "This test should pass because both results must return true");
  }
}
