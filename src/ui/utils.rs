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

pub fn format_time(minutes: f32) -> String {
    let total_seconds = (minutes * 60.0).floor() as u64;
    let mins = total_seconds / 60;
    let secs = total_seconds % 60;
    format!("Time Remaining: {:02}:{:02}", mins, secs)
}
