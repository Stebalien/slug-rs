extern crate unidecode;

use unidecode::unidecode_char;
use std::ascii::AsciiExt;

/// Convert any unicode string to an ascii "slug" (useful for file names/url components)
///
/// The returned "slug" will consist of a-z, 0-9, and '-'. Furthermore, a slug will
/// never contain more than one '-' in a row and will never start or end with '-'.
///
/// ```rust
/// use self::slug::slugify;
///
/// assert_eq!(slugify("My Test String!!!1!1"), "my-test-string-1-1");
/// assert_eq!(slugify("test\nit   now!"), "test-it-now");
/// assert_eq!(slugify("  --test_-_cool"), "test-cool");
/// assert_eq!(slugify("Æúű--cool?"), "aeuu-cool");
/// assert_eq!(slugify("You & Me"), "you-me");
/// assert_eq!(slugify("user@example.com"), "user-example-com");
/// ```
pub fn slugify<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut slug = Vec::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: char| {
            match x {
                'a'...'z' | '0'...'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                },
                'A'...'Z' => {
                    prev_is_dash = false;
                    // Manual lowercasing as Rust to_lowercase() is unicode
                    // aware and therefore much slower
                    slug.push(((x as u8) - ('A' as u8) + ('a' as u8)) as char);
                },
                _ => {
                    if !prev_is_dash {
                        slug.push('-');
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c);
            } else {
                for cx in unidecode_char(c).chars() {
                    (push_char)(cx);
                }
            }
        }
    }

    // Remove trailing / if there is one
    match slug.pop() {
        Some(c) => { if c != '-' { slug.push(c); } },
        None => ()
    }

    slug.into_iter().collect::<String>()
}
