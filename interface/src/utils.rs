pub fn parse_code(code: &str) -> Vec<String> {
    let mut quote = false;
    let mut escaped = false;
    let mut entries: Vec<String> = Vec::new();
    let mut recorded: Vec<char> = Vec::new();

    for ch in code.chars() {
        if ch == '\\' && escaped == false {
            escaped = true;
            continue;
        }

        if ch == '{' && quote == false {
            continue;
        } else if ch == '}' && quote == false {
            entries.push(recorded.iter().collect());
        } else if ch == '"' && escaped == false {
            quote = !quote
        } else if ch == ',' && quote == false {
            entries.push(recorded.iter().collect());
            recorded.clear();
        } else {
            recorded.push(ch);
        }

        escaped = false;
    }

    entries
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

    #[test]
    fn test_parse_one_string_with_quote() {
        let code = "{\"hello , world\"}";
        let ans = parse_code(code);
        assert_eq!(ans, vec!["hello , world"]);
    }
}
