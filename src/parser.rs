use super::Value;
use super::tokenize::Token;

type ParseResult = Result<Value, TokenParseError>;

fn parse_tokens(tokens: &[Token], index: &mut usize) -> ParseResult {
    let token = &tokens[*index];
    match token {
        Token::Null => Ok(Value::Null),
        Token::False => Ok(Value::Boolean(false)),
        Token::True => Ok(Value::Boolean(true)),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::RightCurlyBracket => todo!(),
        Token::LeftSquareBracket => todo!(),
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

#[derive(Debug, PartialEq)]
enum TokenParseError {
    UnfinishedEscape,
    InvalidHexValue,
    InvalidCodePointValue,
}

#[cfg(test)]
mod tests {
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
}
