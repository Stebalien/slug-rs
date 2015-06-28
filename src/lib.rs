extern crate unidecode;
use unidecode::unidecode;

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
    let s = unidecode(s.as_ref());
    let mut output = String::with_capacity(s.len());
    let mut dash = true;
    for c in s.chars() {
        match c {
            'a'...'z'|'0'...'9'|'A'...'Z' => {
                match c {
                    'a'...'z'|'0'...'9' => output.push(c),
                    'A'...'Z' => output.push(((c as u8)-('A' as u8)+('a' as u8)) as char),
                    _ => unreachable!(),
                }
                dash = false;
            },
            _ => if dash {
                match c {
                    '@' => output.push_str("at-"),
                    '&' => output.push_str("and-"),
                    _ => {}
                }
            } else {
                match c {
                    '@' => output.push_str("-at-"),
                    '&' => output.push_str("-and-"),
                    _ => output.push('-'),
                }
                dash = true;
            }
        }
    }

    if output.ends_with('-') {
        output.pop();
    }

    output.shrink_to_fit();
    output
}
