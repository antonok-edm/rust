#![warn(clippy::non_zero_suggestions)]
//@no-rustfix
use std::num::{NonZeroI16, NonZeroI8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize};

fn main() {
    let x: u64 = u64::from(NonZeroU32::new(5).unwrap().get());
    //~^ ERROR: consider using `NonZeroU64::from()` for more efficient and type-safe conversion

    let n = NonZeroU32::new(20).unwrap();
    let y = u64::from(n.get());
    //~^ ERROR: consider using `NonZeroU64::from()` for more efficient and type-safe conversion
    some_fn_that_only_takes_u64(y);
}

fn return_non_zero(x: u64, y: NonZeroU32) -> u64 {
    u64::from(y.get())
    //~^ ERROR: consider using `NonZeroU64::from()` for more efficient and type-safe conversion
}

fn some_fn_that_only_takes_u64(_: u64) {}
