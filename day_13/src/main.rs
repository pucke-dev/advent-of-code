use serde_json::{json, Value};
use std::cmp::Ordering;
use std::fs::read_to_string;

const INPUT_FILE_PATH: &str = "day_13/src/input.txt";

fn main() {
    let mut signal = read_input(INPUT_FILE_PATH);
    let pairs: Vec<Vec<Value>> = signal.chunks(2).map(|c| c.to_vec()).collect();

    let mut sum: usize = 0;
    for (index, pair) in pairs.iter().enumerate() {
        let order = compare(&pair[0], &pair[1]);
        if order.is_some() && matches!(order.unwrap(), Ordering::Less) {
            sum += index + 1
        }
    }
    println!("Sum of ordered indices: {}", sum);

    signal.extend([json!([[2]]), json!([[6]])]);
    signal.sort_by(|a, b| compare(a, b).unwrap());

    let dp1 = signal.iter().position(|p| *p == json!([[2]])).unwrap() + 1;
    let dp2 = signal.iter().position(|p| *p == json!([[6]])).unwrap() + 1;
    println!("decoder key: {:?}", dp1 * dp2);
}

fn compare(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Array(a), Value::Array(b)) => {
            if a.is_empty() || b.is_empty() {
                match a.len().cmp(&b.len()) {
                    Ordering::Equal => None,
                    order => Some(order),
                }
            } else if let Some(v) = compare(&a[0], &b[0]) {
                Some(v)
            } else {
                compare(&json!(a[1..]), &json!(b[1..]))
            }
        }
        (Value::Number(a), Value::Array(b)) => compare(&json!(vec![a]), &json!(b)),
        (Value::Array(a), Value::Number(b)) => compare(&json!(a), &json!(vec![b])),
        (Value::Number(a), Value::Number(b)) => match a.as_u64().cmp(&b.as_u64()) {
            Ordering::Equal => None,
            order => Some(order),
        },
        _ => Some(Ordering::Greater),
    }
}

fn read_input(file_name: &str) -> Vec<Value> {
    let s = read_to_string(file_name).unwrap();
    s.lines()
        .filter(|l| !l.is_empty())
        .map(|line| serde_json::from_str(line).unwrap())
        .collect()
}
