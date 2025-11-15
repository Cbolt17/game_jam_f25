pub fn format_money_text(n: i64) -> String {
    let s = n.to_string();
    let mut result = String::new();
    let mut count = 0;

    for (i, c) in s.chars().rev().enumerate() {
        if i > 0 && count % 3 == 0 {
            result.push(',');
        }
        result.push(c);
        count += 1;
    }
    result.push('$');
    result.chars().rev().collect()
}