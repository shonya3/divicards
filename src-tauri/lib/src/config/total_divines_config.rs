use crate::error::Error;
use std::env;

pub struct TotalDivinesConfig {
    pub card_price: f32,
    pub divine_price: f32,
    pub filename: String,
}

impl TotalDivinesConfig {
    pub fn new(mut args: core::iter::Skip<env::Args>) -> Result<TotalDivinesConfig, Error> {
        let card_price: f32 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(arg) => arg,
                Err(err) => return Err(Error::ParseError("Problem with card price parsing", err)),
            },
            None => {
                return Err(Error::ConfigError(
                    "Didnt get a chaos price(first argument)",
                ))
            }
        };

        let divine_price: f32 = match args.next() {
            Some(arg) => match arg.parse() {
                Ok(arg) => arg,
                Err(err) => {
                    return Err(Error::ParseError("Problem with divine price parsing", err))
                }
            },
            None => {
                return Err(Error::ConfigError(
                    "Didnt get a divine price(second argument)",
                ))
            }
        };

        let filename = match args.next() {
            Some(arg) => arg,
            None => return Err(Error::ConfigError("Didnt get a file name(third argument)")),
        };

        Ok(TotalDivinesConfig {
            card_price,
            divine_price,
            filename,
        })
    }
}
