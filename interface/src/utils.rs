use serde_json::Value;

pub fn parse_code(code: &str) -> Vec<String> {
    let code = code.trim_end_matches("}").trim_start_matches("{");
    let mut ans = Vec::new();
    let mut start_string = false;
    let mut extra = false;
    let mut pre = '\0';
    for ch in code.chars() {
        if ch == '"' && pre != '\\' {
            start_string = !start_string;
        }
        if pre == ',' && start_string == false {
            start_string = true;
            extra = true;
            ans.push('"');
        }
        if ch == ',' && start_string == true && extra == true {
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
    serde_json::from_str::<Value>(&code)
        .unwrap()
        .as_array()
        .unwrap()
        .iter()
        .map(|x| x.as_str().unwrap().to_string())
        .collect()
}
