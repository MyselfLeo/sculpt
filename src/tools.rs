use std::fmt::Display;

/// Return the formatted vec, putting sep between each values
///
/// # Example
/// ```
/// let v = vec![1, 2, 3];
/// let s = list_str(v, ", "); // => "1, 2, 3"
/// ```
pub fn list_str<E: Display>(vec: &Vec<E>, sep: &str) -> String {
    let mut iter = vec.iter().peekable();
    let mut res = String::new();
    for e in iter {
        res.push_str(&format!("{e}"));
        if iter.peek().is_some() {
            res.push_str(sep);
        }
    }
    res
}