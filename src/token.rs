type TokenType = String;

pub struct Token {
    pub kind: TokenType,
    pub literal: String,
}

pub const ILLEGAL: &str = "ILLEGAL";
pub const EOF: &str = "EOF";

pub const IDENT: &str = "IDENT";
pub const INT: &str = "INT";

pub const ASSIGN: &str = "=";
pub const PLUS: &str = "+";

pub const COMMA: &str = ",";
pub const SEMICOLON: &str = ";";

pub const LEFT_PAREN: &str = "(";
pub const RIGHT_PAREN: &str = ")";
pub const LEFT_BRACE: &str = "{";
pub const RIGHT_BRACE: &str = "}";

pub const FUNCTION: &str = "FUNCTION";
pub const LET: &str = "LET";