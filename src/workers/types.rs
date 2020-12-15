pub enum MyResult { Success, Failed }

pub struct Message {
  pub result: MyResult,
  pub url: String,
  pub data: String,
}