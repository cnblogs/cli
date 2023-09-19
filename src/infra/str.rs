use std::ops::ControlFlow;
use unicode_width::UnicodeWidthChar;

pub trait StrExt {
    fn width_split_head(&self, head_width: usize) -> (&str, &str);
    fn width_split(&self, width: usize) -> Option<Vec<&str>>;
}

impl StrExt for str {
    fn width_split_head(&self, head_width: usize) -> (&str, &str) {
        let mut left_take = head_width;
        let mut take_bytes = 0;
        self.chars().try_for_each(|c| {
            let current_width = c.width_cjk().unwrap_or(0);
            if left_take > 0 {
                if left_take >= current_width {
                    left_take -= current_width;
                    take_bytes += c.len_utf8();
                    ControlFlow::Continue(())
                } else {
                    left_take = 0;
                    ControlFlow::Break(())
                }
            } else {
                ControlFlow::Break(())
            }
        });
        self.split_at(take_bytes)
    }

    fn width_split(&self, width: usize) -> Option<Vec<&str>> {
        let mut vec = vec![];
        let mut str = self;
        loop {
            let (head, tail) = str.width_split_head(width);
            // No split strategy exist, return None
            if head.is_empty() {
                return None;
            }
            vec.push(head);
            if tail.is_empty() {
                break;
            }
            str = tail;
        }
        Some(vec)
    }
}

#[test]
fn test_width_split_head() {
    let text = "测试test⭐";
    assert_eq!(text.width_split_head(0), ("", "测试test⭐"));
    assert_eq!(text.width_split_head(1), ("", "测试test⭐"));
    assert_eq!(text.width_split_head(2), ("测", "试test⭐"));
    assert_eq!(text.width_split_head(3), ("测", "试test⭐"));
    assert_eq!(text.width_split_head(4), ("测试", "test⭐"));
    assert_eq!(text.width_split_head(5), ("测试t", "est⭐"));
    assert_eq!(text.width_split_head(9), ("测试test", "⭐"));
    assert_eq!(text.width_split_head(10), ("测试test⭐", ""));
    assert_eq!(text.width_split_head(11), ("测试test⭐", ""));
}

#[test]
fn test_width_split() {
    use crate::infra::option::IntoOption;
    let text = "测试test⭐测试test⭐";
    assert_eq!(text.width_split(0), None);
    assert_eq!(text.width_split(1), None);
    assert_eq!(
        text.width_split(2),
        vec!["测", "试", "te", "st", "⭐", "测", "试", "te", "st", "⭐"].into_some()
    );
    assert_eq!(
        text.width_split(3),
        vec!["测", "试t", "est", "⭐", "测", "试t", "est", "⭐"].into_some()
    );
    assert_eq!(
        text.width_split(4),
        vec!["测试", "test", "⭐测", "试te", "st⭐"].into_some()
    );
    assert_eq!(
        text.width_split(19),
        vec!["测试test⭐测试test", "⭐"].into_some()
    );
    assert_eq!(
        text.width_split(20),
        vec!["测试test⭐测试test⭐"].into_some()
    );
    assert_eq!(
        text.width_split(21),
        vec!["测试test⭐测试test⭐"].into_some()
    );
}
