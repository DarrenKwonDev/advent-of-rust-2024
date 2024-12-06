pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    let s1_cnt = s1.trim().chars().count();
    let s2_cnt = s2.trim().chars().count();

    if s1_cnt > s2_cnt {
        Some(s1)
    } else if s1_cnt < s2_cnt {
        Some(s2)
    } else {
        None
    }
}