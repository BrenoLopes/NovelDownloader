use std::fs;
use std::process::Command;

pub fn can_find_calibre_convert(calibre_dir: &str) -> bool {
  let file_exists = fs::metadata(calibre_dir);

  return match file_exists {
    Ok(_) => true,
    Err(_) => false
  };
}

pub fn delegate_to_calibre(output: &str, calibre_dir: &str) -> Result<String, String> {
  let current_dir = std::env::current_dir().unwrap().display().to_string();
  let output = format!("{}\\{}", current_dir, output);

  let program_query = format!("{}\\ebook-convert", calibre_dir);
  let arg1 = format!("{}.html", output);
  let arg2 = format!("{}.txt", output);
  let arg3 = "--unsmarten-punctuation";

  let output_command = Command::new(program_query)
    .arg(arg1)
    .arg(arg2)
    .arg(arg3)
    .stdout(std::process::Stdio::inherit())
    .output()
    .expect("Failed to use calibre to parse the results.");

  if output_command.status.success() {
    return Ok("Calibre was finished successfully".to_string());
  } else {
    return Err("Calibre was not successful while parsing.".to_string());
  }
}

#[cfg(test)]
mod tests {
  use crate::calibre::can_find_calibre_convert;

  #[test]
  fn can_find_calibre_convert_in_a_valid_directory() {
    let directory = "C:\\Program Files\\Calibre2";

    let expected = true;
    let received = can_find_calibre_convert(directory);

    assert_eq!(expected, received, "This test should find a calibre installation in the directory");
  }

  #[test]
  fn can_not_find_calibre_convert_in_an_invalid_directory() {
    let directory = "C:\\Program Files\\Calibre1231";

    let expected = false;
    let received = can_find_calibre_convert(directory);

    assert_eq!(expected, received, "This test shouldn't find a calibre installation in the \
      directory");
  }
}