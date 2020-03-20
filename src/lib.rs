pub fn say(s: &str) -> String {
  let r = String::from("hello ");
  return r + s;
}

#[cfg(test)]
mod tests {
  use super::*;
  
  #[test]
  fn say_hello() {
    let result = say("ssvm");
    assert!(result.contains("hello ssvm"));
  }
}
