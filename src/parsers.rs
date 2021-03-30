use std::collections::HashMap;

pub(crate) fn parse_config(config: &str) -> Result<HashMap<String, String>, ParseError> {
    let (key, value) = match config.split('=').count() {
        1 => return Err(ParseError(ParseErrorKind::NotEnoughEquals)),
        2 => {
            let mut i = config.split('=');
            (i.next().unwrap(), i.next().unwrap())
        }
        _ => return Err(ParseError(ParseErrorKind::TooManyEquals)),
    };

    let mut map = HashMap::new();
    map.insert(key.to_owned(), value.to_owned());

    Ok(map)
}

#[derive(Debug, thiserror::Error)]
#[error("Parsing failed due to {0}")]
pub(crate) struct ParseError(ParseErrorKind);

#[derive(Debug, thiserror::Error)]
pub(crate) enum ParseErrorKind {
    #[error("no '=' sign found")]
    NotEnoughEquals,
    #[error("too many '=' signs")]
    TooManyEquals,
}
