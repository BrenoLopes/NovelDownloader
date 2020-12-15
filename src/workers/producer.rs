use std::sync::mpsc::Sender;

use crate::workers::types::{Message, MyResult};
use crate::novels::providers::NovelProvider;
use crate::url;

pub fn start_producer<'a>(transmitter: Sender<Message>, url: String, provider: Box<dyn NovelProvider>) {
  let novel_data_html = match get_chapter_html(&url, &transmitter) {
    None => return,
    Some(data) => data
  };

  let novel_data = match get_chapter(novel_data_html, &url, &transmitter, provider) {
    None => return,
    Some(data) => data
  };

  let message = Message {
    result: MyResult::Success,
    url: url.to_string(),
    data: novel_data,
  };

  transmitter.send(message)
    .expect("The producer couldn't send a message to the consumer.");
}

fn get_chapter_html(url: &str, transmitter: &Sender<Message>) -> Option<String> {
  return match url::download(&url) {
    Ok(data) => Some(data),
    Err(_) => {
      let message = Message {
        result: MyResult::Failed,
        url: url.to_string(),
        data: format!("Couldn't download the chapter with link \"{}\".", &url)
      };

      transmitter.send(message)
        .expect("The producer couldn't send a message to the consumer.");

      return None;
    }
  };
}

fn get_chapter(html: String, url: &str, transmitter: &Sender<Message>, provider: Box<dyn NovelProvider>) -> Option<String>{
  return match provider.get_chapter(&html) {
    Some(data) => Some(data),
    None => {
      let message = Message {
        result: MyResult::Failed,
        url: url.to_string(),
        data: format!("Couldn't extract the data from chapter with link \"{}\".", &url)
      };

      transmitter.send(message)
        .expect("The producer couldn't send a message to the consumer.");

      return None;
    }
  };
}