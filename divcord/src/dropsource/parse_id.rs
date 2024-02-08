use std::{fmt::Display, marker::PhantomData};

use strum::IntoEnumIterator;

use super::Identified;

#[derive(Debug)]
pub struct UnknownVariant<T>(pub String, PhantomData<T>);
impl<T: Identified + IntoEnumIterator> Display for UnknownVariant<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Unknown variant {}, expected one of `{}`",
            self.0,
            T::iter()
                .map(|e| e.id().to_owned())
                .collect::<Vec<_>>()
                .join(", "),
        )
    }
}

pub fn parse_id<T>(s: &str) -> Result<T, UnknownVariant<T>>
where
    T: Identified + IntoEnumIterator,
{
    for variant in T::iter() {
        if s == variant.id() || variant.alises().iter().any(|alias| s == *alias) {
            return Ok(variant);
        }
    }

    Err(UnknownVariant::<T>(s.to_owned(), PhantomData::<T>))
}

mod test {

    #[test]
    fn parse_id_aliase() {
        use super::*;
        use std::str::FromStr;
        use strum_macros::EnumIter;

        #[derive(Debug, EnumIter, PartialEq, strum_macros::Display)]
        pub enum TestEnum {
            A,
            B,
            C,
        }

        impl FromStr for TestEnum {
            type Err = UnknownVariant<Self>;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                parse_id(s)
            }
        }

        impl Identified for TestEnum {
            fn id(&self) -> &str {
                match self {
                    TestEnum::A => "a",
                    TestEnum::B => "b",
                    TestEnum::C => "c",
                }
            }

            fn alises(&self) -> Vec<&'static str> {
                match self {
                    TestEnum::A => vec!["aa"],
                    _ => vec![],
                }
            }
        }

        let value: TestEnum = "aa".parse().unwrap();
        assert_eq!(value, TestEnum::A);
    }
}
