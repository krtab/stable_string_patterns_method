mod private {
    pub trait Sealed {}
}
use crate::private::Sealed;

pub trait Searchable: Sealed {
    #[cfg(feature = "1.0")]
    fn contains(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn starts_with(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn ends_with(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn find(self, haystack: &str) -> Option<usize>;
    #[cfg(feature = "1.0")]
    fn rfind(self, haystack: &str) -> Option<usize>;
    #[cfg(feature = "1.0")]
    fn split(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn rsplit(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn split_terminator(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn rsplit_terminator(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn splitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn rsplitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn matches(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn rmatches(self, haystack: &str) -> impl Iterator<Item = &str>;
    #[cfg(feature = "1.0")]
    fn match_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)>;
    #[cfg(feature = "1.0")]
    fn rmatch_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)>;
    #[cfg(feature = "1.30")]
    fn trim_start_matches(self, haystack: &str) -> &str;
    #[cfg(feature = "1.45")]
    fn strip_prefix(self, haystack: &str) -> Option<&str>;
    #[cfg(feature = "1.45")]
    fn strip_suffix(self, haystack: &str) -> Option<&str>;
    #[cfg(feature = "1.0")]
    fn replace(self, haystack: &str, to: &str) -> String;
    #[cfg(feature = "1.0")]
    fn replacen(self, haystack: &str, to: &str, count: usize) -> String;
    #[cfg(feature = "1.51")]
    fn split_inclusive(self, haystack: &str) -> impl Iterator<Item = &str>;
}

pub trait DoubleEndedSearchable: Searchable {
    #[cfg(feature = "1.0")]
    fn trim_matches(self, haystack: &str) -> &str;
    #[cfg(feature = "1.30")]
    fn trim_end_matches(self, haystack: &str) -> &str;
}

impl Sealed for &str {}

impl Searchable for &str {
    #[cfg(feature = "1.0")]
    fn contains(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.0")]
    fn split(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.0")]
    fn split_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn splitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn rsplitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn matches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.0")]
    fn match_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatch_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.51")]
    fn split_inclusive(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_inclusive(self)
    }
}

impl Sealed for char {}

impl Searchable for char {
    #[cfg(feature = "1.0")]
    fn contains(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.0")]
    fn split(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.0")]
    fn split_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn splitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn rsplitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn matches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.0")]
    fn match_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatch_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.51")]
    fn split_inclusive(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_inclusive(self)
    }
}

impl DoubleEndedSearchable for char {
    #[cfg(feature = "1.0")]
    fn trim_matches(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

impl<F> Sealed for F where F: FnMut(char) -> bool {}

impl<F> Searchable for F
where
    F: FnMut(char) -> bool,
{
    #[cfg(feature = "1.0")]
    fn contains(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.0")]
    fn split(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.0")]
    fn split_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn rsplit_terminator(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.0")]
    fn splitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn rsplitn(self, haystack: &str, n: usize) -> impl Iterator<Item = &str> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.0")]
    fn matches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatches(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.0")]
    fn match_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.0")]
    fn rmatch_indices(self, haystack: &str) -> impl Iterator<Item = (usize, &str)> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.51")]
    fn split_inclusive(self, haystack: &str) -> impl Iterator<Item = &str> {
        haystack.split_inclusive(self)
    }
}

impl<F> DoubleEndedSearchable for F
where
    F: FnMut(char) -> bool,
{
    #[cfg(feature = "1.0")]
    fn trim_matches(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}
