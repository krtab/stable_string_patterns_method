mod private {
    pub trait Sealed {}
}
use crate::private::Sealed;

pub trait Searchable: Sealed {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool;
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize>;
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize>;
    #[cfg(feature = "1.65")]
    type Split<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_>;
    #[cfg(feature = "1.65")]
    type RSplit<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_>;
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_>;
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_>;
    #[cfg(feature = "1.65")]
    type SplitN<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_>;
    #[cfg(feature = "1.65")]
    type RSplitN<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_>;
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)>;
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)>;
    #[cfg(feature = "1.65")]
    type Matches<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_>;
    #[cfg(feature = "1.65")]
    type RMatches<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_>;
    #[cfg(feature = "1.65")]
    type MatchIndices<'a>: Iterator<Item = (usize, &'a str)>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_>;
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a>: Iterator<Item = (usize, &'a str)>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_>;
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str;
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str>;
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str>;
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String;
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String;
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a>: Iterator<Item = &'a str>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_>;
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str;
}

pub trait DoubleEndedSearchable: Searchable {
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str;
}

#[cfg(feature = "1.0")]
impl Sealed for &str {}

#[cfg(feature = "1.0")]
impl Searchable for &str {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl Sealed for char {}

#[cfg(feature = "1.0")]
impl Searchable for char {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl DoubleEndedSearchable for char {
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl Sealed for &[char] {}

#[cfg(feature = "1.0")]
impl Searchable for &[char] {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl DoubleEndedSearchable for &[char] {
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl<F> Sealed for F where F: FnMut(char) -> bool {}

#[cfg(feature = "1.0")]
impl<F> Searchable for F
where
    F: FnMut(char) -> bool,
{
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.0")]
impl<F> DoubleEndedSearchable for F
where
    F: FnMut(char) -> bool,
{
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
}

#[cfg(feature = "1.51")]
impl<const N: usize> Sealed for [char; N] {}

#[cfg(feature = "1.51")]
impl<const N: usize> Searchable for [char; N] {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.51")]
impl<const N: usize> DoubleEndedSearchable for [char; N] {
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
}

#[cfg(feature = "1.51")]
impl<const N: usize> Sealed for &[char; N] {}

#[cfg(feature = "1.51")]
impl<const N: usize> Searchable for &[char; N] {
    #[cfg(feature = "1.0")]
    fn contains_impl(self, haystack: &str) -> bool {
        haystack.contains(self)
    }
    #[cfg(feature = "1.0")]
    fn starts_with_impl(self, haystack: &str) -> bool {
        haystack.starts_with(self)
    }
    #[cfg(feature = "1.0")]
    fn ends_with_impl(self, haystack: &str) -> bool {
        haystack.ends_with(self)
    }
    #[cfg(feature = "1.0")]
    fn find_impl(self, haystack: &str) -> Option<usize> {
        haystack.find(self)
    }
    #[cfg(feature = "1.0")]
    fn rfind_impl(self, haystack: &str) -> Option<usize> {
        haystack.rfind(self)
    }
    #[cfg(feature = "1.65")]
    type Split<'a> = std::str::Split<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_impl(self, haystack: &str) -> Self::Split<'_> {
        haystack.split(self)
    }
    #[cfg(feature = "1.65")]
    type RSplit<'a> = std::str::RSplit<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_impl(self, haystack: &str) -> Self::RSplit<'_> {
        haystack.rsplit(self)
    }
    #[cfg(feature = "1.65")]
    type SplitTerminator<'a> = std::str::SplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_terminator_impl(self, haystack: &str) -> Self::SplitTerminator<'_> {
        haystack.split_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type RSplitTerminator<'a> = std::str::RSplitTerminator<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplit_terminator_impl(self, haystack: &str) -> Self::RSplitTerminator<'_> {
        haystack.rsplit_terminator(self)
    }
    #[cfg(feature = "1.65")]
    type SplitN<'a> = std::str::SplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn splitn_impl(self, haystack: &str, n: usize) -> Self::SplitN<'_> {
        haystack.splitn(n, self)
    }
    #[cfg(feature = "1.65")]
    type RSplitN<'a> = std::str::RSplitN<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rsplitn_impl(self, haystack: &str, n: usize) -> Self::RSplitN<'_> {
        haystack.rsplitn(n, self)
    }
    #[cfg(feature = "1.52")]
    fn split_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.split_once(self)
    }
    #[cfg(feature = "1.52")]
    fn rsplit_once_impl(self, haystack: &str) -> Option<(&str, &str)> {
        haystack.rsplit_once(self)
    }
    #[cfg(feature = "1.65")]
    type Matches<'a> = std::str::Matches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn matches_impl(self, haystack: &str) -> Self::Matches<'_> {
        haystack.matches(self)
    }
    #[cfg(feature = "1.65")]
    type RMatches<'a> = std::str::RMatches<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatches_impl(self, haystack: &str) -> Self::RMatches<'_> {
        haystack.rmatches(self)
    }
    #[cfg(feature = "1.65")]
    type MatchIndices<'a> = std::str::MatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn match_indices_impl(self, haystack: &str) -> Self::MatchIndices<'_> {
        haystack.match_indices(self)
    }
    #[cfg(feature = "1.65")]
    type RMatchIndices<'a> = std::str::RMatchIndices<'a, Self>;
    #[cfg(feature = "1.65")]
    fn rmatch_indices_impl(self, haystack: &str) -> Self::RMatchIndices<'_> {
        haystack.rmatch_indices(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_start_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_start_matches(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_prefix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_prefix(self)
    }
    #[cfg(feature = "1.45")]
    fn strip_suffix_impl(self, haystack: &str) -> Option<&str> {
        haystack.strip_suffix(self)
    }
    #[cfg(feature = "1.0")]
    fn replace_impl(self, haystack: &str, to: &str) -> String {
        haystack.replace(self, to)
    }
    #[cfg(feature = "1.0")]
    fn replacen_impl(self, haystack: &str, to: &str, count: usize) -> String {
        haystack.replacen(self, to, count)
    }
    #[cfg(feature = "1.65")]
    type SplitInclusive<'a> = std::str::SplitInclusive<'a, Self>;
    #[cfg(feature = "1.65")]
    fn split_inclusive_impl(self, haystack: &str) -> Self::SplitInclusive<'_> {
        haystack.split_inclusive(self)
    }
    #[cfg(feature = "1.30")]
    fn trim_end_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_end_matches(self)
    }
}

#[cfg(feature = "1.51")]
impl<const N: usize> DoubleEndedSearchable for &[char; N] {
    #[cfg(feature = "1.0")]
    fn trim_matches_impl(self, haystack: &str) -> &str {
        haystack.trim_matches(self)
    }
}

pub trait IntoSearchable {
    type Search: Searchable;

    fn into_searchable(self) -> Self::Search;
}

impl<S: Searchable> IntoSearchable for S {
    type Search = Self;

    fn into_searchable(self) -> Self {
        self
    }
}

pub struct WhiteSpace;

impl IntoSearchable for WhiteSpace {
    type Search = fn(char) -> bool;

    fn into_searchable(self) -> Self::Search {
        char::is_whitespace
    }
}

pub trait StrPatternExt: std::convert::AsRef<str> {
    fn contains_<P>(&self, pattern: P) -> bool
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().contains_impl(self.as_ref())
    }
    fn starts_with_<P>(&self, pattern: P) -> bool
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().starts_with_impl(self.as_ref())
    }
    fn ends_with_<P>(&self, pattern: P) -> bool
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().ends_with_impl(self.as_ref())
    }
    fn find_<P>(&self, pattern: P) -> Option<usize>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().find_impl(self.as_ref())
    }
    fn rfind_<P>(&self, pattern: P) -> Option<usize>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rfind_impl(self.as_ref())
    }
    fn split_<P>(&self, pattern: P) -> <<P as IntoSearchable>::Search as Searchable>::Split<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().split_impl(self.as_ref())
    }
    fn rsplit_<P>(&self, pattern: P) -> <<P as IntoSearchable>::Search as Searchable>::RSplit<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rsplit_impl(self.as_ref())
    }
    fn split_terminator_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::SplitTerminator<'_>
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .split_terminator_impl(self.as_ref())
    }
    fn rsplit_terminator_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::RSplitTerminator<'_>
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .rsplit_terminator_impl(self.as_ref())
    }
    fn splitn_<P>(
        &self,
        n: usize,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::SplitN<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().splitn_impl(self.as_ref(), n)
    }
    fn rsplitn_<P>(
        &self,
        n: usize,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::RSplitN<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rsplitn_impl(self.as_ref(), n)
    }
    fn split_once_<P>(&self, pattern: P) -> Option<(&str, &str)>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().split_once_impl(self.as_ref())
    }
    fn rsplit_once_<P>(&self, pattern: P) -> Option<(&str, &str)>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rsplit_once_impl(self.as_ref())
    }
    fn matches_<P>(&self, pattern: P) -> <<P as IntoSearchable>::Search as Searchable>::Matches<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().matches_impl(self.as_ref())
    }
    fn rmatches_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::RMatches<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rmatches_impl(self.as_ref())
    }
    fn match_indices_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::MatchIndices<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().match_indices_impl(self.as_ref())
    }
    fn rmatch_indices_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::RMatchIndices<'_>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().rmatch_indices_impl(self.as_ref())
    }
    fn trim_start_matches_<P>(&self, pattern: P) -> &str
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .trim_start_matches_impl(self.as_ref())
    }
    fn strip_prefix_<P>(&self, pattern: P) -> Option<&str>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().strip_prefix_impl(self.as_ref())
    }
    fn strip_suffix_<P>(&self, pattern: P) -> Option<&str>
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().strip_suffix_impl(self.as_ref())
    }
    fn replace_<P>(&self, pattern: P, to: &str) -> String
    where
        P: IntoSearchable,
    {
        pattern.into_searchable().replace_impl(self.as_ref(), to)
    }
    fn replacen_<P>(&self, pattern: P, to: &str, count: usize) -> String
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .replacen_impl(self.as_ref(), to, count)
    }
    fn split_inclusive_<P>(
        &self,
        pattern: P,
    ) -> <<P as IntoSearchable>::Search as Searchable>::SplitInclusive<'_>
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .split_inclusive_impl(self.as_ref())
    }
    fn trim_matches_<P>(&self, pattern: P) -> &str
    where
        P: IntoSearchable,
        P::Search: DoubleEndedSearchable,
    {
        pattern.into_searchable().trim_matches_impl(self.as_ref())
    }
    fn trim_end_matches_<P>(&self, pattern: P) -> &str
    where
        P: IntoSearchable,
    {
        pattern
            .into_searchable()
            .trim_end_matches_impl(self.as_ref())
    }
}

impl StrPatternExt for str {}
