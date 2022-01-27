use std::cell::RefCell;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenType {
    Noop,
    Operand,
    Function,
    Subexpression,
    Argument,
    OperatorPrefix,
    OperatorInfix,
    OperatorPostfix,
    Whitespace,
    Unknown,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum TokenSubType {
    Nothing,
    Start,
    Stop,
    Text,
    Number,
    Logical,
    Error,
    Math,
    Concatenation,
    Intersection,
    Union,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub value: String,
    pub token_type: TokenType,
    pub sub_type: TokenSubType,
}

impl Token {
    pub fn new(v: String, token_type: TokenType, sub_type: TokenSubType) -> Token {
        Token {
            value: v,
            token_type,
            sub_type,
        }
    }
}

pub struct Tokens {
    index: i64,
    pub items: Vec<Token>,
}

impl Tokens {
    pub fn new() -> Tokens {
        Tokens {
            index: -1,
            items: vec![],
        }
    }
    pub fn add(&mut self, v: String, token_type: TokenType, sub_type: TokenSubType) -> &Token {
        let t = Token::new(v, token_type, sub_type);
        self.add_ref(&t);
        &self.items[self.items.len() - 1]
    }
    // addRef provides pub fntion to add a token to the end of the list.
    pub fn add_ref(&mut self, t: &Token) {
        self.items.push(t.clone());
    }

    // reset provides pub fntion to reset the index to -1.
    pub fn reset(&mut self) {
        self.index = -1;
    }

    // BOF provides pub fntion to check whether or not beginning of list.
    pub fn bof(&self) -> bool {
        self.index <= 0
    }

    // EOF provides pub fntion to check whether or not end of list.
    pub fn eof(&self) -> bool {
        self.index >= ((self.items.len() as i64) - 1)
    }

    // moveNext provides pub fntion to move the index along one.
    pub fn move_next(&mut self) -> bool {
        if self.eof() {
            return false;
        }
        self.index += 1;
        true
    }

    // current return the current token.
    pub fn current(&self) -> RefCell<Token> {
        if self.index < 0 {}
        RefCell::new(self.items[self.index as usize].clone())
    }

    pub fn set_current_field(
        &mut self,
        _value: Option<String>,
        _token_type: Option<TokenType>,
        _sub_type: Option<TokenSubType>,
    ) {
        let mut token = &mut self.items[self.index as usize];
        if let Some(value) = _value {
            token.value = value;
        }
        if let Some(token_type) = _token_type {
            token.token_type = token_type;
        }
        if let Some(sub_type) = _sub_type {
            token.sub_type = sub_type;
        }
        self.items[self.index as usize] = token.clone();
    }

    // next return the next token (leave the index unchanged).
    pub fn next(&self) -> &Token {
        if self.eof() {
            // return nil;
        }
        &self.items[(self.index as usize) + 1]
    }

    // previous return the previous token (leave the index unchanged).
    pub fn previous(&self) -> &Token {
        if self.index < 1 {
            // return None;
        }
        &self.items[((self.index - 1) as usize)]
    }

    // push provides pub fntion to push a token onto the stack.
    pub fn push(&mut self, t: Token) {
        self.items.push(t);
    }

    // pop provides pub fntion to pop a token off the stack.
    pub fn pop(&mut self) -> Option<Token> {
        if self.items.is_empty() {
            return Some(Token {
                value: String::new(),
                token_type: TokenType::Function,
                sub_type: TokenSubType::Stop,
            });
        }
        let t = self.items.pop().unwrap();
        Some(Token::new(String::new(), t.token_type, TokenSubType::Stop))
    }

    // token provides pub fntion to non-destructively return the top item on the
    // stack.
    pub fn token(&self) -> &Token {
        &self.items[((self.index - 1) as usize)]
    }

    // tp return the top token's type.
    pub fn tp(&self) -> &TokenType {
        if self.index == -1 {
            return &TokenType::Noop;
        }
        let a: &Token = self.token();
        &a.token_type
    }
}
