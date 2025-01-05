fn fx(input: f64) -> f64 {
    1.0 / input
}

fn main() {
    let intervals: u8 = 100;
    let lower_limit: f64 = 1.0;
    let upper_limit: f64 = 2.0;
    let partition: f64 = (upper_limit - lower_limit) / f64::from(intervals);

    let mut interval_values: [f64; 101] = [0.0; 101];
    for i in 0..interval_values.len() {
        if i == 0 {
            interval_values[i] = lower_limit;
        } else if i == interval_values.len() {
            interval_values[i] = upper_limit;
        } else {
            interval_values[i] = interval_values[i - 1] + partition;
        }
    }

    let mut areas: [f64; 100] = [0.0; 100];
    for i in 0..areas.len() {
        areas[i] = fx((interval_values[i] + interval_values[i + 1]) / (2 as f64)) * partition;
    }

    let mut sum: f64 = 0.0;
    for i in 0..areas.len() {
        sum += areas[i];
    }
    println!("With {} intervals the total area is {}", intervals, sum);
}
