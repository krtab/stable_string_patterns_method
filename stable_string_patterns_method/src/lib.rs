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
    #[cfg(feature = "1.65")]
    type Split<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split(self, haystack: &str) -> Self::Split<'_>;
    #[cfg(feature = "1.65")]
    type RSplit<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplit(self, haystack: &str) -> Self::RSplit<'_>;
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split_terminator(self, haystack: &str) -> Self::SplitTerminator<'_>;
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator(self, haystack: &str) -> Self::RSplitTerminator<'_>;
    #[cfg(feature = "1.65")]
    type SplitN<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn splitn(self, haystack: &str, n: usize) -> Self::SplitN<'_>;
    #[cfg(feature = "1.65")]
    type RSplitN<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplitn(self, haystack: &str, n: usize) -> Self::RSplitN<'_>;
    #[cfg(feature = "1.65")]
    type Matches<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn matches(self, haystack: &str) -> Self::Matches<'_>;
    #[cfg(feature = "1.65")]
    type RMatches<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rmatches(self, haystack: &str) -> Self::RMatches<'_>;
    #[cfg(feature = "1.65")]
    type MatchIndices<'a>: Iterator<Item = (usize, &'a str)>;
    #[cfg(feature = "1.65")]
    fn match_indices(self, haystack: &str) -> Self::MatchIndices<'_>;
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a>: Iterator<Item = (usize, &'a str)>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices(self, haystack: &str) -> Self::RMatchIndices<'_>;
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
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split_inclusive(self, haystack: &str) -> Self::SplitInclusive<'_>;
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
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices(self, haystack: &str) -> Self::RMatchIndices<'_> {
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
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive(self, haystack: &str) -> Self::SplitInclusive<'_> {
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
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices(self, haystack: &str) -> Self::RMatchIndices<'_> {
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
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive(self, haystack: &str) -> Self::SplitInclusive<'_> {
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

impl Sealed for &[char] {}

impl Searchable for &[char] {
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
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices(self, haystack: &str) -> Self::RMatchIndices<'_> {
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
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
}

impl DoubleEndedSearchable for &[char] {
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
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices(self, haystack: &str) -> Self::RMatchIndices<'_> {
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
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive(self, haystack: &str) -> Self::SplitInclusive<'_> {
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
