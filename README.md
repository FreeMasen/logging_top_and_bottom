# Add Logging Top And Bottom Example

Trying to debug a recursive application in Rust can be difficult, the LLDB debugger has a hard time handing recursion and/or macro expansion reliably (at least via the VSCode plugin). `println` debugging can be useful but it is difficult to tell when any given recursive iteration has completed.

This crate is a naive approach to handing this sort of problem. The proc_macro located in [crates/macro/src/lib.rs](crates/macro/src/lib.rs) simply wraps any function in a immediately executed move closure adding a `log::debug` call to the start and end of the body's execution. For example

```rust
use logging_top_and_bottom::prelude::*;

#[log_top_and_bottom]
pub fn fib(arg: u32) -> u32 {
    match arg {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    }
}
```

Would expand to something like this:

```rust
use logging_top_and_bottom::prelude::*;

pub fn fib(arg: u32) -> u32 {
    log::debug!("start fib");
    let ret = (move || match arg {
        0 | 1 => 1,
        _ => fib(n - 1) + fib(n - 2)
    })();
    log::debug!("end fib");
    ret
}
```

If someone were to use the above and include a the `simple_logger` crate, it might look like this.

```rust
use logging_top_and_bottom::prelude::*;

fn main() {
    simple_logger::init().unwrap();
    fib(2);
}

#[log_top_and_bottom]
pub fn fib(arg: u32) -> u32 {
    match arg {
        0 | 1 => 1,
        _ => fib(arg - 1) + fib(arg - 2)
    }
}
```

Running it would print something like
```
1972-01-31 01:45:00,206 DEBUG [fib] start fib
1972-01-31 01:45:00,209 DEBUG [fib] start fib
1972-01-31 01:45:00,209 DEBUG [fib] end fib
1972-01-31 01:45:00,209 DEBUG [fib] start fib
1972-01-31 01:45:00,209 DEBUG [fib] end fib
1972-01-31 01:45:00,209 DEBUG [fib] end fib
```