use super::tokenize::Token;
use crate::Value;

type ParseResult = Result<Value, TokenParseError>;

fn parse_string(input: &str) -> ParseResult {
    let mut output = String::new();
    let mut is_escaping = false;
    let mut chars = input.chars();
    while let Some(next_char) = chars.next() {
        if is_escaping {
            todo!("implement");
            is_escaping = false;
        } else if next_char == '\\' {
            is_escaping = true;
        } else {
            output.push(next_char);
        }
    }

    Ok(Value::String(output))
}

fn parse_token(tokens: &[Token], index: &mut usize) -> Result<Value, TokenParseError>{
    let token = &tokens[*index];
    match token {
        Token::Null => todo!("implement null"),
        Token::False => todo!("implement false"),
        Token::True => todo!("implement true"),
        Token::Number(number) => Ok(Value::Number(*number)),
        Token::String(string) => parse_string(string),
        Token::LeftBracket => todo!("implement left bracket"),
        Token::RightBracket => todo!("implement right bracket"),
        _ => todo!("implement other"),
    }
}


#[derive(Debug, PartialEq)]
pub enum TokenParseError {
}


#[cfg(test)]
mod tests {
    use crate::tokenize::Token;
    use crate::Value;
    use super::parse_token;

    fn check(input: &[Token], expected: Value) {
        let actual = parse_token(input, &mut 0).unwrap();
        assert_eq!(actual, expected);
    }

    #[test]
    fn parses_string_no_escapes() {
        let input = [Token::String("hello world".into())];
        let expected = Value::String("hello world".into());

        check(&input, expected);
    }

}