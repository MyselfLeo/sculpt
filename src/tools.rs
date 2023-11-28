use std::fmt::Display;
use std::iter::zip;

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


/// Return the formatted elements of the vec, as two columns
/// which each line of length width (excluding \n).
pub fn in_columns<E: Display>(vec: &Vec<E>, width: usize) -> String {
    const SEP_SIZE: usize = 10;

    if vec.len() == 0 {return String::new()}

    let mut strings: Vec<String> = vec.iter().map(|e| e.to_string()).collect();
    let mut column_length = strings.iter().map(|e| e.len()).max().expect("Iterator was empty");

    let shorten = |s: &mut String, n: usize| {
        if s.len() <= n {return}
        s.truncate(n - 2);
        s.push_str("..");
    };

    // Shorten the strings if required by the requested width
    if width < (column_length * 2 + SEP_SIZE) {
        column_length = (width - SEP_SIZE) / 2;
        strings.iter_mut().for_each(|s| shorten(s, column_length));
    };

    // Split the elements in 2 columns
    let nb = vec.len() / 2;
    let left_col = &strings[0..nb];
    let right_col = &strings[nb..];

    let mut res = String::new();

    for t in GreedyZip::new(left_col, right_col) {
        let line = match t {
            (Some(l), Some(r)) => format!("{1:<0$}{3}{2:<0$}", column_length, l, r, " ".repeat(SEP_SIZE)),
            (Some(l), None) => format!("{1:<0$}", column_length, l),
            (None, Some(r)) => format!("{1:<0$}  {2:<0$}", column_length, "", r),
            _ => unreachable!()
        };

        res.push_str(&line);
        res.push('\n');
    }


    res
}




/// Similar to [std::iter::Zip], but will return `None` ONLY
/// when both iterators return `None`.
pub struct GreedyZip<'a, A: IntoIterator, B: IntoIterator> {
    a: Box<dyn Iterator<Item = A::Item> + 'a>,
    b: Box<dyn Iterator<Item = B::Item> + 'a>
}

impl<'a, A, B> GreedyZip<'a, A, B>
where
    A: IntoIterator + 'a,
    B: IntoIterator + 'a
{
    pub fn new(a: A, b: B) -> GreedyZip<'a, A, B> {
        GreedyZip {
            a: Box::new(a.into_iter()),
            b: Box::new(b.into_iter())
        }
    }
}


impl<'a, A, B> Iterator for GreedyZip<'a, A, B>
where
    A: IntoIterator + 'a,
    B: IntoIterator + 'a
{
    type Item = (Option<A::Item>, Option<B::Item>);

    fn next(&mut self) -> Option<Self::Item> {
        let res = (self.a.next(), self.b.next());
        match res {
            (None, None) => None,
            _ => Some(res)
        }
    }
}