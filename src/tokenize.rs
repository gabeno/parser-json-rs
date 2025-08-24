// REference for possible tokens https://www.json.org/json-en.html

#[derive(Debug, PartialEq)]
pub enum Token {
    // punctuation tokens
    /// `{`
    LeftCurlyBracket,
    /// `}`
    RightCurlyBracket,
    /// `[`
    LeftSquareBracket,
    /// `]`
    RightSquareBracket,
    /// `,`
    Comma,
    /// `:`
    Colon,

    // literal tokens
    /// `null`
    Null,
    /// `false`
    False,
    /// `true`
    True,
    /// Any number literal
    Number(f64),
    /// Key of a key/value pair or String
    String(String),
}

#[derive(Debug, PartialEq)]
enum TokenizeError {
    UnfinishedLiteralValue,
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);
        index += 1
    }

    Ok(tokens)
}

fn make_token(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let ch = chars[*index];
    match ch {
        '{' => Ok(Token::LeftCurlyBracket),
        '}' => Ok(Token::RightCurlyBracket),
        '[' => Ok(Token::LeftSquareBracket),
        ']' => Ok(Token::RightSquareBracket),
        ':' => Ok(Token::Colon),
        ',' => Ok(Token::Comma),

        'n' => tokenize_literal(chars, index, Token::Null, String::from("null")),
        't' => tokenize_literal(chars, index, Token::True, String::from("true")),
        'f' => tokenize_literal(chars, index, Token::False, String::from("false")),

        // others
        _ => todo!("implement other tokens"),
    }
}

fn tokenize_null(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1
    }
    Ok(Token::Null)
}

fn tokenize_literal(
    chars: &[char],
    index: &mut usize,
    token: Token,
    token_str: String,
) -> Result<Token, TokenizeError> {
    for expected_char in token_str.chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1
    }
    Ok(token)
}

#[cfg(test)]
mod tests {
    use crate::tokenize::TokenizeError;

    use super::{Token, tokenize};

    #[test]
    fn test_tokenize_punctuation_literals() {
        let comma = String::from(",{}[]:");
        let expected = vec![
            Token::Comma,
            Token::LeftCurlyBracket,
            Token::RightCurlyBracket,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Colon,
        ];

        let actual = tokenize(comma).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_broken_literal_tokens_return_error() {
        let bad_null = String::from("nolll");
        assert!(tokenize(bad_null).is_err());
    }

    macro_rules! test_literal_tokens {
        ($name:ident, $token_name:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(tokenize($token_name).unwrap(), $expected);
            }
        };
    }
    test_literal_tokens!(test_null, String::from("null"), vec![Token::Null]);
    test_literal_tokens!(test_false, String::from("false"), vec![Token::False]);
    test_literal_tokens!(test_true, String::from("true"), vec![Token::True]);
}
