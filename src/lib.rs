extern crate deunicode;

use deunicode::deunicode_char;

/// Convert any unicode string to an ascii "slug" (useful for file names/url components)
///
/// The returned "slug" will consist of a-z, 0-9, and '-'. Furthermore, a slug will
/// never contain more than one '-' in a row and will never start or end with '-'.
///
/// ```rust
/// use self::slug::{slugify, Case};
///
/// assert_eq!(slugify("My Test String!!!1!1", "_", None), "My_Test_String_1_1");
/// assert_eq!(slugify("test\nit   now!", "*", Some(Case::Upper)), "TEST*IT*NOW");
/// assert_eq!(slugify("  --TEST-_COOL", "-", Some(Case::Lower)), "test-cool");
/// assert_eq!(slugify("Æúű--cool?", "-", None), "AEuu-cool");
/// assert_eq!(slugify("You & Me", "-", Some(Case::Lower)), "you-me");
/// assert_eq!(slugify("user@example.com", "-", Some(Case::Upper)), "USER-EXAMPLE-COM");
/// assert_eq!(slugify("遊戲", "-", None), "You-Xi");
/// ```

pub enum Case {
    Lower,
    Upper,
}

pub fn slugify<S: AsRef<str>, T: AsRef<str>>(s: S, sep: T, transform: Option<Case>) -> String {
    _slugify(s.as_ref(), sep.as_ref(), transform)
}

// avoid unnecessary monomorphizations
fn _slugify(s: &str, sep: &str, transform: Option<Case>) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    let sep_char: Vec<char> = sep.chars().collect();
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| match x {
            b'a'..=b'z' => {
                prev_is_dash = false;

                if let Some(Case::Upper) = transform {
                    slug.push(x.to_ascii_uppercase());
                } else {
                    slug.push(x);
                }
            }
            b'A'..=b'Z' => {
                prev_is_dash = false;
                if let Some(Case::Lower) = transform {
                    slug.push(x.to_ascii_lowercase());
                } else {
                    slug.push(x);
                }
            }
            b'0'..=b'9' => {
                prev_is_dash = false;
                slug.push(x);
            }
            _ => {
                if !prev_is_dash {
                    slug.push(sep_char[0] as u8);
                    prev_is_dash = true;
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or(sep).as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    if string.ends_with(sep) {
        string.pop();
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}
