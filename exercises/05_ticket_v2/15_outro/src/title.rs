// TODO: Implement `TryFrom<String>` and `TryFrom<&str>` for the `TicketTitle` type,
//   enforcing that the title is not empty and is not longer than 50 characters.
//   Implement the traits required to make the tests pass too.

#[derive(Debug, PartialEq, Clone)]
pub struct TicketTitle(String);

#[derive(Debug, thiserror::Error)]
#[error("`{invalid_status}` is not a valid status. Use one of: ToDo, InProgress, Done")]
pub struct ParseStatusError {
    pub(crate) invalid_status: String,
}
#[derive(Debug, thiserror::Error)]
pub enum TicketNewError {
    #[error("The title cannot be empty")]
    TitleCannotBeEmpty,
    #[error("The title cannot be longer than 50 bytes")]
    TitleTooLong,
    #[error("The description cannot be empty")]
    DescriptionCannotBeEmpty,
    #[error("The description cannot be longer than 500 bytes")]
    DescriptionTooLong,
    #[error(transparent)]
    StatusParseFailed(#[from] ParseStatusError),
}

impl TryFrom<String> for TicketTitle {
    type Error = TicketNewError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if value.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        Ok(TicketTitle(value))
    }
}

impl TryFrom<&str> for TicketTitle {
    type Error = TicketNewError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if value.is_empty() {
            return Err(TicketNewError::TitleCannotBeEmpty);
        }
        if value.len() > 50 {
            return Err(TicketNewError::TitleTooLong);
        }
        Ok(TicketTitle(value.to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let title = TicketTitle::try_from("A title".to_string()).unwrap();
        assert_eq!(title.0, "A title");
    }

    #[test]
    fn test_try_from_empty_string() {
        let err = TicketTitle::try_from("".to_string()).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be empty");
    }

    #[test]
    fn test_try_from_long_string() {
        let title =
            "A title that's definitely longer than what should be allowed in a development ticket"
                .to_string();
        let err = TicketTitle::try_from(title).unwrap_err();
        assert_eq!(err.to_string(), "The title cannot be longer than 50 bytes");
    }

    #[test]
    fn test_try_from_str() {
        let title = TicketTitle::try_from("A title").unwrap();
        assert_eq!(title.0, "A title");
    }
}
