use std::fmt::Display;

/// Return the formatted vec, putting sep between each values
///
/// # Example
/// ```
/// let v = vec![1, 2, 3];
/// let s = list_str(v, ", "); // => "1, 2, 3"
/// ```
pub fn list_str<E: Display>(vec: &Vec<E>, sep: &str) -> String {
    let n = vec.len();
    let mut res = String::new();
    for (i, e) in vec.iter().enumerate() {
        res.push_str(&format!("{e}"));
        if i < n - 1 { res.push_str(sep); }
    }
    res
}