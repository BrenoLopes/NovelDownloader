mod types;
mod consumer;
mod producer;

use crate::workers::types::{Message};
use crate::cache::in_cache;
use crate::novels::providers::load;
use crate::workers::consumer::start_consumer;
use crate::workers::producer::start_producer;
use crate::files::novel_to_file;

use simple_disk_cache::SimpleCache;
use std::sync::mpsc::{Sender, Receiver, channel};

pub fn start_workers(
  link_list: &Vec<String>, mut cache: SimpleCache<String, String>, output_file: &str
) -> bool {
  // Initializing the counter to show the progress
  let mut counter = 1;
  let n = link_list.len();

  // Transmitting channel
  let (transmitter, receiver): (Sender<Message>, Receiver<Message>) = channel();

  // Create the thread poll
  let thread_pool = threadpool::ThreadPool::new(num_cpus::get() * 2);

  for url in link_list {
    if in_cache(url, &mut cache) {
      println!("Loaded chapter {} of {} from the cache!", counter, n - 1);
      counter += 1;
      continue;
    }

    // Copy the url to move it to the thread
    let url = url.to_string();

    // Create a copy of the transmitter for the producer to signal completion
    let tx = transmitter.clone();

    // Start producer
    thread_pool.execute(move || {
      let provider = load(&url)
        .unwrap();

      start_producer(tx, url, provider);
    });
  }

  // Flag to check if any chapter failed to download
  let had_errors = start_consumer(&receiver, &mut cache, counter, n);

  // Save everything into a single file
  return novel_to_file(&link_list, &mut cache, output_file, had_errors);
}
