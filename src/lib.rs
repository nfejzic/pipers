#[macro_export]
macro_rules! pipe {
    ($value:tt) => {
        $value
    };

    ($value:tt |> partial($fun:path, $($partial_vals:tt)*) $(|> $($tail:tt)*)?) => {
        $crate::pipe!({ (|_self| $fun(_self, $($partial_vals)*))($value) } $(|> $($tail)*)?)
    };

    ($value:tt |> partial($fun:tt, $($partial_vals:tt)*) $(|> $($tail:tt)*)?) => {
        $crate::pipe!({ (|_self| $fun(dbg!(_self), $($partial_vals)*))($value) } $(|> $($tail)*)?)
    };

    ($value:tt |> $fun:path $(|> $($tail:tt)*)?) => {
        $crate::pipe!({ $fun($value) } $(|> $($tail)*)?)
    };

    ($value:tt |> $fun:tt $(|> $($tail:tt)*)?) => {
        $crate::pipe!({ $fun($value) } $(|> $($tail)*)?)
    };
}

// #[macro_export]
// macro_rules! partial {
//     ($fun:path, $($values:tt)*) => {
//         |_self| $fun(_self, $($values)*)
//     };
//
//     ($fun:tt, $($values:tt)*) => {
//         |_self| _self.$fun($($values)*)
//     };
// }

#[cfg(test)]
mod tests {
    #[test]
    fn only_value() {
        let value = crate::pipe!(42);
        assert_eq!(value, 42);
    }

    #[test]
    fn single_function() {
        let value = crate::pipe!(42 |> (|val| val * 2));
        assert_eq!(value, 84);
    }

    #[test]
    fn multiple_functions() {
        let value = crate::pipe! {
            0b00001111u8
            |> u8::count_ones // = 4
            |> (|val| (val * 2) as u8) // = 8 = 0b00001000
            |> u8::count_zeros // = 7
        };

        assert_eq!(value, 7);
    }

    #[test]
    fn with_partial_application() {
        let value = crate::pipe! {
            0b00001111u8
            |> u8::count_ones // = 4
            |> partial(u32::checked_add, 10) // = 14 = 0b00001110
            |> Option::unwrap
            |> TryInto::try_into
            |> Result::unwrap
            |> u8::count_zeros // = 5
        };

        assert_eq!(value, 5);

        let value = crate::pipe! {
            0b00001111u8
            |> u8::count_ones // = 4
            |> partial((|first, second, third| -> u8 {
                    (first + second + third) as u8
                }), 4, 6) // = 14 = 0b00001110
            |> u8::count_zeros // = 5
        };

        assert_eq!(value, 5);
    }

    #[test]
    fn with_partial_method() {
        struct Test {
            inner: u8,
        }

        impl Test {
            fn add_two_vals(self, second: u8, third: u8) -> Self {
                Test {
                    inner: self.inner + second + third,
                }
            }

            fn into_inner(self) -> u8 {
                self.inner
            }
        }

        #[allow(unused_parens)]
        let value = crate::pipe! {
            (Test { inner: 0b00001111 }) // = 15
            |> partial(Test::add_two_vals, 2, 3) // = 20 = 0b00010100
            |> Test::into_inner
            |> u8::count_zeros // = 6
        };

        assert_eq!(value, 6);
    }
}
