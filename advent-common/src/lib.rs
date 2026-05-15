//! Shared helpers for Advent of Code workspace crates.
//!
//! The [`define_advent_registry`] macro emits `mod dayN` declarations and `pub fn factory`
//! for a comma-separated list of day numbers. Invoke it once per yearly crate **after**
//! declaring `trait AdventProblem` (the factory returns `dyn AdventProblem`).
//!
//! Each day module must expose `pub struct DayN` as `crate::dayN::DayN`. The invoking
//! crate must depend on the `paste` crate (workspace dependency).

#[macro_export]
macro_rules! define_advent_registry {
    ($($day:literal),+ $(,)*) => {
        $(
            ::paste::paste! {
                mod [<day $day>];
            }
        )*

        pub fn factory(date: u8) -> ::std::boxed::Box<dyn AdventProblem> {
            match date {
                $(
                    $day => ::std::boxed::Box::new(::paste::paste! {
                        [<day $day>]::[<Day $day>]
                    }),
                )*
                _ => ::core::unimplemented!(),
            }
        }
    };
}
