use clap::{Arg, App, ArgGroup, Error, ErrorKind};
use std::ffi::OsString;

pub struct CmdLineArgs {
  pub url: String,
  pub output: String,
  pub calibre_dir: String,
}

// Factory that by default, uses the command line args
pub fn load() -> CmdLineArgs {
  let args = std::env::args_os().into_iter();

  return load_from(args).unwrap_or_else(|e| e.exit());
}

// Function with the args exported as dependency to easily test.
fn load_from<I, T>(args: I) -> Result<CmdLineArgs, clap::Error>
  where
    I: Iterator<Item = T>,
    T: Into<OsString> + Clone,{

  let app_matches = App::new("Novel Downloader")
    .version("1.0.1")
    .author("Breno P. <brenolprimo@gmail.com>")
    .long_about("This application downloads all the chapters from a novel's table of content \
      page and put it all inside a single file. You can then create an ebook format with it later \
      by using it as input in calibre and modifying the look and feel based in your preferences.")
    .arg(Arg::with_name("url")
      .short("u")
      .long("url")
      .value_name("URL")
      .help("The novel's table of content url")
      .takes_value(true))
    .arg(Arg::with_name("output")
      .short("o")
      .long("output")
      .value_name("FILE")
      .required(false)
      .default_value("novel.txt")
      .help("The name of the output file")
      .takes_value(true))
    .arg(Arg::with_name("calibre-dir")
      .short("c")
      .long("calibre-dir")
      .value_name("DIRECTORY")
      .default_value("C:\\Program Files\\Calibre2")
      .help("The directory of you installed calibre")
      .takes_value(true))
    .arg(Arg::with_name("supported-websites")
      .long("supported-websites")
      .value_name("BOOLEAN")
      .help("Print all the websites supported by this application")
      .takes_value(false))
    .group(ArgGroup::with_name("default")
      .arg("url")
      .arg("output")
      .arg("calibre-dir")
      .multiple(true)
      .required(true))
    .group(ArgGroup::with_name("help")
      .arg("supported-websites"))
    .get_matches_from_safe(args);

  let matches = match app_matches {
    Ok(d) => d,
    Err(e) => return Err(e)
  };

  if matches.is_present("supported-websites") {
    crate::novels::providers::print_supported_websites();

    std::process::exit(0);
  }

  let url = match matches.value_of("url") {
    None => return Err(
      Error::with_description(
        "Failed to read the url arg",
        ErrorKind::ArgumentNotFound)
    ),
    Some(v) => v.to_string()
  };

  let mut output = matches.value_of("output").unwrap();

  let path = std::path::Path::new(output);
  output = path.file_stem().unwrap().to_str().unwrap();

  let calibre_dir = matches.value_of("calibre-dir").unwrap().to_string();
  return Ok(CmdLineArgs { url, output: output.to_string(), calibre_dir });
}

#[cfg(test)]
mod test {
  use crate::cmd_line::load_from;

  #[test]
  fn test_load_success() {
    let args = [
      "noveldownloader.exe",
      "--url",
      "https://www.test.com",
      "--output",
      "test.txt",
      "--calibre-dir",
      "C:\\Program Files\\Calibre2"
    ].iter();

    load_from(args).unwrap();
  }

  #[test]
  fn test_empty() {
    let args = ["noveldownloader.exe"].iter();

    let result = load_from(args);
    match result {
      Ok(_) => assert!(false, "This test should throw an error with no arguments."),
      Err(_) => assert!(true)
    }
  }
}