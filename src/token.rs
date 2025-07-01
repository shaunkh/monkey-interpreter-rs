pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers and literals
    Ident,
    Int,

    // Operators
    Assign,
    Plus,

    // Delimiters
    Comma,
    Semicolon,

    Lparen,
    Rparen,
    Lbrace,
    Rbrace,

    // Keywords
    Function,
    Let,
}

pub struct Token {
    token_type: TokenType,
    literal: String,
}

impl Token {
    pub fn new(self, token_type: TokenType) -> Token {
        let literal = match self.token_type {
            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Lparen => "(",
            TokenType::Rparen => ")",
            TokenType::Lbrace => "{",
            TokenType::Rbrace => "}",
            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::Eof => "",
            _ => panic!("Not implemented"),
        };
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }
    // pub fn token_literal(&self) -> &str {
    //     match self.token_type {
    //         TokenType::Assign => "=",
    //         TokenType::Plus => "+",
    //         TokenType::Lparen => "(",
    //         TokenType::Rparen => ")",
    //         TokenType::Lbrace => "{",
    //         TokenType::Rbrace => "}",
    //         TokenType::Comma => ",",
    //         TokenType::Semicolon => ";",
    //         TokenType::Eof => "",
    //         _ => panic!("Not implemented"),
    //     }
    // }
}

#[cfg(test)]
mod tests {
    use super::{Token, TokenType};

    #[test]
    fn exploration() {
        let expected = [
            Token {
                token_type: TokenType::Assign,
                literal: "=".to_string(),
            },
            Token {
                token_type: TokenType::Plus,
                literal: "+".to_string(),
            },
            Token {
                token_type: TokenType::Lparen,
                literal: "(".to_string(),
            },
            Token {
                token_type: TokenType::Rparen,
                literal: ")".to_string(),
            },
            Token {
                token_type: TokenType::Lbrace,
                literal: "{".to_string(),
            },
            Token {
                token_type: TokenType::Rbrace,
                literal: "}".to_string(),
            },
            Token {
                token_type: TokenType::Comma,
                literal: ",".to_string(),
            },
            Token {
                token_type: TokenType::Semicolon,
                literal: ";".to_string(),
            },
            Token {
                token_type: TokenType::Eof,
                literal: "".to_string(),
            },
        ];
    }
}
