#![warn(missing_docs)]

//! # String Iterator Library
//!
//! This crate provides substring and character function string iterators.

use std::iter::Iterator;

/// `Substr` is a string substring iterator.
pub struct SubstrIterator<'a> {
    s: &'a str,
    needle: &'a str,
    l: usize,
    needle_len: usize,
    start: usize,
    emit_all: bool,
}

/// `Func` defines the character function iterator trait.
pub trait Substr<'a> {
    /// Returns a character function iterator for the given string and character function.
    fn substr_iter(&'a self, &'a str) -> SubstrIterator<'a>;
}

impl<'a> Substr<'a> for str {
    #[inline]
    fn substr_iter(&'a self, needle: &'a str) -> SubstrIterator<'a> {
        SubstrIterator {
            s: self,
            needle: needle,
            l: self.len(),
            needle_len: needle.len(),
            start: 0,
            emit_all: false,
        }
    }
}

impl<'a> Iterator for SubstrIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start > self.l {
            return None;
        }

        if self.needle_len == 0 {
            return self.next_char();
        }

        if self.l == 0 {
            if !self.emit_all {
                return None;
            }

            self.start = self.l + 1;

            return Some(self.s);
        }

        let mut has_match = false;

        for i in self.start..self.l {
            let end = i + self.needle_len;

            if end > self.l {
                break;
            }

            if &self.s[i..end] != self.needle {
                // Within a value range; continue reading
                if has_match {
                    continue;
                }

                self.start = i;
                has_match = true;

                continue;
            }

            if !self.emit_all && !has_match {
                self.start = i;

                continue;
            }

            // Emit current value
            let v = &self.s[self.start..i];
            self.start = i + self.needle_len;

            return Some(v);
        }

        if !has_match {
            if !self.emit_all || &self.s[self.l - self.needle_len..] != self.needle {
                return None;
            }

            let v = &self.s[self.start..];
            self.start = self.l + 1;

            return Some(v);
        }

        // Emit remaing value
        let v = &self.s[self.start..];
        self.start += self.l;

        Some(v)
    }

    #[inline]
    fn count(self) -> usize {
        let mut v = 0;

        for _ in self {
            v += 1;
        }

        v
    }

    #[inline]
    fn for_each<F>(self, mut f: F)
    where
        F: FnMut(Self::Item),
    {
        for v in self {
            f(v)
        }
    }
}

impl<'a> SubstrIterator<'a> {
    /// Returns an iterator which emits all values; emulating string `split` methods.
    #[inline]
    pub fn all(mut self) -> SubstrIterator<'a> {
        self.emit_all = true;

        self
    }

    #[inline]
    fn next_char(&mut self) -> Option<&'a str> {
        if self.l == 0 || self.start == self.l {
            return None;
        }

        let c = self.s[self.start..].chars().next().unwrap();
        let l = c.len_utf8();
        let end = self.start + l;
        let v = &self.s[self.start..end];

        self.start = end;

        Some(v)
    }

    /// Resets the iterator to the start position.
    #[inline]
    pub fn reset(&mut self) {
        self.start = 0;
    }
}

/// `FuncIterator` is a character function iterator.
pub struct FuncIterator<'a> {
    f: fn(char) -> bool,
    s: &'a str,
    l: usize,
    start: usize,
}

/// `Func` defines the character function iterator trait.
pub trait Func<'a> {
    /// Returns a character function iterator for the given string and character function.
    fn func_iter(&'a self, fn(char) -> bool) -> FuncIterator<'a>;
}

impl<'a> Func<'a> for str {
    #[inline]
    fn func_iter(&'a self, f: fn(char) -> bool) -> FuncIterator<'a> {
        FuncIterator {
            f: f,
            s: self,
            l: self.len(),
            start: 0,
        }
    }
}

/// `Word` defines the words iterator trait.
pub trait Word<'a> {
    /// Returns a words iterator for the given string.
    fn word_iter(&'a self) -> FuncIterator<'a>;
}

#[inline]
fn word_iter<'a>(s: &'a str) -> FuncIterator<'a> {
    s.func_iter(|c: char| !c.is_alphanumeric())
}

impl<'a> Word<'a> for str {
    #[inline]
    fn word_iter(&'a self) -> FuncIterator<'a> {
        word_iter(self)
    }
}

impl<'a> Iterator for FuncIterator<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        if self.start == self.l {
            return None;
        }

        let mut i = self.start;
        let mut has_match = false;

        while i < self.l {
            let c = self.s[i..].chars().next().unwrap();
            let l = c.len_utf8();

            if (self.f)(c) {
                if has_match {
                    let v = &self.s[self.start..i];
                    self.start = i + l;

                    return Some(v);
                }

                self.start = i + l;
                has_match = false;
            } else if !has_match {
                self.start = i;
                has_match = true;
            }

            i += l;
        }

        if has_match && self.start < self.l {
            let v = &self.s[self.start..];
            self.start += self.l - self.start;

            return Some(v);
        }

        None
    }

    #[inline]
    fn count(self) -> usize {
        let mut v = 0;

        for _ in self {
            v += 1;
        }

        v
    }

    #[inline]
    fn for_each<F>(self, mut f: F)
    where
        F: FnMut(Self::Item),
    {
        for v in self {
            f(v)
        }
    }
}

impl<'a> FuncIterator<'a> {
    /// Resets the iterator to the start position.
    #[inline]
    pub fn reset(&mut self) {
        self.start = 0;
    }
}

#[cfg(test)]
mod tests {
    use Func;
    use Substr;
    use Word;

    #[test]
    fn func_count() {
        let c = "Hello 😎 Dennis! 😀"
            .func_iter(|c: char| c < '\u{1F600}' || c > '\u{1F64F}')
            .count();

        assert_eq!(2, c);
    }

    #[test]
    fn substr_count() {
        let c = "hello  world".substr_iter(" ").count();

        assert_eq!(2, c);
    }

    #[test]
    fn word_count() {
        let c = "1 2 3 a b c".word_iter().count();

        assert_eq!(6, c);
    }
}
