use aoc::aoc_input;
use regex::Regex;
use serde_json::{json, Value};

fn find_sum(data: &str) -> i64 {
    let re: Regex = Regex::new(r"([-]*[[:digit:]]+)").unwrap();
    re.captures_iter(data)
        .filter_map(|cap| match cap.get(1) {
            Some(n) => Some(n.as_str().parse::<i64>().unwrap()),
            _ => None,
        })
        .sum()
}

fn has_red(val: &Value) -> bool {
    for (_, v) in val.as_object().unwrap_or(json!({}).as_object().unwrap()) {
        match v {
            Value::String(s) => {
                if s == "red" {
                    return true;
                }
            }
            _ => {}
        }
    }
    false
}

fn sum_node(val: &Value) -> i64 {
    let mut sum = 0;

    match val {
        Value::Object(obj) => {
            if !has_red(val) {
                for (_, v) in obj {
                    sum += sum_node(v);
                }
            }
        }
        Value::Array(arr) => {
            for elem in arr {
                sum += sum_node(elem);
            }
        }
        Value::Number(num) => sum += num.as_i64().unwrap(),
        _ => {}
    }
    sum
}

fn main() {
    let data = aoc_input!(2015, 12).unwrap();

    // Part I
    println!("{}", find_sum(&data));

    // Part II
    let json: Value = serde_json::from_str(&data).unwrap();
    println!("{}", sum_node(&json));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_red() {
        let data = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let json = serde_json::from_str(data).unwrap();
        assert!(has_red(&json));
    }

    #[test]
    fn test_has_no_red() {
        let data = r#"[1,"red",5]"#;
        let json = serde_json::from_str(data).unwrap();
        assert!(!has_red(&json));
    }

    #[test]
    fn test_sum_node1() {
        let data = r#"[1,"red",5]"#;
        let json = serde_json::from_str(data).unwrap();
        assert_eq!(sum_node(&json), 6);
    }

    #[test]
    fn test_sum_node2() {
        let data = r#"[1,{"c":"red","b":2},3]"#;
        let json = serde_json::from_str(data).unwrap();
        assert_eq!(sum_node(&json), 4);
    }

    #[test]
    fn test_sum_node3() {
        let data = r#"{"d":"red","e":[1,2,3,4],"f":5}"#;
        let json = serde_json::from_str(data).unwrap();
        assert_eq!(sum_node(&json), 0);
    }
}
