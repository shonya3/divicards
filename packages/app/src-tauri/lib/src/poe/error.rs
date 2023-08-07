use std::fmt::Display;

use serde::{ser::SerializeStruct, Serialize};

#[derive(Debug)]
pub enum AuthError {
    UserDenied,
    OtherWithDescription {
        error: String,
        error_description: String,
    },
    Failed,
}

impl Display for AuthError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            AuthError::UserDenied => write!(f, "You have cancelled the login"),
            AuthError::OtherWithDescription {
                error,
                error_description,
            } => write!(
                f,
                "Authorization Error: {error}. Description: {error_description}"
            ),
            AuthError::Failed => write!(f, "Authorization failed. Try again."),
        }
    }
}

impl Serialize for AuthError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut err = serializer.serialize_struct("AuthError", 4)?;
        match self {
            AuthError::UserDenied => err.serialize_field("authError", "userDenied")?,
            AuthError::OtherWithDescription {
                error: _,
                error_description: _,
            } => err.serialize_field("authError", "otherWithDescription")?,
            AuthError::Failed => err.serialize_field("authError", "failed")?,
        };

        err.serialize_field("message", self.to_string().as_str())?;
        err.serialize_field("kind", "authError")?;
        err.serialize_field("appErrorFromTauri", &true)?;
        err.end()
    }
}
