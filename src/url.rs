// use std::fs::File;
// use std::io::Write;

pub fn download(url: &str) -> Result<String, String> {
  let response = match reqwest::blocking::get(url) {
    Ok(r) => r,
    Err(_) => {
      let message = format!("Could not connect to the remote address: {}", url);
      return Err(message);
    }
  };

  let status_code = response.status();
  if status_code != 200 {
    let message = format!(
      "An error happened. The request was returned with status code {}", status_code
    );
    return Err(message);
  }

  let body = match response.text() {
    Ok(b) => b,
    Err(_) => return Err("An error happened while processing the data.".to_string()),
  };

  Ok(body)
}

// pub fn download_to_path<'a>(url: &'a str, path: &'a str) -> Result<(), String> {
//   let data = match download(url) {
//     Ok(data) => data,
//     Err(e) => return Err(e),
//   };
//
//   let mut file = match File::create(path) {
//     Ok(f) => f,
//     Err(_) => return Err("Could not create the file!".to_string()),
//   };
//
//   match file.write_all(data.as_bytes()) {
//     Ok(_) => return Ok(()),
//     Err(_) => return Err("Could not write to the file!".to_string()),
//   }
// }
