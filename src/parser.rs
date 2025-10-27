use std::collections::HashMap;

use super::Value;
use super::tokenize::Token;

type ParseResult = Result<Value, TokenParseError>;

fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];
    if matches!(
        token,
        Token::Null | Token::False | Token::True | Token::Number(_) | Token::String(_)
    ) {
        *index += 1
    }
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftCurlyBracket => parse_object(tokens, index),
        Token::LeftSquareBracket => parse_array(tokens, index),
        _ => todo!(),
    }
}

fn parse_string(s: &str) -> ParseResult {
    let mut output = String::with_capacity(s.len());
    let mut is_escaping = false;
    let mut chars = s.chars();

    while let Some(next_char) = chars.next() {
        if is_escaping {
            match next_char {
                '"' => output.push('"'),
                '\\' => output.push('\\'),
                'b' => output.push('\u{8}'),
                'f' => output.push('\u{12}'),
                '\n' => output.push('\n'),
                '\r' => output.push('\r'),
                '\t' => output.push('\t'),
                'u' => {
                    let mut sum = 0;
                    for i in 0..4 {
                        let next_char = chars.next().ok_or(TokenParseError::UnfinishedEscape)?;
                        let digit = next_char
                            .to_digit(16)
                            .ok_or(TokenParseError::InvalidHexValue)?;
                        sum += (16u32).pow(3 - i) * digit;
                    }
                    let unescape_char =
                        char::from_u32(sum).ok_or(TokenParseError::InvalidCodePointValue)?;
                    output.push(unescape_char);
                }
                // any other character *may* be escaped, ex. `\q` just push that letter `q`
                _ => output.push(next_char),
            }
            is_escaping = true;
        } else if next_char == '\\' {
            is_escaping = true;
        } else {
            output.push(next_char);
        }
    }

    Ok(Value::String(output))
}

fn parse_array(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut arr: Vec<Value> = Vec::new();
    loop {
        // consume previous left bracket or comma token
        *index += 1;
        if tokens[*index] == Token::RightSquareBracket {
            break;
        }
        let value = parse_tokens(tokens, index)?;
        arr.push(value);

        let token = &tokens[*index];
        match token {
            Token::Comma => {}
            Token::RightSquareBracket => break,
            _ => return Err(TokenParseError::ExpectedComma),
        }
    }
    // consume right bracket token
    *index += 1;
    Ok(Value::Array(arr))
}

fn parse_object(tokens: &[Token], index: &mut usize) -> ParseResult {
    let mut map = HashMap::new();
    loop {
        // consume previous left brace or comma
        *index += 1;
        if tokens[*index] == Token::RightCurlyBracket {
            break;
        }
        if let Token::String(s) = &tokens[*index] {
            *index += 1;
            if tokens[*index] == Token::Colon {
                *index += 1;
                let key = s.clone();
                let value = parse_tokens(tokens, index)?;
                println!("{:?}", value);
                map.insert(key, value);
            } else {
                return Err(TokenParseError::ExpectedColon);
            }
            match &tokens[*index] {
                Token::Comma => {}
                Token::RightCurlyBracket => break,
                _ => return Err(TokenParseError::ExpectedComma),
            }
        } else {
            return Err(TokenParseError::ExpectedProperty);
        }
    }
    // consume right brace
    *index += 1;
    Ok(Value::Object(map))
}

#[derive(Debug, PartialEq)]
enum TokenParseError {
    UnfinishedEscape,
    InvalidHexValue,
    InvalidCodePointValue,
    ExpectedComma,
    ExpectedProperty,
    ExpectedColon,
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{Value, tokenize::Token};

    use super::parse_tokens;

    fn check(input: &[Token], expected: Value) {
        let actual = parse_tokens(&input, &mut 0).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_null() {
        let input = [Token::Null];
        let expected = Value::Null;

        check(&input, expected);
    }

    #[test]
    fn parse_false() {
        let input = [Token::False];
        let expected = Value::Boolean(false);

        check(&input, expected);
    }

    #[test]
    fn parse_true() {
        let input = [Token::True];
        let expected = Value::Boolean(true);

        check(&input, expected);
    }

    #[test]
    fn parse_number() {
        let input = [Token::Number(1.2)];
        let expected = Value::Number(1.2);

        check(&input, expected);
    }

    #[test]
    fn parse_string_no_escape() {
        let input = [Token::String("hello world".into())];
        let expected = Value::String("hello world".into());

        check(&input, expected);
    }

    #[test]
    fn parse_string_non_ascii() {
        let input = [Token::String("olá_こんにちは_नमस्ते_привіт".into())];
        let expected = Value::String("olá_こんにちは_नमस्ते_привіт".into());

        check(&input, expected);
    }

    #[test]
    fn parse_string_with_emoji() {
        let input = [Token::String("hello 💩 world".into())];
        let expected = Value::String("hello 💩 world".into());

        check(&input, expected);
    }

    #[test]
    fn parse_string_unescape_backslash() {
        let input = [Token::String(r#"hello\\world"#.into())];
        let expected = Value::String(r#"hello\world"#.into());

        check(&input, expected);
    }

    #[test]
    fn parses_array_one_element() {
        // [true]
        let input = [
            Token::LeftSquareBracket,
            Token::True,
            Token::RightSquareBracket,
        ];
        let expected = Value::Array(vec![Value::Boolean(true)]);

        check(&input, expected);
    }

    #[test]
    fn parses_array_two_elements() {
        // [null, 16]
        let input = [
            Token::LeftSquareBracket,
            Token::Null,
            Token::Comma,
            Token::Number(16.0),
            Token::RightSquareBracket,
        ];
        let expected = Value::Array(vec![Value::Null, Value::Number(16.0)]);

        check(&input, expected);
    }

    #[test]
    fn parse_empty_array() {
        // []
        let input = [Token::LeftSquareBracket, Token::RightSquareBracket];
        let expected = Value::Array(vec![]);

        check(&input, expected);
    }

    #[test]
    fn parse_nested_array() {
        // [null, [null]]
        let input = [
            Token::LeftSquareBracket,
            Token::Null,
            Token::Comma,
            Token::LeftSquareBracket,
            Token::Null,
            Token::RightSquareBracket,
            Token::RightSquareBracket,
        ];
        let expected = Value::Array(vec![Value::Null, Value::Array(vec![Value::Null])]);

        check(&input, expected);
    }

    #[test]
    fn parse_empty_object() {
        // {}
        let input = [Token::LeftCurlyBracket, Token::RightCurlyBracket];
        let expected = Value::Object(HashMap::new());

        check(&input, expected);
    }

    #[test]
    fn parse_object_one_item() {
        // {"a": "A"}
        let mut map = HashMap::new();
        map.insert(String::from("a"), Value::String(String::from("A")));
        let input = [
            Token::LeftCurlyBracket,
            Token::String("a".into()),
            Token::Colon,
            Token::String("A".into()),
            Token::RightCurlyBracket,
        ];
        let expected = Value::Object(map);

        check(&input, expected);
    }

    #[test]
    fn parse_object_two_items() {
        // {"a": "A", "b": null}
        let mut map = HashMap::new();
        map.insert(String::from("a"), Value::String(String::from("A")));
        map.insert(String::from("b"), Value::Null);
        let input = [
            Token::LeftCurlyBracket,
            Token::String("a".into()),
            Token::Colon,
            Token::String("A".into()),
            Token::Comma,
            Token::String("b".into()),
            Token::Colon,
            Token::Null,
            Token::RightCurlyBracket,
        ];
        let expected = Value::Object(map);

        check(&input, expected);
    }

    #[test]
    fn parse_object_nested_with_array() {
        // {"a": [null, 6]}
        let mut map = HashMap::new();
        map.insert(
            String::from("a"),
            Value::Array(vec![Value::Null, Value::Number(6f64)]),
        );
        let input = [
            Token::LeftCurlyBracket,
            Token::String("a".into()),
            Token::Colon,
            Token::LeftSquareBracket,
            Token::Null,
            Token::Comma,
            Token::Number(6f64),
            Token::RightSquareBracket,
            Token::RightCurlyBracket,
        ];
        let expected = Value::Object(map);

        check(&input, expected);
    }

    #[test]
    fn parse_object_nested_with_object() {
        // {"a": {"b": 6}}
        let mut map = HashMap::new();
        let mut inner = HashMap::new();
        inner.insert(String::from("b"), Value::Number(6f64));
        map.insert(String::from("a"), Value::Object(inner));
        let input = [
            Token::LeftCurlyBracket,
            Token::String("a".into()),
            Token::Colon,
            Token::LeftCurlyBracket,
            Token::String("b".into()),
            Token::Colon,
            Token::Number(6f64),
            Token::RightCurlyBracket,
            Token::RightCurlyBracket,
        ];
        let expected = Value::Object(map);

        check(&input, expected);
    }
}
