use std::string::ToString;

/// A Matcher is a single rule of fizzbuzz: given a function on T, should
/// a word be substituted in? If yes, which word?
pub struct Matcher<T> {
    f: Box<Fn(T) -> Option<String>>,
}

impl<T> Matcher<T> {
    pub fn new<F: 'static + Fn(T) -> bool, S: 'static + ToString>(matcher: F, sub: S) -> Matcher<T> where {
        Self {
            f: Box::new(move |x| if matcher(x) { Some(sub.to_string()) } else { None })
        }
    }
}

/// A Fizzy is a set of matchers, which may be applied to an iterator.
///
/// Strictly speaking, it's usually more idiomatic to use `iter.map()` than to
/// consume an iterator with an `apply` method. Given a Fizzy instance, it's
/// pretty straightforward to construct a closure which applies it to all
/// elements of the iterator. However, we're using the `apply` pattern
/// here because it's a simpler interface for students to implement.
///
/// Also, it's a good excuse to try out using impl trait.
pub struct Fizzy<T> {
    matchers: Vec<Matcher<T>>,
}

impl<T: ToString + Copy> Fizzy<T> {
    pub fn new() -> Self {
        Self {
            matchers: Vec::new(),
        }
    }

    pub fn add_matcher(mut self, matcher: Matcher<T>) -> Self {
        self.matchers.push(matcher);
        self
    }

    /// map this fizzy onto every element of an interator, returning a new iterator
    pub fn apply<'a>(&'a self, iter: impl Iterator<Item = T> + 'a) -> impl Iterator<Item = String> + 'a {
        iter.map(move |x| {
            let strs = self.matchers.iter().filter_map(|m| (m.f)(x)).collect::<Vec<_>>();
            if strs.is_empty() {
                x.to_string()
            } else {
                strs.join("")
            }
        })
    }
}

/// convenience function: return a Fizzy which applies the standard fizz-buzz rules
pub fn fizz_buzz<T: std::ops::Rem<Output = T> + std::convert::From<u8> + PartialEq + ToString + Copy>() -> Fizzy<T> {
    Fizzy::new()
        .add_matcher(Matcher::new(|n: T| n % T::from(3) == T::from(0), "fizz"))
        .add_matcher(Matcher::new(|n: T| n % T::from(5) == T::from(0), "buzz"))
}
