// REference for possible tokens https://www.json.org/json-en.html

use regex::Regex;

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
pub enum TokenizeError {
    UnrecognizedToken,
    UnfinishedLiteralValue,
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while index < chars.len() {
        let token = make_token(&chars, &mut index, &input)?;
        tokens.push(token);
        index += 1
    }

    Ok(tokens)
}

fn make_token(chars: &[char], index: &mut usize, input: &str) -> Result<Token, TokenizeError> {
    let ch = chars[*index];
    let token = match ch {
        '{' => Token::LeftCurlyBracket,
        '}' => Token::RightCurlyBracket,
        '[' => Token::LeftSquareBracket,
        ']' => Token::RightSquareBracket,
        ':' => Token::Colon,
        ',' => Token::Comma,

        'n' => tokenize_literal(index, Token::Null, input)?,
        't' => tokenize_literal(index, Token::True, input)?,
        'f' => tokenize_literal(index, Token::False, input)?,

        _ => todo!("implement for other"),
    };

    Ok(token)
}

fn tokenize_literal(index: &mut usize, token: Token, input: &str) -> Result<Token, TokenizeError> {
    let re = Regex::new(r"(?<name>null|false|true)").unwrap();
    let Some(captures) = re.captures(input) else {
        return Err(TokenizeError::UnfinishedLiteralValue);
    };
    println!(">>> {:?}", &captures["name"]);
    *index += &captures["name"].len() - 1;
    Ok(token)
}

#[cfg(test)]
mod tests {
    use super::{Token, tokenize};

    #[test]
    fn test_tokenize_punctuation_literals() {
        let input = String::from(",{}[]:");
        let expected = vec![
            Token::Comma,
            Token::LeftCurlyBracket,
            Token::RightCurlyBracket,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Colon,
        ];

        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_broken_literal_tokens_return_error() {
        let bad_null = String::from("nolll");
        assert!(tokenize(bad_null).is_err());
    }

    #[test]
    fn test_true_comma() {
        let input = String::from("true,");
        let expected = vec![Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(expected, actual);
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
