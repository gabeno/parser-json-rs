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

pub fn tokenize(input: String) -> Vec<Token> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut tokens: Vec<Token> = Vec::new();

    while index < chars.len() {
        let token = make_token(chars[index]);
        tokens.push(token);
        index += 1
    }

    tokens
}

fn make_token(ch: char) -> Token {
    match ch {
        '{' => Token::LeftCurlyBracket,
        '}' => Token::RightCurlyBracket,
        '[' => Token::LeftSquareBracket,
        ']' => Token::RightSquareBracket,
        ':' => Token::Colon,
        ',' => Token::Comma,

        // others
        _ => todo!("implement other tokens"),
    }
}

#[cfg(test)]
mod tests {
    use super::{Token, tokenize};

    #[test]
    fn test_tokenize_comma() {
        let comma = String::from(",{}[]:");
        let expected = vec![
            Token::Comma,
            Token::LeftCurlyBracket,
            Token::RightCurlyBracket,
            Token::LeftSquareBracket,
            Token::RightSquareBracket,
            Token::Colon,
        ];

        let actual = tokenize(comma);

        assert_eq!(expected, actual);
    }
}
