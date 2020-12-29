use crate::workers::types::{Message, MyResult};

use std::sync::mpsc::Receiver;
use simple_disk_cache::SimpleCache;

pub fn start_consumer<'a>(
  receiver: &'a Receiver<Message>,
  cache: &'a mut SimpleCache<String, String>,
  mut novel_counter: u32,
  n: usize
) -> bool {
  let mut had_errors = false;

  loop {
    if novel_counter >= n as u32 {
      break;
    }

    let message = receiver.recv()
      .expect("The application could not create a valid channel");

    match message.result {
      MyResult::Success => {
        cache.put(&message.url, &message.data)
          .expect("Couldn't write to cache");

        println!("Downloaded chapter {} of {}...", novel_counter, n - 1);
        novel_counter += 1;
      },
      MyResult::Failed => {
        cache.put(&message.url, &"".to_string())
          .expect("Couldn't write to cache");

        println!("Failed to download chapter {} of {}. Error: {}...", novel_counter, n - 1, message.data);
        novel_counter += 1;
        had_errors = true;
      }
    }
  }

  return had_errors;
}