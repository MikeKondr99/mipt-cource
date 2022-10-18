#![forbid(unsafe_code)]

pub fn longest_common_prefix(strs: Vec<&str>) -> String {
    let mut res = String::new();
    if strs.is_empty() {
        return res;
    }
    let mut vec_chars: Vec<_> = strs.iter().map(|x| x.chars()).collect();
    'l: loop {
        let first = vec_chars[0].next();
        if first.is_none() {
            break 'l;
        }
        for vec_ch in vec_chars.iter_mut().skip(1) {
            let ch2 = vec_ch.next();
            if first != ch2 {
                break 'l;
            }
        }
        res.push(first.unwrap());
    }
    res
}

#[cfg(test)]
mod tests {
    use super::longest_common_prefix;
    use ntest::timeout;
    use rand::{distributions::Alphanumeric, Rng};

    #[test]
    fn empty_vec() {
        assert_eq!(longest_common_prefix(vec![]), "");
    }

    #[test]
    fn empty_string() {
        assert_eq!(longest_common_prefix(vec![""]), "");
    }

    #[test]
    fn two_empty_strings() {
        assert_eq!(longest_common_prefix(vec!["", ""]), "");
    }

    #[test]
    fn it_works1() {
        assert_eq!(
            longest_common_prefix(vec!["flower", "flow", "flight"]),
            "fl"
        );
    }

    #[test]
    fn it_works2() {
        assert_eq!(longest_common_prefix(vec!["dog", "racecar", "car"]), "");
    }

    #[test]
    fn it_works3() {
        assert_eq!(longest_common_prefix(vec!["hello", "hello"]), "hello");
    }

    #[test]
    fn it_works4() {
        assert_eq!(longest_common_prefix(vec!["hello123", "hello"]), "hello");
    }

    #[test]
    #[timeout(1000)]
    fn simple_stress() {
        let s: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(100000)
            .map(char::from)
            .collect();
        let test_vec = (0..100).map(|_| &s[..]).collect();
        assert_eq!(longest_common_prefix(test_vec), s);
    }

    #[test]
    fn unicode1() {
        assert_eq!(
            longest_common_prefix(vec![" ( ͡❛ ͜ʖ ͡❛) ✊", " ( ͡❛ ͜ʖ ͡❛)✊"]),
            " ( ͡❛ ͜ʖ ͡❛)"
        );
    }

    #[test]
    fn unicode2() {
        // Note:
        // Ì = U+00CD
        // Ý = U+00DD
        // I.e. they aren't equal somewhere in between
        assert_eq!(
            longest_common_prefix(vec!["hi!✊ Ìha", "hi!✊ Ýha"]),
            "hi!✊ "
        );
    }
}
