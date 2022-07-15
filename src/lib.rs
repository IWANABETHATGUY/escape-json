use std::borrow::Cow;
use std::collections::BinaryHeap;
const ESCAPE_STRING_LENGTH: usize = 3;
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
    vec.sort_unstable_by(|a, b| a.0.cmp(&b.0));
    // println!("{:?}", vec.iter().map(|item| item.0).collect::<Vec<_>>());
    let ret = if vec.len() > 0 {
        let mut ret = String::with_capacity(input.len() + vec.len());
        let mut last = 0;
        for (i, ch) in vec.into_iter() {
            ret.push_str(unsafe { input.get_unchecked(last..i) });
            ret.push_str(if ch == "\u{2028}" { r#"\\u2028"# } else { r#"\\u2029"# });
            last = i + ESCAPE_STRING_LENGTH;
        }
        ret.push_str(unsafe { input.get_unchecked(last..) });
        Cow::Owned(ret)
    } else {
        Cow::Borrowed(input)
    };
    ret
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
            let escaped = super::two_pass_search_one_pass_copy(source);
            assert_eq!(escaped, expected)
        }
    }
}
