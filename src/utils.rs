use std::collections::VecDeque;

pub fn percent(numerator: u64, denominator: u64) -> f64 {
    if denominator == 0 {
        return 0.0;
    }

    numerator as f64 / denominator as f64 * 100.0
}

#[inline]
pub fn to_gbs(bytes: u64) -> f64 {
    bytes as f64 / 1024.0 / 1024.0 / 1024.0
}

pub fn push_history(history: &mut VecDeque<f64>, value: f64, max_len: usize) {
    if history.len() == max_len {
        history.pop_front();
    }
    history.push_back(value);
}

pub fn to_points(history: &VecDeque<f64>, max_len: usize) -> Vec<(f64, f64)> {
    let offset = max_len - history.len();

    history
        .iter()
        .enumerate()
        .map(|(idx, value)| ((offset + idx) as f64, *value))
        .collect()
}
