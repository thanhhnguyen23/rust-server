#[derive(Debug, PartialEq)]
pub enum Method{
  Get,
  Post,
  Uninitialized
}

impl From<&str> for Method{

  fn from(s: &str) -> Method{
    match s {
      "GET" => Method::Get,
      "POST" => Method::Post,
      _ => Method::Uninitialized,
    }
  }

}
// * NOTE: running tests
// scenario1> $ cargo test -p http


#[cfg(test)]
mod tests{
  use super::*;
  #[test]
  fn test_method_into(){
    let m: Method = "GET".into();
    assert_eq!(m, Method::Get);
  }
}

#[derive(Debug, PartialEq)]
pub enum Version{
  V1_1,
  V2_0,
  Uninitialized
}

impl From<&str> for Version{

  fn from(s: &str) -> Version{

    match s{
      "HTTP/1.1" => Version::V1_1,
      _ => Version::Uninitialized,
    }
  }
}

#[test]
fn test_version_into(){
  let m: Version = "HTTP/1.1".into();
  assert_eq!(m, Version::V1_1);
}