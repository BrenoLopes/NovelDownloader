use simple_disk_cache::SimpleCache;
use std::io::Write;

pub fn novel_to_file(
  link_list: &Vec<String>,
  cache: &mut SimpleCache<String, String>,
  output_file: &str,
  mut had_errors: bool
) -> bool {
  println!("Writing all chapters to output file...");

  let mut f = std::fs::File::create(output_file)
    .expect("Failed to create the output file.");

  for url in link_list {
    let chapter = match cache.get(&url)
      .expect("Failed to read from cache.") {
      None => "".to_string(),
      Some(data) => data,
    };

    match f.write_all(chapter.as_bytes()) {
      Ok(_) => {}
      Err(_) => {
        println!("Failed to write chapter {} into the output file.", url);
        had_errors = true;
      }
    }
  }

  return had_errors;
}

pub fn clean_file(src: &str) {
  let copy_src = format!("{}.txt", src);
  let mut data = std::fs::read_to_string(&copy_src).unwrap();

  // Removes a single or multiple empty lines
  let re_empty_lines = regex::Regex::new(r"(?:[\t ]*(?:\r?\n|\r))+").unwrap();
  data = re_empty_lines.replace_all(&data, "\n").to_string();

  // Format all chapters as h1 for make it easy for calibre to map the chapters
  let re_format_chapter = regex::Regex::new(r"Chapter\s0*(\d+)[:\s-]+(.+)")
    .unwrap();
  data = re_format_chapter.replace_all(&data, "\n# Chapter $1: $2")
    .to_string();

  // Overwrite the src file
  std::fs::write(&copy_src, data).unwrap();
}