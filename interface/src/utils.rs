use serde_json::Value;

pub fn parse_code(code: &str) -> Vec<String> {
    println!("{}", code);
    let tmp = "{\"\\\\\\\",\\\"\\\\\"}";
    println!("{}", tmp == code);
    let code = code.trim_end_matches("}").trim_start_matches("{");
    let mut ans = Vec::new();
    let mut start_string = false;
    let mut extra = false;
    let mut pre = ',';
    for ch in code.chars() {
        if ch == '"' && pre != '\\' {
            start_string = !start_string;
        } else if pre == ',' && start_string == false {
            start_string = true;
            extra = true;
            ans.push('"');
        } else if ch == ',' && start_string == true && extra == true {
            start_string = false;
            extra = false;
            ans.push('"');
        }
        ans.push(ch);
        pre = ch;
    }
    if extra {
        ans.push('"');
    }
    let ans: String = ans.iter().collect();
    let code = "[".to_string() + &ans + "]";
    println!("{}", code);
    serde_json::from_str::<Value>(&code)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_code() {
        let code = "{\"hello , world\",quote,\"\\\",\\\"q\"}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["hello , world", "quote", "\",\"q"]);
    }
    #[test]
    fn test_parse_one_elem() {
        let code = "{print(5)}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["print(5)"]);
    }
    #[test]
    fn test_parse_all_single() {
        let code = "{print(5),print(6),print(7)}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["print(5)", "print(6)", "print(7)"]);
    }

    #[test]
    fn test_parse_all_quote() {
        let code = "{\"\\\\\\\\, \\\\\\\\\\\\,\"}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["\\\\, \\\\\\,"]);
    }

    #[test]
    fn test_random_complicated_token() {
        let code = "{\"\\\\\\\",\\\"\\\\\"}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["\\\",\"\\"]);
    }
}
