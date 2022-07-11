extern crate deunicode;

use deunicode::deunicode_char;

/// Convert any unicode string to an ascii "slug" (useful for file names/url components)
///
/// The returned "slug" will consist of a-z, 0-9, and '-'. Furthermore, a slug will
/// never contain more than one '-' in a row and will never start or end with '-'.
///
/// ```rust
/// use self::slug::slugify;
///
/// assert_eq!(slugify("My Test String!!!1!1", "_"), "my_test_string_1_1");
/// assert_eq!(slugify("test\nit   now!", "*"), "test*it*now");
/// assert_eq!(slugify("  --test_-_cool", "-"), "test-cool");
/// assert_eq!(slugify("Æúű--cool?", "-"), "aeuu-cool");
/// assert_eq!(slugify("You & Me", "-"), "you-me");
/// assert_eq!(slugify("user@example.com", "-"), "user-example-com");
/// ```
pub fn slugify<S: AsRef<str>, T: AsRef<str>>(s: S, sep: T) -> String {
    _slugify(s.as_ref(), sep.as_ref())
}

// avoid unnecessary monomorphizations
fn _slugify(s: &str, sep: &str) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    let sep_char: Vec<char> = sep.chars().collect();
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    // Manual lowercasing as Rust to_lowercase() is unicode
                    // aware and therefore much slower
                    slug.push(x - b'A' + b'a');
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(sep_char[0] as u8);
                        prev_is_dash = true;
                    }
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
