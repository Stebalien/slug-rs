use deunicode::deunicode_char;
#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::*;

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
    let opts = Options::default();
    slugify_with_options(s.as_ref(), &opts)
}

#[doc(hidden)]
#[cfg(target_family = "wasm")]
#[wasm_bindgen(js_name = slugify)]
pub fn slugify_owned(s: String) -> String {
    slugify(s.as_ref())
}

#[derive(Clone, Debug)]
pub struct Options {
    pub separator: char,
}

impl Default for Options {
    fn default() -> Self {
        Self { separator: '-' }
    }
}

impl Options {
    /// Sets the delimiter used for slugging
    pub fn with_separator(mut self, separator: char) -> Self {
        self.separator = separator;
        self
    }
}

/// Convert any unicode string to an ascii "slug" with some options
///
/// ```rust
/// use self::slug::{slugify_with_options, Options};
///
/// let opts = Options::default().with_separator('_');
///
/// assert_eq!(slugify_with_options("My Test String!!!1!!!!1", &opts), "my_test_string_1_1");
/// ```
pub fn slugify_with_options(s: &str, opts: &Options) -> String {
    let mut slug = String::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    slug.push(x.into());
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    // Manual lowercasing as Rust to_lowercase() is unicode
                    // aware and therefore much slower
                    slug.push((x - b'A' + b'a').into());
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(opts.separator);
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    if slug.ends_with('-') {
        slug.pop();
    }
    // We likely reserved more space than needed.
    slug.shrink_to_fit();
    slug
}
