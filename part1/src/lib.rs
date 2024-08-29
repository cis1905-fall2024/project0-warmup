/// Returns the sqrt of `n`
///
/// Using built-in square root functions is not allowed.
pub fn sqrt(n: u32) -> u32 {
    todo!();
}

/// Consumes a sorted list of integers and a query integer. Returns the index of the query integer
///
/// Note that a 3-valued comparison between numbers `a` and `b` can be done easily:
/// ```rust,ignore
/// match a.cmp(&b) {
///    std::cmp::Ordering::Less => { ... },
///    std::cmp::Ordering::Greater => { ... },
///    std::cmp::Ordering::Equal => { ... },
/// }
/// ```
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32> {
    todo!();
}

/// Consumes a list of numbers representing daily rainfall. The list may contain -999 signifying
/// the end of data of interest. Returns the average of non-negative values up to the first
/// occurrence of -999 (if it occurs). There may be negative numbers other -999 in the list.
/// Returns None if the average is incomputable.
///
/// example: rainfall([6, 8, -1, 1, -999, 4, 5, 6]) -> Some(5.0)
pub fn rainfall(values: &[i32]) -> Option<f64> {
    todo!();
}
