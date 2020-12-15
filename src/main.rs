mod cmd_line;
mod url;
mod novels;
mod cache;
mod workers;
mod calibre;
mod files;

fn main() {
  // Must be able to get the command line parameters
  let cmd_args = cmd_line::load();

  // Check for calibre
  if !calibre::can_find_calibre_convert(&cmd_args.calibre_dir) {
    panic!("Please inform a valid calibre installation directory. The app couldn't find software \
      in the given directory");
  }

  // Initializing the provider
  let provider = novels::providers::load(
    &cmd_args.url
  ).expect("Failed to load a provider");

  println!("Loading chapter links...");

  // Downloading the html and parsing it in a list of chapter's links
  let novel_list;
  {
    let novel_list_html = url::download(
      &cmd_args.url
    ).expect("Failed to load chapter's list");

    novel_list = provider.get_release_links(&novel_list_html)
      .expect("Failed to parse the chapter's list");
  }

  // Initializing the cache and checking if the user can read and write to the directory.
  let cache = cache::load()
    .expect("Failed to load the cache");

  println!("Starting download...");
  let output = format!("{}.html", cmd_args.output);
  let had_errors = workers::start_workers(&novel_list, cache, &output);

  match had_errors {
    true => println!("Download finished with errors!"),
    false => println!("Download finished successfully!")
  }

  println!("\n\nStarting calibre\n\n");

  match calibre::delegate_to_calibre(
    &cmd_args.output,
    &cmd_args.calibre_dir
  ) {
    Ok(m) => {
      println!("{}", m);
    },
    Err(m) => panic!(m)
  }

  println!("\n\nCleaning the output file");

  let current_dir = std::env::current_dir().unwrap().display().to_string();
  let output = format!("{}\\{}", current_dir, cmd_args.output);

  files::clean_file(&output);

  // Remove the tmp html file
  let _ = std::fs::remove_file(format!("{}.html", output));

  println!("\n\nThe download was successful. Now you can use the txt file to convert it to the \
    epub format, or any other, using the calibre user interface.");
}
