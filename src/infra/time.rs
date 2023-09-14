// HACK:
// Sometimes cnblogs' web API returns time string like: "2023-09-12T14:07:00" or "2019-02-06T08:45:53.94"
// This will patch it to standard RFC3339 format
pub fn patch_rfc3339(time_str: &str) -> String {
    if time_str.len() != 25 {
        let u8vec: Vec<_> = time_str.bytes().take(19).collect();
        format!(
            "{}+08:00",
            String::from_utf8(u8vec)
                .unwrap_or_else(|_| panic!("Can not patch time string: {}", time_str))
        )
    } else {
        time_str.to_owned()
    }
}
