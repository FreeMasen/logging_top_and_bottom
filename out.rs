#![feature(prelude_import)]
#[prelude_import]
use std::prelude::v1::*;
#[macro_use]
extern crate std;
use logging_top_and_bottom::prelude::*;
fn main() {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["start "],
                    &match (&"main",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &("test", "test", "examples\\test.rs", 3u32),
            );
        }
    };
    let ret = (move || {
        let _ = next(1, 2, 3);
        let t = Thing;
        let _ = t.test(4, 5);
        let _ = t.test2();
    })();
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["end "],
                    &match (&"main",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &("test", "test", "examples\\test.rs", 3u32),
            );
        }
    };
    ret
}
fn next(one: u32, two: u32, three: u32) -> u32 {
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["start "],
                    &match (&"next",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &("test", "test", "examples\\test.rs", 11u32),
            );
        }
    };
    let ret = (move || one + two + three)();
    {
        let lvl = ::log::Level::Debug;
        if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
            ::log::__private_api_log(
                ::core::fmt::Arguments::new_v1(
                    &["end "],
                    &match (&"next",) {
                        (arg0,) => [::core::fmt::ArgumentV1::new(
                            arg0,
                            ::core::fmt::Display::fmt,
                        )],
                    },
                ),
                lvl,
                &("test", "test", "examples\\test.rs", 11u32),
            );
        }
    };
    ret
}
struct Thing;
impl Thing {
    fn test(&self, one: u32, two: u32) -> u32 {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["start "],
                        &match (&"test",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    lvl,
                    &("test", "test", "examples\\test.rs", 19u32),
                );
            }
        };
        let ret = (move || one + two)();
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["end "],
                        &match (&"test",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    lvl,
                    &("test", "test", "examples\\test.rs", 19u32),
                );
            }
        };
        ret
    }
    fn test2<'a>(&self) -> &'a str {
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["start "],
                        &match (&"test2",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    lvl,
                    &("test", "test", "examples\\test.rs", 23u32),
                );
            }
        };
        let ret = (move || "hahaha")();
        {
            let lvl = ::log::Level::Debug;
            if lvl <= ::log::STATIC_MAX_LEVEL && lvl <= ::log::max_level() {
                ::log::__private_api_log(
                    ::core::fmt::Arguments::new_v1(
                        &["end "],
                        &match (&"test2",) {
                            (arg0,) => [::core::fmt::ArgumentV1::new(
                                arg0,
                                ::core::fmt::Display::fmt,
                            )],
                        },
                    ),
                    lvl,
                    &("test", "test", "examples\\test.rs", 23u32),
                );
            }
        };
        ret
    }
}
