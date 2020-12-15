use simple_disk_cache::SimpleCache;
use simple_disk_cache::config::{CacheConfig, DataEncoding};

pub fn load() -> Option<SimpleCache<String, String>> {
  return load_from("cache");
}

fn load_from(data_dir: &str) -> Option<SimpleCache<String, String>> {
  let hundred_mb = 100 * 1024 * 1024;

  let config = CacheConfig {
    max_bytes: hundred_mb,
    encoding: DataEncoding::Bincode,
    strategy: Default::default(),
    subdirs_per_level: 2
  };

  let cache;
  match SimpleCache::initialize(data_dir, config) {
    Ok(c) => { cache = Some(c) }
    Err(_) => cache = None
  };

  return cache;
}

pub fn in_cache(url: &String, cache: &mut SimpleCache<String, String>) -> bool {
  let has_failed_to_download_before = |value: &str| -> bool {
    return value.eq("");
  };

  // Check if it's in the cache and it's not an empty string
  return match cache.get(&url)
    .expect("Error while reading from cache") {
    None => false,
    Some(value) => {
      !has_failed_to_download_before(&value)
    }
  }
}

#[cfg(test)]
mod tests {
  use crate::cache::{load_from, in_cache};
  use simple_disk_cache::SimpleCache;

  const TEST_CACHE_PATH: &str = "#test123-#";

  fn after_test(path: &str) {
    std::fs::remove_dir_all(path)
      .expect("Couldn't clean the cache created by the testing functions");
  }

  fn before_test(path: &str) -> SimpleCache<String, String> {
    return load_from(path)
      .expect("The cache must be loaded without errors!");
  }

  #[test]
  fn test_cache_without_error() {
    let dir_path = String::from(format!("{}1", &TEST_CACHE_PATH));

    before_test(&dir_path);
    after_test(&dir_path);
  }

  #[test]
  fn test_cache_set_data_and_read() {
    let dir_path = String::from(format!("{}2", &TEST_CACHE_PATH));

    let mut cache = before_test(&dir_path);

    let data = (&"testing".to_string(), &"123456".to_string());

    cache.put(data.0, data.1)
      .expect("The cache must be able to set without errors!");

    let received = cache.get(data.0)
      .expect("The cache must be able to get without errors")
      .expect("The cache must be able to deserialize data without errors");

    after_test(&dir_path);

    assert_eq!(*data.1, received, "The returned data must not be corrupted!");
  }

  #[test]
  fn test_in_cache_with_empty_chapter() {
    let dir_path = String::from(format!("{}3", &TEST_CACHE_PATH));

    let url = "https://test/1234567".to_string();
    let mut cache = before_test(&dir_path);

    // This is the same as the download failing, so it should be marked to be downloaded
    // again
    cache.put(&url, &"".to_string())
      .expect("Couldn't write to cache");

    let expected = false;
    let received = in_cache(&url, &mut cache);

    after_test(&dir_path);

    assert_eq!(expected, received, "The empty str tells that a chapter was not successfully \
    downloaded. So it should return false.");
  }

  #[test]
  fn test_in_cache_with_valid_chapter() {
    let dir_path = String::from(format!("{}4", &TEST_CACHE_PATH));

    let url = "https://test/1234567".to_string();
    let mut cache = before_test(&dir_path);

    cache.put(&url, &"This is a random chapter".to_string())
      .expect("Couldn't write to cache");

    let expected = true;
    let received = in_cache(&url, &mut cache);

    after_test(&dir_path);

    assert_eq!(expected, received, "The non empty str tells that a chapter was successfully \
    downloaded. So it should return true.");
  }

  #[test]
  fn test_in_cache_with_non_cached_chapter() {
    let dir_path = String::from(format!("{}5", &TEST_CACHE_PATH));
    let url = "https://test/12345678".to_string();

    let mut cache = before_test(&dir_path);

    let expected = false;
    let received = in_cache(&url, &mut cache);

    after_test(&dir_path);

    assert_eq!(expected, received, "The non existing str tells that a chapter was not downloaded. \
    So it should return true.")
  }
}
