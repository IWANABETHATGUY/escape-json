use std::borrow::Cow;
const ESCAPE_STRING_LENGTH: usize = 3;
const U2028: &'static str = r#"\\u2028"#;
const U2029: &'static str = r#"\\u2029"#;
use once_cell::sync::Lazy;
use regex::Regex;

static REGEX: Lazy<Regex> = Lazy::new(|| Regex::new("\u{2028}|\u{2029}").unwrap());
static REGEX2028: Lazy<Regex> = Lazy::new(|| Regex::new("\u{2028}").unwrap());
static REGEX2029: Lazy<Regex> = Lazy::new(|| Regex::new("\u{2029}").unwrap());

pub fn two_pass_replace<'a>(input: &str) -> String {
    let ret = input
        .replace('\u{2028}', r#"\\u2028"#)
        .replace('\u{2029}', r#"\\u2029"#);
    ret
    // Cow::Owned(ret)
}

pub fn two_pass_search_one_pass_copy<'a>(input: &'a str) -> Cow<'a, str> {
    let mut vec = input
        .match_indices('\u{2028}')
        .chain(input.match_indices('\u{2029}'))
        .collect::<Vec<_>>();
    // println!("{:?}", vec.iter().map(|item| item.0).collect::<Vec<_>>());
    let ret = if vec.len() > 0 {
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut ret = String::with_capacity(input.len() + vec.len() * 4);
        // let mut last = 0;
        // for (i, ch) in vec.into_iter() {
        //     ret.push_str(unsafe { input.get_unchecked(last..i) });
        //     ret.push_str(if ch == "\u{2028}" { U2028 } else { U2029 });
        //     last = i + ESCAPE_STRING_LENGTH;
        // }
        // ret.push_str(unsafe { input.get_unchecked(last..) });
        Cow::Owned(ret)
    } else {
        Cow::Borrowed(input)
    };
    ret
}
static BYTES_2028: &'static [u8] = "\u{2028}".as_bytes();
static BYTES_2029: &'static [u8] = "\u{2029}".as_bytes();
pub fn memchr3_replace<'a>(input: &'a str) -> Cow<'a, str> {
    let mut vec = memchr::memmem::find_iter(input.as_bytes(), BYTES_2028)
        .chain(memchr::memmem::find_iter(input.as_bytes(), BYTES_2029))
        .map(|i| (i, &input[i..i + 3]))
        .collect::<Vec<_>>();
    let ret = if vec.len() > 0 {
        vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));
        let mut ret = String::with_capacity(input.len() + vec.len() * 4 + 1);
        let mut last = 0;
        for (i, ch) in vec.into_iter() {
            ret.push_str(unsafe { input.get_unchecked(last..i) });
            ret.push_str(if ch == "\u{2028}" { U2028 } else { U2029 });
            last = i + ESCAPE_STRING_LENGTH;
        }
        ret.push_str(unsafe { input.get_unchecked(last..) });
        Cow::Owned(ret)
    } else {
        Cow::Borrowed(input)
    };
    ret
}
pub fn regex_replace<'a>(input: &'a str) -> String {
    REGEX2029
        .replace_all(&REGEX2028.replace_all(input, U2028), U2029)
        .to_string()
}

pub fn regex_iter_replace<'a>(input: &'a str) -> Cow<'a, str> {
    let mut result_iterator = REGEX.find_iter(input);
    if let Some(mat) = result_iterator.next() {
        let mut ret = String::with_capacity(input.len());
        // This duplicate due to we don't want to allocate a new string if there is no escape `\u2028` or `\u2029`
        ret.push_str(unsafe { input.get_unchecked(0..mat.start()) });
        ret.push_str(if mat.as_str() == "\u{2028}" {
            U2028
        } else {
            U2029
        });
        let mut last = mat.start() + ESCAPE_STRING_LENGTH;
        for mat in result_iterator {
            let start_index = mat.start();
            ret.push_str(unsafe { input.get_unchecked(last..start_index) });
            ret.push_str(if mat.as_str() == "\u{2028}" {
                U2028
            } else {
                U2029
            });
            last = start_index + ESCAPE_STRING_LENGTH;
        }
        ret.push_str(unsafe { input.get_unchecked(last..) });
        Cow::Owned(ret)
    } else {
        Cow::Borrowed(input)
    }
}

pub fn escape_json(json_str: &str) -> String {
    use std::fmt::Write;

    let mut escaped = String::with_capacity(json_str.len());

    for c in json_str.chars() {
        if c == '\u{2028}' {
            write!(&mut escaped, "\\\\u{:04X}", &0x2028).unwrap();
        } else if c == '\u{2029}' {
            write!(&mut escaped, "\\\\u{:04X}", &0x2029).unwrap();
        } else {
            escaped.push(c)
        }
    }

    escaped
}

mod test {
    #[test]
    fn test_escape_json() {
        let cases = vec![
            (
                r#"{"LS":" ","PS":" ","escaped":"\\u2028"}"#,
                r#"{"LS":"\\u2028","PS":"\\u2029","escaped":"\\u2028"}"#,
            ),
            (r#"{"na\ me": "\ntest"}"#, r#"{"na\ me": "\ntest"}"#),
            (r#"{"a": \n\r\t"a"}"#, r#"{"a": \n\r\t"a"}"#),
            (
                r#"{"\"\\\/\b \f\t\r\n": "\"\\\/\b\f\t \r\n"}"#,
                r#"{"\"\\\/\b\\u2028\f\t\r\n": "\"\\\/\b\f\t\\u2028\r\n"}"#,
            ),
        ];

        for (source, expected) in cases {
            assert_eq!(super::two_pass_search_one_pass_copy(source), expected);
            assert_eq!(super::regex_replace(source), expected);
            assert_eq!(super::regex_iter_replace(source), expected);
            assert_eq!(super::memchr3_replace(source), expected);
        }
    }
}
