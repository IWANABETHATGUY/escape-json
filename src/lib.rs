use std::borrow::Cow;
const ESCAPE_STRING_LENGTH: usize = 2;
pub fn two_pass_replace<'a>(input: &str) -> String {
    let ret = input
        .replace('\u{2028}', r#"\\u2028"#)
        .replace('\u{2029}', r#"\\u2029"#);
        ret
    // Cow::Owned(ret)
}

pub fn two_pass_search_one_pass_copy<'a>(input: &'a str) -> Cow<'a, str> {
    let vec = input
        .match_indices('\u{2028}')
        .chain(input.match_indices('\u{2029}'))
        .collect::<Vec<_>>();
    let ret = if vec.len() > 0 {
        let mut ret = String::with_capacity(input.len() + vec.len());
        let mut last = 0;
        for (i, ch) in vec {
            ret.push_str(unsafe { input.get_unchecked(last..i) });
            ret.push('\\');
            ret.push_str(ch);
            last = i + ESCAPE_STRING_LENGTH;
        }
        ret.push_str(unsafe { input.get_unchecked(last..) });
        Cow::Owned(ret)
    } else {
        Cow::Borrowed(input)
    };
    ret
}
