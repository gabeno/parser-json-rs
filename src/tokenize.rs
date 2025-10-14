// REference for possible tokens https://www.json.org/json-en.html

use std::num::ParseFloatError;

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
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    UnexpectedEof,
    CharNotRecognized(char),
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
    let mut ch = chars[*index];
    while ch.is_ascii_whitespace() {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars[*index];
    }
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
        ch if ch.is_ascii_digit() | (ch == '-' && chars[*index + 1].is_ascii_digit()) => {
            tokenize_float(chars, index)?
        }
        '"' => tokenize_string(chars, index)?,

        ch => return Err(TokenizeError::CharNotRecognized(ch)),
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

fn tokenize_float(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut has_decimal = false;
    let mut is_negative = false;

    while *index < chars.len() {
        let ch = chars[*index];
        match ch {
            ch if ch.is_ascii_digit() => unparsed_num.push(ch),
            ch if ch == '.' && !has_decimal => {
                unparsed_num.push('.');
                has_decimal = true;
            }
            ch if ch == '-' => is_negative = true,
            _ => break,
        }
        *index += 1;
    }

    match unparsed_num.parse() {
        Ok(f) => {
            if is_negative {
                Ok(Token::Number(-1.0 * f))
            } else {
                Ok(Token::Number(f))
            }
        }
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

fn tokenize_string(chars: &[char], index: &mut usize) -> Result<Token, TokenizeError> {
    let mut string = String::new();
    let mut is_escaping = false;

    loop {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnclosedQuotes);
        }

        let ch = chars[*index];
        match ch {
            '"' if !is_escaping => break,
            '\\' => is_escaping = !is_escaping,
            _ => is_escaping = false,
        }

        string.push(ch);
    }

    Ok(Token::String(string))
}

#[cfg(test)]
mod tests {
    use super::{Token, tokenize};

    #[test]
    fn test_broken_literal_tokens_return_error() {
        let bad_null = String::from("nolll");
        assert!(tokenize(bad_null).is_err());
    }

    macro_rules! test_tokens {
        ($name:ident, $token_name:expr, $expected:expr) => {
            #[test]
            fn $name() {
                assert_eq!(tokenize($token_name).unwrap(), $expected);
            }
        };
    }
    test_tokens!(
        test_punctuation_literals,
        String::from(",{}[]:"),
        vec![
            Token::Comma,
            Token::LeftCurlyBracket,
            Token::RightCurlyBracket,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Colon,
        ]
    );
    test_tokens!(test_null, String::from("null"), vec![Token::Null]);
    test_tokens!(test_false, String::from("false"), vec![Token::False]);
    test_tokens!(test_true, String::from("true"), vec![Token::True]);
    test_tokens!(
        test_true_comma,
        String::from("true,"),
        vec![Token::True, Token::Comma]
    );
    test_tokens!(
        test_integer,
        String::from("123"),
        vec![Token::Number(123.0)]
    );
    test_tokens!(
        test_float,
        String::from("123.9"),
        vec![Token::Number(123.9)]
    );
    test_tokens!(
        test_negative_float,
        String::from("-123.9"),
        vec![Token::Number(-123.9)]
    );
    test_tokens!(
        test_string,
        String::from("\"gabe\""),
        vec![Token::String(String::from("gabe"))]
    );
}
