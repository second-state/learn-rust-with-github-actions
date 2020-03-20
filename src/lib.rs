pub fn say(s: String) -> String {
  let r = String::from("hello ");
  return r + &s;
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
    
    #[test]
    fn say_hello() {
        let result = say("ssvm");
        assert!(result.contains("hello ssvm"));
    }
}
