use crate::token::Token;

struct Lexer {

}

impl Lexer {
    fn new(input: &str) -> Lexer {

    }

    fn next_token(&self) -> Token {

    }
}

#[cfg(test)]
mod tests {

    use crate::token;
    use crate::lexer::Lexer;

    #[test]
    fn test_next_token() {
        let input = "=+(){},;";
        let tests = [
            (token::ASSIGN, "="),
            (token::PLUS, "+"),
            (token::LEFT_PAREN, "("),
            (token::RIGHT_PAREN, ")"),
            (token::LEFT_BRACE, "{"),
            (token::RIGHT_BRACE, "}"),
            (token::COMMA, ","),
            (token::SEMICOLON, ";"),
            (token::EOF, ""),
        ];

        let l = Lexer::new(input);

        for (i, tt) in tests.iter().enumerate() {
            let tok = l.next_token();
            let (expected_kind, expected_literal) = tt;

            assert_eq!(tok.kind, expected_kind, "test[{}] - token kind wrong. expected={} got={}", i, expected_kind, tok.kind);
            assert_eq!(tok.literal, expected_literal, "test[{}] - literal wrong. expected={} got={}", i, expected_literal, tok.literal);
        }

    }

}