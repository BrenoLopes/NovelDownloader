use easy_scraper::Pattern;

use crate::novels::providers::NovelProvider;

use regex::Regex;

#[derive(Copy, Clone)]
pub struct EatApplePies {}

impl EatApplePies {
  pub fn new() -> EatApplePies { return EatApplePies{}; }
}

impl NovelProvider for EatApplePies
{
  fn supports_url(&self, url: &str) -> bool {
    let re = Regex::new(r"^(https?://)?([\s\S]*\.)?eatapplepies.com(/[\s\S]*)?$")
      .expect("Could not parse and load the regex engine.");

    return re.is_match(url);
  }

  fn get_name(&self) -> String {
    return "eatapplepies.com".to_string();
  }

  fn get_release_links(&self, html: &str) -> Result<Vec<String>, &str> {
    let mut novel_links: Vec<String> = Vec::with_capacity(300);

    let pattern = match Pattern::new(r#"
    <div class="entry-content">
      <details>
        <p>
          <a href="{{url}}">{{text}}</a>
        </p>
      </details>
    </div>
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

    return Ok(novel_links);
  }

  fn get_chapter(&self, html: & str) -> Option<String> {
    let pattern = Pattern::new(r#"
      <div id="primary">
        <div id="content" role="main">
          <article class="post">
            <header class="entry-header">
              <h1 class="entry-title">{{title}}</h1>
            </header>
            <div class="entry-content">
              <p>{{body}}</p>
            </div>
          </article>
        </div>
      </div>
    "#).expect("Could not load the pattern.");

    let matches = pattern.matches(html);

    let mut data = String::from("");

    match matches[0].get("title") {
      None => {},
      Some(d) => {
        let re = regex::Regex::new(r".+Chapter\s0*(\d+)[\s:–-]+(.+)")
          .unwrap();
        let new_data =
          format!("{}\n\n",re.replace(d, "<h1>Chapter $1: $2</h1>").to_string());
        data.push_str(&new_data);
      }
    }

    let invalid_html = vec![
      "No account yet?",
      "Register",
      "Username or Email Address",
      "Password",
      "Remember Me",
    ];

    for m in matches {
      if let Some(m) = m.get("body") {
        if invalid_html.contains(&&**m) {
          continue;
        }

        let paragraph = format!("<p>{}</p>\n", m);
        data.push_str(&paragraph);
      }
    }

    // println!("\nDebug");
    // println!(data);
    // println!("Debug\n");

    return Some(data);
  }
}

#[cfg(test)]
mod tests {
  use crate::novels::eatapplepies::EatApplePies;
  use crate::novels::providers::NovelProvider;
  use crate::url;
  use std::io::Write;

  #[test]
  fn test_main() {
    let url1 = "https://eatapplepies.com/tcf-chapter-1/";
    let url2 = "https://eatapplepies.com/tcf-chapter-2/";

    let html1 = url::download(url1).unwrap();
    let html2 = url::download(url2).unwrap();

    let parser = EatApplePies::new();

    let chapter1 = parser.get_chapter(&html1).unwrap();
    let chapter2 = parser.get_chapter(&html2).unwrap();

    let mut f = std::fs::File::create("AFF.html").unwrap();
    f.write(chapter1.as_bytes()).unwrap();
    f.write("\n\n".as_bytes()).unwrap();
    f.write(chapter2.as_bytes()).unwrap();
  }

  #[test]
  fn test_supports_url_simple() {
    let url = "https://eatapplepies.com";

    let expected = true;
    let received = EatApplePies::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_with_domain() {
    let url = "https://test.eatapplepies.com";

    let expected = true;
    let received = EatApplePies::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_supports_url_long() {
    let url = "https://eatapplepies.com/trash-of-the-counts-family/table-of-contents/";

    let expected = true;
    let received = EatApplePies::new().supports_url(url);

    assert_eq!(expected, received, "It should support this url.");
  }

  #[test]
  fn test_not_supports_url_nonsense() {
    let url = "https://eatapplepies.com.nonsence/trash-of-the-counts-family/table-of-contents/";

    let expected = false;
    let received = EatApplePies::new().supports_url(url);

    assert_eq!(expected, received, "It should not support this url.");
  }

  #[test]
  fn test_provider_get_name_from_thread() {
    use std::thread;

    let thread = thread::spawn(move || {
      let provider = EatApplePies::new();

      let expected = "eatapplepies.com".to_string();
      let received: String = provider.get_name();

      assert_eq!(expected, received, "The provider should be accessible from multiple threads.")
    });
    thread.join()
      .unwrap();
  }

  #[test]
  fn test_eatapplepies_get_name() {
    let guinea_pig = EatApplePies::new();

    let expected = "eatapplepies.com".to_string();
    let received = guinea_pig.get_name();

    assert_eq!(expected, received, "The names must match!")
  }

  #[test]
  fn test_eatapplepies_get_release_link() {
    let guinea_pig = EatApplePies::new();

    let html = r#"
    <div class="entry-content">
      <details>
        <summary>Chapters 1-100</summary>
        <p>
          <a href="https://eatapplepies.com/tcf-chapter-1/" target="_blank" rel="noopener noreferrer"> &#8212; Chapter 1: Prologue</a><br />
          <a href="https://eatapplepies.com/tcf-chapter-2/" target="_blank" rel="noopener noreferrer"> &#8212; Chapter 2: When I Opened My Eyes (1)</a><br />
        </p>
      </details>
      <details>
        <summary>Chapters 1-100</summary>
        <p>
          <a href="https://eatapplepies.com/tcf-chapter-101/" target="_blank" rel="noopener noreferrer"> &#8212; Chapter 101: It’s real (1)</a><br />
          <a href="https://eatapplepies.com/tcf-chapter-102/" target="_blank" rel="noopener noreferrer"> &#8212; Chapter 102: It’s real (2)</a><br />
        </p>
      </details>
    </div>
  "#;

    let expected = vec![
      "https://eatapplepies.com/tcf-chapter-1/".to_string(),
      "https://eatapplepies.com/tcf-chapter-2/".to_string(),
      "https://eatapplepies.com/tcf-chapter-101/".to_string(),
      "https://eatapplepies.com/tcf-chapter-102/".to_string(),
    ];

    let received = guinea_pig.get_release_links(&html)
      .expect("Could not parse the html to get the release links");

    assert_eq!(expected, received, "The links in the list should match");
  }

  #[test]
  fn test_eatapplepies_get_chapter() {
    let guinea_pig = EatApplePies::new();

    let html = r#"
      <html>
        <div id="primary">
          <div id="content" role="main">
            <nav id="nav-single">
              <h3 class="assistive-text">Post navigation</h3>
              <span class="nav-previous"></span>
              <span class="nav-next"><a href="https://eatapplepies.com/tcf-chapter-2/" rel="next">Next <span class="meta-nav">&rarr;</span></a></span>
            </nav><!-- #nav-single -->
            <article id="post-11" class="post-11 post type-post status-publish format-standard has-post-thumbnail hentry category-tcf">
              <header class="entry-header">
                <h1 class="entry-title">Trash of the Count&#8217;s Family &#8211; Chapter 1 &#8211; Prologue</h1>
                <div class="entry-meta">
                  <span class="sep">Posted on </span><a href="https://eatapplepies.com/tcf-chapter-1/" title="9:24 pm" rel="bookmark"><time class="entry-date updated" datetime="2020-11-03T21:24:24+00:00" pubdate>November 3, 2020</time></a><span class="by-author"> <span class="sep"> by </span> <span class="author vcard"><a class="url fn n" href="https://eatapplepies.com" title="View all posts by admin" rel="author">admin</a></span></span>                                    <span class="sep"> &mdash; </span>
                  <span class="comments-link">
                  <a href="https://eatapplepies.com/tcf-chapter-1/#comments">9 Comments &darr;</a>                    </span>
                </div><!-- .entry-meta -->
              </header><!-- .entry-header -->
              <div class="entry-content">
                <p>When I opened my eyes, I was inside a novel.</p>
                <p>[The Birth of a Hero].</p>
                <p>[The Birth of a Hero] was a novel focused on the adventures of the main character, Choi Han, a high school boy who was transported to a different dimension from Earth, along with the birth of the numerous heroes of the continent.</p>
              </div>
            </article>
          </div>
        </div>
      </html>
    "#;

    let expected =
      "<h1>Chapter 1: Prologue</h1>\n\n<p>When I opened my eyes, I was inside a novel.</p>\n<p>[The Birth of a Hero].</p>\n<p>[The Birth of a Hero] was a novel focused on the adventures of the main character, Choi Han, a high school boy who was transported to a different dimension from Earth, along with the birth of the numerous heroes of the continent.</p>\n";

    let received = guinea_pig.get_chapter(&html)
      .expect("Should not have any errors in the parsing!");

    assert_eq!(expected, received, "This test should pass because both results must return true");
  }
}