use compact_str::CompactString;
use std::{
    borrow::{Borrow, Cow},
    fmt,
    ops::Deref,
};

#[derive(Clone, Eq, Hash)]
#[cfg_attr(feature = "serde", derive(Serialize))]
pub struct Atom(CompactString);

const BASE54_CHARS: &[u8; 64] = b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ$_0123456789";

impl Atom {
    pub const fn new_inline(s: &str) -> Self {
        Self(CompactString::new_inline(s))
    }

    #[inline]
    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }

    pub fn base54(n: usize) -> Self {
        let mut num = n;

        let base = 54usize;
        let mut ret = CompactString::default();
        ret.push(BASE54_CHARS[num % base] as char);
        num /= base;

        let base = 64usize;
        while num > 0 {
            num -= 1;
            ret.push(BASE54_CHARS[num % base] as char);
            num /= base;
        }

        Self(ret)
    }
}

impl Deref for Atom {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        self.0.as_str()
    }
}

impl<'a> From<&'a str> for Atom {
    fn from(s: &'a str) -> Self {
        Self(s.into())
    }
}

impl From<String> for Atom {
    fn from(s: String) -> Self {
        Self(s.into())
    }
}

impl From<Cow<'_, str>> for Atom {
    fn from(s: Cow<'_, str>) -> Self {
        Self(s.into())
    }
}

impl AsRef<str> for Atom {
    #[inline]
    fn as_ref(&self) -> &str {
        self.0.as_str()
    }
}

impl Borrow<str> for Atom {
    #[inline]
    fn borrow(&self) -> &str {
        self.0.as_str()
    }
}

impl<T: AsRef<str>> PartialEq<T> for Atom {
    fn eq(&self, other: &T) -> bool {
        self.0.as_str() == other.as_ref()
    }
}

impl PartialEq<Atom> for &str {
    fn eq(&self, other: &Atom) -> bool {
        *self == other.0.as_str()
    }
}

impl fmt::Debug for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.0.as_str(), f)
    }
}

impl fmt::Display for Atom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self.0.as_str(), f)
    }
}
