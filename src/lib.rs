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
/// assert_eq!(slugify("You & Me"), "you-and-me");
/// assert_eq!(slugify("user@example.com"), "user-at-example-com");
/// ```
pub fn slugify<S: AsRef<str>>(s: S) -> String {
    let s = s.as_ref();
    let mut string = String::with_capacity(s.len());
    {
        let output = unsafe { string.as_mut_vec() };
        let mut dash = true;

        let mut push_char = |c: char| {
            match c {
                'a'...'z'|'0'...'9'|'A'...'Z' => {
                    match c {
                        'a'...'z'|'0'...'9' => output.push(c as u8),
                        'A'...'Z' => output.push((c as u8)-('A' as u8)+('a' as u8)),
                        _ => unreachable!(),
                    }
                    dash = false;
                },
                _ => if dash {
                    match c {
                        '@' => output.extend(b"at-"),
                        '&' => output.extend(b"and-"),
                        _ => {}
                    }
                } else {
                    match c {
                        '@' => output.extend(b"-at-"),
                        '&' => output.extend(b"-and-"),
                        _ => output.push(b'-'),
                    }
                    dash = true;
                }
            }
        };
        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c);
            } else {
                for c in unidecode_char(c).chars() {
                    (push_char)(c);
                }
            }
        }
    }

    if string.ends_with('-') {
        string.pop();
    }

    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}
