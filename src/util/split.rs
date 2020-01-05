
/// Split a iterator of pairs into pair of vectors
pub fn split<T, U, I>(xs: I) -> (Vec<T>, Vec<U>) where I : IntoIterator<Item = (T, U)> {
    let mut ts = vec![];
    let mut us = vec![];
    xs.into_iter().for_each(|(t, u)| {
        ts.push(t);
        us.push(u);
    });
    (ts, us)
}
