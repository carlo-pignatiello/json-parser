use std::num::ParseFloatError;

#[derive(Debug, PartialEq)]
pub enum Token {
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Colon,
    Null,
    False,
    True,
    String(String),
    Number(f64),
}



#[derive(Debug, PartialEq, Eq, Clone)]
pub enum TokenizeError {
    UnfinishedLiteralValue,
    ParseNumberError(ParseFloatError),
    UnclosedQuotes,
    UnexpectedEof,
    CharNotRecognized(char),
}

fn tokenize_null(char: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "null".chars() {
        if expected_char != char[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::Null)
}
fn tokenize_false(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "false".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::False)
}

fn tokenize_true(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    for expected_char in "true".chars() {
        if expected_char != chars[*index] {
            return Err(TokenizeError::UnfinishedLiteralValue);
        }
        *index += 1;
    }
    *index -= 1;
    Ok(Token::True)
}

fn tokenize_float(chars: &Vec<char>, curr_idx: &mut usize) -> Result<Token, TokenizeError> {
    let mut unparsed_num = String::new();
    let mut has_decimal = false;

    while *curr_idx < chars.len() {
        let ch = chars[*curr_idx];
        match ch {
            c if c.is_ascii_digit() => unparsed_num.push(c),
            c if c == '.' && !has_decimal => {
                unparsed_num.push('.');
                has_decimal = true;
            }
            _ => break,
        }
        *curr_idx += 1;
    }

    match unparsed_num.parse() {
        Ok(f) => Ok(Token::Number(f)),
        Err(err) => Err(TokenizeError::ParseNumberError(err)),
    }
}

fn tokenize_string(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
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


fn make_token(chars: &Vec<char>, index: &mut usize) -> Result<Token, TokenizeError> {
    let mut ch = chars[*index];
    while ch.is_ascii_whitespace() {
        *index += 1;
        if *index >= chars.len() {
            return Err(TokenizeError::UnexpectedEof);
        }
        ch = chars[*index];
    }
    let token = match ch {
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        '[' => Token::LeftBracket,
        ']' => Token::RightBracket,
        ',' => Token::Comma,
        ':' => Token::Colon,
        'n' => tokenize_null(chars, index)?,
        'f' => tokenize_false(chars, index)?,
        't' => tokenize_true(chars, index)?,
         c if c.is_ascii_digit() => tokenize_float(chars, index)?,
         '"' => tokenize_string(chars, index)?,
        ch => return Err(TokenizeError::CharNotRecognized(ch)),
        _ => todo!("implement other tokens"),
    };
    Ok(token)
}

pub fn tokenize(input: String) -> Result<Vec<Token>, TokenizeError> {
    let chars: Vec<char> = input.chars().collect();
    let mut index = 0;
    let mut tokens = Vec::new();
    while index < chars.len() {
        let token = make_token(&chars, &mut index)?;
        tokens.push(token);
        index += 1;
    }
    Ok(tokens)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenize() {
        let input = String::from(",");
        let expected = vec![Token::Comma];
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn all_punctuation() {
        let input = String::from("{}[],:");
        let expected = vec![
            Token::LeftBrace,
            Token::RightBrace,
            Token::LeftBracket,
            Token::RightBracket,
            Token::Comma,
            Token::Colon,
        ];
        let tokens = tokenize(input).unwrap();
        assert_eq!(tokens, expected);
    }

    #[test]
    fn just_null() {
        let input = String::from("null");
        let expected = [Token::Null];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_false() {
        let input = String::from("false");
        let expected = [Token::False];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_true() {
        let input = String::from("true");
        let expected = [Token::True];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }
    #[test]
    fn true_comma() {
        let input = String::from("true,");
        let expected = [Token::True, Token::Comma];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn integer() {
        let input = String::from("123");
        let expected = [Token::Number(123.0)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn floating_point() {
        let input = String::from("1.23");
        let expected = [Token::Number(1.23)];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    #[test]
    fn just_ken() {
        let input = String::from("\"ken\"");
        let expected = [Token::String(String::from("ken"))];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

    // inside of `mod tests`, among the other tests
    #[test]
    fn escaped_quote() {
        let input = String::from(r#""the \" is OK""#);
        let expected = [Token::String(String::from(r#"the \" is OK"#))];

        let actual = tokenize(input).unwrap();

        assert_eq!(actual, expected);
    }

}
