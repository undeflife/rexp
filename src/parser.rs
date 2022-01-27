/// translated from github.com/xuri/efp , mostly
use crate::exp;
use crate::token::{Token, TokenSubType, TokenType, Tokens};
use crate::value::Value;

const QUOTE_DOUBLE: u8 = b'"';
const QUOTE_SINGLE: u8 = b'\'';
const BRACKET_CLOSE: u8 = b']';
const BRACKET_OPEN: u8 = b'[';
const BRACE_OPEN: u8 = b'{';
const BRACE_CLOSE: u8 = b'}';
const PAREN_OPEN: u8 = b'(';
const PAREN_CLOSE: u8 = b')';
const SEMICOLON: u8 = b';';
const WHITESPACE: u8 = b' ';
const COMMA: u8 = b',';
const ERROR_START: u8 = b'#';

const ERRORS: &[&str; 7] = &[
    ",#NULL!,",
    ",#DIV/0!,",
    ",#VALUE!,",
    ",#REF!,",
    ",#NAME?,",
    ",#NUM!,",
    ",#N/A,",
];

const COMPARATORS: &[&str; 4] = &[",==,", ",>=,", ",<=,", ",<>,"];

const OPERATORS: &[&str; 9] = &["+", "-", "*", "/", "^", "&", "=", ">", "<"];

const C_OPERATORS: &[&str; 3] = &["=", ">", "<"];

#[allow(dead_code)]
pub struct Parser {
    formula: String,
    tokens: Tokens,
    token_stack: Tokens,
    offset: usize,
    in_string: bool,
    in_path: bool,
    in_range: bool,
    in_error: bool,
}

impl Parser {
    pub fn new(formula: &str) -> Parser {
        return Parser {
            formula: formula.trim().to_string(),
            tokens: Tokens::new(),
            token_stack: Tokens::new(),
            offset: 0,
            in_string: false,
            in_path: false,
            in_range: false,
            in_error: false,
        };
    }
    // doubleChar provides function to get two characters after the current
    // position.
    pub fn double_char(&self) -> &str {
        if self.formula.len() >= (self.offset + 2) {
            return &self.formula[self.offset..self.offset + 2];
        }
        ""
    }

    // currentChar provides function to get the character of the current position.
    pub fn current_char(&self) -> u8 {
        *self.formula.as_bytes().get(self.offset).unwrap()
    }

    // EOF provides function to check whether or not end of tokens stack.
    pub fn eof(&self) -> bool {
        self.offset >= self.formula.len()
    }

    // Tokenize provides function to parse formula as a token stream (list).
    pub fn tokenize(&mut self) {
        self.get_tokens();
    }

    pub fn parse(&mut self) -> exp::Expression {
        self.tokens.reset();
        self.tokens.move_next(); //move to first one
        let left = self.parse_primary();
        self.build_expression_tree(0, left).unwrap()
    }

    fn new_value(t: &Token) -> Value {
        match t.sub_type {
            TokenSubType::Number => match t.value.parse::<f64>() {
                Ok(f) => Value::Number(f),
                Err(_) => Value::Error("#N/A".to_string()),
            },
            TokenSubType::Logical => match t.value.parse::<bool>() {
                Ok(f) => Value::Boolean(f),
                Err(_) => Value::Error("#N/A".to_string()),
            },
            TokenSubType::Text => Value::String(t.value.clone()),
            TokenSubType::Error => Value::Error(t.value.clone()),
            _ => Value::Unknown,
        }
    }

    fn parse_expression(&mut self) -> exp::Expression {
        let left = self.parse_primary();
        self.build_expression_tree(0, left).unwrap()
    }

    fn build_expression_tree(
        &mut self,
        p: i8,
        mut left: Option<exp::Expression>,
    ) -> Option<exp::Expression> {
        loop {
            let p_token = exp::get_precedence(&self.tokens.current().borrow().value);
            if p_token < p {
                // not a operator
                return left;
            }
            let bin_op = &self.tokens.current().into_inner().value;
            if self.tokens.eof() {
                return left;
            }
            self.tokens.move_next();
            let mut right = self.parse_primary();
            right.as_ref()?; // return none is right is none
            let next = exp::get_precedence(&self.tokens.current().borrow().value);
            if p_token < next {
                right = self.build_expression_tree(p_token + 1, right);
                right.as_ref()?;
            }
            left = Some(exp::Expression::Operator {
                op: Some(bin_op.clone()),
                left: Box::new(left.unwrap()),
                right: Box::new(right),
            })
        }
    }
    fn parse_function_expression(&mut self) -> exp::Expression {
        let tokens = &mut self.tokens;
        let t = tokens.current().into_inner();
        tokens.move_next();
        let mut exps: Vec<exp::Expression> = vec![];
        if self.tokens.current().borrow().sub_type == TokenSubType::Stop {
            // function with no args
            self.tokens.move_next();
            return exp::Expression::Function {
                function: t.value,
                arguments: exps,
            };
        }
        exps.push(self.parse_expression());
        while self.tokens.current().borrow().sub_type != TokenSubType::Stop && !self.tokens.eof() {
            self.tokens.move_next();
            if self.tokens.current().borrow().token_type == TokenType::Argument {
                continue;
            }
            exps.push(self.parse_expression());
        }
        self.tokens.move_next();
        exp::Expression::Function {
            function: t.value,
            arguments: exps,
        }
    }
    fn parse_primary(&mut self) -> Option<exp::Expression> {
        let t = self.tokens.current().into_inner();
        match t.token_type {
            TokenType::Function => Some(self.parse_function_expression()),
            TokenType::Operand => {
                let e = exp::Expression::Literal(Parser::new_value(&t));
                self.tokens.move_next();
                Some(e)
            }
            TokenType::Subexpression => {
                self.tokens.move_next(); // skip paren
                let e = self.parse_expression();
                if self.tokens.current().borrow().token_type != TokenType::Subexpression {
                    return Some(exp::Expression::Literal(Value::Error("#N/A".to_string())));
                }
                self.tokens.move_next(); // skip paren
                Some(e)
            }
            TokenType::OperatorPrefix | TokenType::OperatorInfix => {
                self.tokens.move_next();
                if self.tokens.eof() {
                    return None;
                }
                Some(exp::Expression::Operator {
                    op: Some("-".to_string()),
                    left: Box::new(exp::Expression::Literal(Value::Empty)),
                    right: Box::new(self.parse_primary()),
                })
            }
            _ => Some(exp::Expression::Literal(Value::Error("#N/A".to_string()))),
        }
    }

    // getTokens return a token stream (list).
    pub fn get_tokens(&mut self) {
        if !self.formula.is_empty() && self.formula.as_bytes().get(0).unwrap() != &b'=' {
            self.formula.insert(0, '=');
        }
        let mut token = String::new();
        // state-dependent character evaluation (order is important)
        while !self.eof() {
            // double-quoted strings
            // embeds are doubled
            // end marks token
            if self.in_string {
                if self.current_char() == QUOTE_DOUBLE {
                    self.in_string = false;
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Text);
                    token = String::new();
                } else {
                    token += std::str::from_utf8(&[self.current_char()]).unwrap();
                }
                self.offset += 1;
                continue;
            }
            if self.in_error {
                if self.current_char() == BRACKET_CLOSE {
                    self.in_range = false
                }
                token += std::str::from_utf8(&[self.current_char()]).unwrap();
                self.offset += 1;
                continue;
            }

            if self.in_error {
                token += std::str::from_utf8(&[self.current_char()]).unwrap();
                self.offset += 1;
                if ERRORS
                    .contains(&std::str::from_utf8(&[b',', self.current_char(), b',']).unwrap())
                {
                    self.in_error = false;
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Error);
                    token = String::new();
                }
                continue;
            }
            if self.current_char() == QUOTE_DOUBLE {
                if !token.is_empty() {
                    // not expected
                    self.tokens
                        .add(token, TokenType::Unknown, TokenSubType::Nothing);
                    token = String::new();
                }
                self.in_string = true;
                self.offset += 1;
                continue;
            }

            if self.current_char() == QUOTE_SINGLE {
                if !token.is_empty() {
                    // not expected
                    self.tokens
                        .add(token, TokenType::Unknown, TokenSubType::Nothing);
                    token = String::new();
                }
                //self.InPath = true
                self.offset += 1;
                continue;
            }

            if self.current_char() == BRACKET_OPEN {
                self.in_range = true;
                token += std::str::from_utf8(&[self.current_char()]).unwrap();
                self.offset += 1;
                continue;
            }

            if self.current_char() == ERROR_START {
                if !token.is_empty() {
                    // not expected
                    self.tokens
                        .add(token, TokenType::Unknown, TokenSubType::Nothing);
                    token = String::new();
                }
                self.in_error = true;
                token += std::str::from_utf8(&[self.current_char()]).unwrap();
                self.offset += 1;
                continue;
            }

            // mark start and end of arrays and array rows
            if self.current_char() == BRACE_OPEN {
                if !token.is_empty() {
                    // not expected
                    self.tokens
                        .add(token, TokenType::Unknown, TokenSubType::Nothing);
                    token = String::new();
                }
                self.token_stack.push(
                    self.tokens
                        .add(
                            "ARRAY".to_string(),
                            TokenType::Function,
                            TokenSubType::Start,
                        )
                        .clone(),
                );
                self.token_stack.push(
                    self.tokens
                        .add(
                            "ARRAYROW".to_string(),
                            TokenType::Function,
                            TokenSubType::Start,
                        )
                        .clone(),
                );
                self.offset += 1;
                continue;
            }

            if self.current_char() == SEMICOLON {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add_ref(&self.token_stack.pop().unwrap());
                self.tokens
                    .add(",".to_string(), TokenType::Argument, TokenSubType::Nothing);
                self.offset += 1;
                continue;
            }

            if self.current_char() == BRACE_CLOSE {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add_ref(&self.token_stack.pop().unwrap());
                self.tokens.add_ref(&self.token_stack.pop().unwrap());
                self.offset += 1;
                continue;
            }

            // trim white-space
            if self.current_char() == WHITESPACE {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens
                    .add(String::new(), TokenType::Whitespace, TokenSubType::Nothing);
                self.offset += 1;
                while (self.current_char() == WHITESPACE) && (!self.eof()) {
                    self.offset += 1;
                }
                continue;
            }

            // multi-character comparators
            let mut double_char: Vec<u8> = self.double_char().bytes().collect();
            double_char.insert(0, b',');
            double_char.push(b',');
            if COMPARATORS.contains(&std::str::from_utf8(&double_char).unwrap()) {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add(
                    self.double_char().to_string(),
                    TokenType::OperatorInfix,
                    TokenSubType::Logical,
                );
                self.offset += 2;
                continue;
            }

            // standard infix operators
            if OPERATORS.contains(&std::str::from_utf8(&[self.current_char()]).unwrap()) {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add(
                    std::str::from_utf8(&[self.current_char()])
                        .unwrap()
                        .to_string(),
                    TokenType::OperatorInfix,
                    TokenSubType::Nothing,
                );
                self.offset += 1;
                continue;
            }

            // standard postfix operators
            if self.current_char() == b'%' {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add(
                    std::str::from_utf8(&[self.current_char()])
                        .unwrap()
                        .to_string(),
                    TokenType::OperatorPostfix,
                    TokenSubType::Nothing,
                );
                self.offset += 1;
                continue;
            }

            // start subexpression or function
            if self.current_char() == PAREN_OPEN {
                if !token.is_empty() {
                    self.token_stack.push(
                        self.tokens
                            .add(
                                token.to_string().to_uppercase(),
                                TokenType::Function,
                                TokenSubType::Start,
                            )
                            .clone(),
                    );
                    token = String::new();
                } else {
                    self.token_stack.push(
                        self.tokens
                            .add(String::new(), TokenType::Subexpression, TokenSubType::Start)
                            .clone(),
                    );
                }
                self.offset += 1;
                continue;
            }

            // function, subexpression, array parameters
            if self.current_char() == COMMA {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                if *self.token_stack.tp() != TokenType::Function {
                    self.tokens.add(
                        std::str::from_utf8(&[self.current_char()])
                            .unwrap()
                            .to_string(),
                        TokenType::OperatorInfix,
                        TokenSubType::Union,
                    );
                } else {
                    self.tokens.add(
                        std::str::from_utf8(&[self.current_char()])
                            .unwrap()
                            .to_string(),
                        TokenType::Argument,
                        TokenSubType::Nothing,
                    );
                }
                self.offset += 1;
                continue;
            }

            // stop subexpression
            if self.current_char() == PAREN_CLOSE {
                if !token.is_empty() {
                    self.tokens
                        .add(token, TokenType::Operand, TokenSubType::Nothing);
                    token = String::new();
                }
                self.tokens.add_ref(&self.token_stack.pop().unwrap());
                self.offset += 1;
                continue;
            }

            // token accumulation
            token += std::str::from_utf8(&[self.current_char()]).unwrap();
            self.offset += 1;
        }
        // dump remaining accumulation
        if !token.is_empty() {
            self.tokens
                .add(token, TokenType::Operand, TokenSubType::Nothing);
        }

        // move all tokens to a new collection, excluding all unnecessary white-space tokens
        let mut tokens2 = Tokens::new();
        while self.tokens.move_next() {
            let token = self.tokens.current().into_inner();
            if token.token_type == TokenType::Whitespace {
                if self.tokens.bof() || self.tokens.eof() {
                } else if !(((self.tokens.previous().token_type == TokenType::Function)
                    && (self.tokens.previous().sub_type == TokenSubType::Stop))
                    || ((self.tokens.previous().token_type == TokenType::Subexpression)
                        && (self.tokens.previous().sub_type == TokenSubType::Stop))
                    || (self.tokens.previous().token_type == TokenType::Operand))
                {
                } else if !(((self.tokens.next().token_type == TokenType::Function)
                    && (self.tokens.next().sub_type == TokenSubType::Start))
                    || ((self.tokens.next().token_type == TokenType::Subexpression)
                        && (self.tokens.next().sub_type == TokenSubType::Start))
                    || (self.tokens.next().token_type == TokenType::Operand))
                {
                } else {
                    tokens2.add(
                        token.value.clone(),
                        TokenType::OperatorInfix,
                        TokenSubType::Intersection,
                    );
                }
                continue;
            }

            tokens2.add_ref(&Token {
                value: token.value.clone(),
                token_type: token.token_type,
                sub_type: token.sub_type,
            })
        }

        while tokens2.move_next() {
            let token = tokens2.current().into_inner();
            if token.token_type == TokenType::OperatorInfix && token.value == "-" {
                if tokens2.bof() {
                    tokens2.set_current_field(None, Some(TokenType::OperatorPrefix), None)
                } else if (tokens2.previous().token_type == TokenType::Function
                    && tokens2.previous().sub_type == TokenSubType::Stop)
                    || (tokens2.previous().token_type == TokenType::Subexpression
                        && tokens2.previous().sub_type == TokenSubType::Stop)
                    || tokens2.previous().token_type == TokenType::OperatorPostfix
                    || tokens2.previous().token_type == TokenType::Operand
                {
                    tokens2.set_current_field(None, None, Some(TokenSubType::Math))
                } else {
                    tokens2.set_current_field(None, Some(TokenType::OperatorPrefix), None)
                }
                continue;
            }
            if token.token_type == TokenType::OperatorInfix && token.value == "+" {
                if tokens2.bof() {
                    tokens2.set_current_field(None, Some(TokenType::Noop), None)
                } else if tokens2.previous().token_type == TokenType::Function
                    && tokens2.previous().sub_type == TokenSubType::Stop
                    || (tokens2.previous().token_type == TokenType::Subexpression
                        && tokens2.previous().sub_type == TokenSubType::Stop
                        || tokens2.previous().token_type == TokenType::OperatorPostfix
                        || tokens2.previous().token_type == TokenType::Operand)
                {
                    tokens2.set_current_field(None, None, Some(TokenSubType::Math))
                } else {
                    tokens2.set_current_field(None, Some(TokenType::Noop), None)
                }
                continue;
            }

            if token.token_type == TokenType::OperatorInfix
                && token.sub_type == TokenSubType::Nothing
            {
                if C_OPERATORS.contains(&&token.value[0..1]) {
                    tokens2.set_current_field(None, None, Some(TokenSubType::Logical))
                } else if token.value == "&" {
                    tokens2.set_current_field(None, None, Some(TokenSubType::Concatenation))
                } else {
                    tokens2.set_current_field(None, None, Some(TokenSubType::Math))
                }
                continue;
            }

            if token.token_type == TokenType::Operand && token.sub_type == TokenSubType::Nothing {
                match token.value.parse::<f64>() {
                    Ok(_) => tokens2.set_current_field(None, None, Some(TokenSubType::Number)),
                    Err(_) => {
                        if token.value.to_uppercase() == "TRUE"
                            || token.value.to_uppercase() == "FALSE"
                        {
                            tokens2.set_current_field(None, None, Some(TokenSubType::Logical))
                        } else {
                            tokens2.set_current_field(None, None, Some(TokenSubType::Text))
                        }
                    }
                }
                continue;
            }
            if token.token_type == TokenType::Function {
                if !token.value.is_empty() && &token.value[0..1] == "@" {
                    tokens2.set_current_field(Some(token.value[1..].to_string()), None, None)
                }
                continue;
            }
        }
        tokens2.reset();
        // move all tokens to a new collection, excluding all noops
        let mut tokens = Tokens::new();
        while tokens2.move_next() {
            let t = tokens2.current().into_inner();
            if t.token_type != TokenType::Noop {
                tokens.add_ref(&Token {
                    value: t.value.clone(),
                    token_type: t.token_type,
                    sub_type: t.sub_type,
                })
            }
        }
        tokens.reset();
        self.tokens = tokens;
    }

    pub fn pretty(&self) -> String {
        let mut indent = 0;
        let mut output = String::from("");
        for item in self.tokens.items.iter() {
            if item.sub_type == TokenSubType::Stop {
                indent -= 1;
            }
            let mut i = 0;
            while i < indent {
                output += "\t";
                i += 1;
            }

            output = format!(
                "{} {} <{:?}> <{:?}> \n",
                output, item.value, item.token_type, item.sub_type
            );
            if item.sub_type == TokenSubType::Start {
                indent += 1;
            }
        }
        output
    }

    // Render provides function to get formatted formula after parsed.
    pub fn render(&self) -> String {
        let mut output = String::from("");
        for item in self.tokens.items.iter() {
            if item.token_type == TokenType::Function && item.sub_type == TokenSubType::Start {
                output = format!("{}({}", output, item.value)
            } else if item.token_type == TokenType::Function && item.sub_type == TokenSubType::Stop
            {
                output += ")"
            } else if item.token_type == TokenType::Subexpression
                && item.sub_type == TokenSubType::Start
            {
                output += "("
            } else if item.token_type == TokenType::Subexpression
                && item.sub_type == TokenSubType::Stop
            {
                output += ")"
            } else if item.token_type == TokenType::Operand && item.sub_type == TokenSubType::Text {
                output = format!("{}\"{}\"", output, item.value)
            } else if item.token_type == TokenType::OperatorInfix
                && item.sub_type == TokenSubType::Intersection
            {
                output += " "
            } else {
                output = format!("{}{}", output, item.value)
            }
        }
        output
    }
}
