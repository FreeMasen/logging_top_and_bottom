use logging_top_and_bottom::prelude::*;

#[log_top_and_bottom]
fn main() {
    let _ = next(1, 2, 3);
    let t = Thing;
    let _ = t.test(4, 5);
    let _ = t.test2();
}

#[log_top_and_bottom]
fn next(one: u32, two: u32, three: u32) -> u32 {
    one + two + three
}

struct Thing;

impl Thing {
    #[log_top_and_bottom]
    fn test(&self, one: u32, two: u32) -> u32 {
        one + two
    }
    #[log_top_and_bottom]
    fn test2<'a>(&self) -> &'a str {
        "hahaha"
    }
}