//! Interpreter for the College Board's 2026 AP CSP exam-reference pseudocode.

use std::collections::HashMap;
use std::fmt;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Span {
    pub offset: usize,
    pub line: usize,
    pub column: usize,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub span: Span,
    pub message: String,
}

impl fmt::Display for Diagnostic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}:{}: {}",
            self.span.line, self.span.column, self.message
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
enum TokenKind {
    Ident(String),
    Number(f64),
    String(String),
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Comma,
    Newline,
    Semicolon,
    Assign,
    Plus,
    Minus,
    Star,
    Slash,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    Eof,
}

#[derive(Clone, Debug)]
struct Token {
    kind: TokenKind,
    span: Span,
}

struct Lexer<'a> {
    source: &'a str,
    chars: Vec<char>,
    at: usize,
    line: usize,
    column: usize,
}

impl<'a> Lexer<'a> {
    fn new(source: &'a str) -> Self {
        Self {
            source,
            chars: source.chars().collect(),
            at: 0,
            line: 1,
            column: 1,
        }
    }

    fn lex(mut self) -> Result<Vec<Token>, Diagnostic> {
        let mut tokens = Vec::new();
        while let Some(ch) = self.peek() {
            let span = self.span();
            match ch {
                ' ' | '\t' | '\r' => {
                    self.advance();
                }
                '\n' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Newline,
                        span,
                    });
                }
                ';' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Semicolon,
                        span,
                    });
                }
                '/' if self.peek_next() == Some('/') => {
                    while self.peek().is_some_and(|c| c != '\n') {
                        self.advance();
                    }
                }
                '(' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftParen,
                        span,
                    });
                }
                ')' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightParen,
                        span,
                    });
                }
                '{' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftBrace,
                        span,
                    });
                }
                '}' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightBrace,
                        span,
                    });
                }
                '[' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LeftBracket,
                        span,
                    });
                }
                ']' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::RightBracket,
                        span,
                    });
                }
                ',' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Comma,
                        span,
                    });
                }
                '+' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Plus,
                        span,
                    });
                }
                '-' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Minus,
                        span,
                    });
                }
                '*' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Star,
                        span,
                    });
                }
                '/' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Slash,
                        span,
                    });
                }
                '<' => {
                    self.advance();
                    if self.peek() == Some('-') {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::Assign,
                            span,
                        });
                    } else if self.peek() == Some('=') {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::LessEqual,
                            span,
                        });
                    } else if self.peek() == Some('>') {
                        self.advance();
                        tokens.push(Token {
                            kind: TokenKind::NotEqual,
                            span,
                        });
                    } else {
                        tokens.push(Token {
                            kind: TokenKind::Less,
                            span,
                        });
                    }
                }
                '←' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Assign,
                        span,
                    });
                }
                '≤' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::LessEqual,
                        span,
                    });
                }
                '≥' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::GreaterEqual,
                        span,
                    });
                }
                '≠' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::NotEqual,
                        span,
                    });
                }
                '=' => {
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::Equal,
                        span,
                    });
                }
                '>' => {
                    self.advance();
                    let kind = if self.peek() == Some('=') {
                        self.advance();
                        TokenKind::GreaterEqual
                    } else {
                        TokenKind::Greater
                    };
                    tokens.push(Token { kind, span });
                }
                '!' if self.peek_next() == Some('=') => {
                    self.advance();
                    self.advance();
                    tokens.push(Token {
                        kind: TokenKind::NotEqual,
                        span,
                    });
                }
                '"' | '\'' => tokens.push(self.string_token(ch, span)?),
                c if c.is_ascii_digit() => tokens.push(self.number_token(span)?),
                c if is_ident_start(c) => tokens.push(self.ident_token(span)),
                _ => {
                    return Err(Diagnostic {
                        span,
                        message: format!("unexpected character `{ch}`"),
                    });
                }
            }
        }
        tokens.push(Token {
            kind: TokenKind::Eof,
            span: self.span(),
        });
        let _ = self.source;
        Ok(tokens)
    }

    fn string_token(&mut self, quote: char, span: Span) -> Result<Token, Diagnostic> {
        self.advance();
        let mut value = String::new();
        while let Some(ch) = self.peek() {
            if ch == quote {
                self.advance();
                return Ok(Token {
                    kind: TokenKind::String(value),
                    span,
                });
            }
            if ch == '\n' {
                return Err(Diagnostic {
                    span,
                    message: "unterminated string literal".into(),
                });
            }
            if ch == '\\' {
                self.advance();
                match self.peek() {
                    Some('n') => {
                        self.advance();
                        value.push('\n');
                    }
                    Some('t') => {
                        self.advance();
                        value.push('\t');
                    }
                    Some('\\') => {
                        self.advance();
                        value.push('\\');
                    }
                    Some(c) if c == quote => {
                        self.advance();
                        value.push(c);
                    }
                    Some(c) => {
                        self.advance();
                        value.push(c);
                    }
                    None => break,
                }
            } else {
                value.push(ch);
                self.advance();
            }
        }
        Err(Diagnostic {
            span,
            message: "unterminated string literal".into(),
        })
    }

    fn number_token(&mut self, span: Span) -> Result<Token, Diagnostic> {
        let start = self.at;
        while self.peek().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
        }
        if self.peek() == Some('.') && self.peek_next().is_some_and(|c| c.is_ascii_digit()) {
            self.advance();
            while self.peek().is_some_and(|c| c.is_ascii_digit()) {
                self.advance();
            }
        }
        let text: String = self.chars[start..self.at].iter().collect();
        let number = text.parse().map_err(|_| Diagnostic {
            span,
            message: "invalid numeric literal".into(),
        })?;
        Ok(Token {
            kind: TokenKind::Number(number),
            span,
        })
    }

    fn ident_token(&mut self, span: Span) -> Token {
        let start = self.at;
        while self.peek().is_some_and(is_ident_continue) {
            self.advance();
        }
        let name: String = self.chars[start..self.at].iter().collect();
        let kind = if name.eq_ignore_ascii_case("MOD") {
            TokenKind::Mod
        } else {
            TokenKind::Ident(name)
        };
        Token { kind, span }
    }

    fn peek(&self) -> Option<char> {
        self.chars.get(self.at).copied()
    }
    fn peek_next(&self) -> Option<char> {
        self.chars.get(self.at + 1).copied()
    }
    fn span(&self) -> Span {
        Span {
            offset: self.at,
            line: self.line,
            column: self.column,
        }
    }
    fn advance(&mut self) -> Option<char> {
        let ch = self.peek()?;
        self.at += 1;
        if ch == '\n' {
            self.line += 1;
            self.column = 1;
        } else {
            self.column += 1;
        }
        Some(ch)
    }
}

fn is_ident_start(c: char) -> bool {
    c.is_ascii_alphabetic() || c == '_'
}
fn is_ident_continue(c: char) -> bool {
    c.is_ascii_alphanumeric() || c == '_'
}

#[derive(Clone, Debug)]
pub struct Program {
    procedures: HashMap<String, Procedure>,
    statements: Vec<Stmt>,
}

#[derive(Clone, Debug)]
struct Procedure {
    name: String,
    parameters: Vec<String>,
    body: Vec<Stmt>,
}

#[derive(Clone, Debug)]
enum Stmt {
    Assign {
        name: String,
        index: Option<Expr>,
        value: Expr,
    },
    Expression(Expr),
    If {
        condition: Expr,
        then_body: Vec<Stmt>,
        else_body: Vec<Stmt>,
    },
    RepeatTimes {
        count: Expr,
        body: Vec<Stmt>,
    },
    RepeatUntil {
        condition: Expr,
        body: Vec<Stmt>,
    },
    ForEach {
        item: String,
        list: Expr,
        body: Vec<Stmt>,
    },
    Return(Expr),
}

#[derive(Clone, Debug)]
enum Expr {
    Number(f64),
    String(String),
    Boolean(bool),
    Variable(String),
    List(Vec<Expr>),
    Index(Box<Expr>, Box<Expr>),
    Unary(UnaryOp, Box<Expr>),
    Binary(Box<Expr>, BinaryOp, Box<Expr>),
    Call(String, Vec<Expr>),
}

#[derive(Clone, Copy, Debug)]
enum UnaryOp {
    Negate,
    Not,
}
#[derive(Clone, Copy, Debug)]
enum BinaryOp {
    Add,
    Subtract,
    Multiply,
    Divide,
    Mod,
    Equal,
    NotEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,
    And,
    Or,
}

struct Parser {
    tokens: Vec<Token>,
    at: usize,
}

impl Parser {
    fn parse(tokens: Vec<Token>) -> Result<Program, Diagnostic> {
        let mut parser = Self { tokens, at: 0 };
        let mut procedures = HashMap::new();
        let mut statements = Vec::new();
        parser.separators();
        while !parser.is(&TokenKind::Eof) {
            if parser.word("PROCEDURE") {
                let procedure = parser.procedure()?;
                let key = procedure.name.to_ascii_lowercase();
                if procedures.insert(key, procedure.clone()).is_some() {
                    return Err(parser.error("procedure is declared more than once"));
                }
            } else {
                statements.push(parser.statement()?);
            }
            parser.separators();
        }
        Ok(Program {
            procedures,
            statements,
        })
    }

    fn procedure(&mut self) -> Result<Procedure, Diagnostic> {
        self.expect_word("PROCEDURE")?;
        let name = self.identifier()?;
        self.expect(&TokenKind::LeftParen, "expected `(` after procedure name")?;
        let mut parameters = Vec::new();
        if !self.is(&TokenKind::RightParen) {
            loop {
                parameters.push(self.identifier()?);
                if !self.take(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(
            &TokenKind::RightParen,
            "expected `)` after procedure parameters",
        )?;
        let body = self.block()?;
        Ok(Procedure {
            name,
            parameters,
            body,
        })
    }

    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        self.separators();
        if self.word("IF") {
            self.advance();
            let condition = self.parenthesized_expression()?;
            let then_body = self.block()?;
            self.separators();
            let else_body = if self.word("ELSE") {
                self.advance();
                self.block()?
            } else {
                Vec::new()
            };
            return Ok(Stmt::If {
                condition,
                then_body,
                else_body,
            });
        }
        if self.word("REPEAT") {
            self.advance();
            if self.word("UNTIL") {
                self.advance();
                let condition = self.parenthesized_expression()?;
                return Ok(Stmt::RepeatUntil {
                    condition,
                    body: self.block()?,
                });
            }
            let count = self.expression(0)?;
            self.expect_word("TIMES")?;
            return Ok(Stmt::RepeatTimes {
                count,
                body: self.block()?,
            });
        }
        if self.word("FOR") {
            self.advance();
            self.expect_word("EACH")?;
            let item = self.identifier()?;
            self.expect_word("IN")?;
            let list = self.expression(0)?;
            return Ok(Stmt::ForEach {
                item,
                list,
                body: self.block()?,
            });
        }
        if self.word("RETURN") {
            self.advance();
            self.take(&TokenKind::LeftParen);
            let expression = self.expression(0)?;
            self.take(&TokenKind::RightParen);
            return Ok(Stmt::Return(expression));
        }
        if let TokenKind::Ident(name) = self.current().kind.clone() {
            let saved = self.at;
            self.advance();
            let index = if self.take(&TokenKind::LeftBracket) {
                let idx = self.expression(0)?;
                self.expect(&TokenKind::RightBracket, "expected `]` after list index")?;
                Some(idx)
            } else {
                None
            };
            if self.take(&TokenKind::Assign) {
                let value = self.expression(0)?;
                return Ok(Stmt::Assign { name, index, value });
            }
            self.at = saved;
        }
        let expression = self.expression(0)?;
        if matches!(expression, Expr::Call(_, _)) {
            Ok(Stmt::Expression(expression))
        } else {
            Err(self.error("expected assignment or procedure call"))
        }
    }

    fn block(&mut self) -> Result<Vec<Stmt>, Diagnostic> {
        while self.is(&TokenKind::Newline) {
            self.advance();
        }
        self.expect(&TokenKind::LeftBrace, "expected `{` to begin block")?;
        let mut statements = Vec::new();
        self.separators();
        while !self.is(&TokenKind::RightBrace) && !self.is(&TokenKind::Eof) {
            statements.push(self.statement()?);
            self.separators();
        }
        self.expect(&TokenKind::RightBrace, "expected `}` to end block")?;
        Ok(statements)
    }

    fn parenthesized_expression(&mut self) -> Result<Expr, Diagnostic> {
        self.expect(&TokenKind::LeftParen, "expected `(` before condition")?;
        let expr = self.expression(0)?;
        self.expect(&TokenKind::RightParen, "expected `)` after condition")?;
        Ok(expr)
    }

    fn expression(&mut self, min_precedence: u8) -> Result<Expr, Diagnostic> {
        let mut left = self.prefix()?;
        loop {
            if self.take(&TokenKind::LeftBracket) {
                let index = self.expression(0)?;
                self.expect(&TokenKind::RightBracket, "expected `]` after index")?;
                left = Expr::Index(Box::new(left), Box::new(index));
                continue;
            }
            if self.is(&TokenKind::LeftParen) {
                if let Expr::Variable(name) = left {
                    self.advance();
                    let args = self.arguments_after_open_paren()?;
                    left = Expr::Call(name, args);
                    continue;
                }
            }
            let Some((op, precedence)) = self.binary_operator() else {
                break;
            };
            if precedence < min_precedence {
                break;
            }
            self.advance();
            let right = self.expression(precedence + 1)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }

    fn prefix(&mut self) -> Result<Expr, Diagnostic> {
        let token = self.current().clone();
        match token.kind {
            TokenKind::Number(value) => {
                self.advance();
                Ok(Expr::Number(value))
            }
            TokenKind::String(value) => {
                self.advance();
                Ok(Expr::String(value))
            }
            TokenKind::Ident(ref name) if name.eq_ignore_ascii_case("TRUE") => {
                self.advance();
                Ok(Expr::Boolean(true))
            }
            TokenKind::Ident(ref name) if name.eq_ignore_ascii_case("FALSE") => {
                self.advance();
                Ok(Expr::Boolean(false))
            }
            TokenKind::Ident(ref name) if name.eq_ignore_ascii_case("NOT") => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Not, Box::new(self.expression(6)?)))
            }
            TokenKind::Ident(name) => {
                self.advance();
                Ok(Expr::Variable(name))
            }
            TokenKind::Minus => {
                self.advance();
                Ok(Expr::Unary(UnaryOp::Negate, Box::new(self.expression(6)?)))
            }
            TokenKind::LeftParen => {
                self.advance();
                let expr = self.expression(0)?;
                self.expect(&TokenKind::RightParen, "expected `)`")?;
                Ok(expr)
            }
            TokenKind::LeftBracket => {
                self.advance();
                let mut items = Vec::new();
                if !self.is(&TokenKind::RightBracket) {
                    loop {
                        items.push(self.expression(0)?);
                        if !self.take(&TokenKind::Comma) {
                            break;
                        }
                    }
                }
                self.expect(&TokenKind::RightBracket, "expected `]` after list literal")?;
                Ok(Expr::List(items))
            }
            _ => Err(Diagnostic {
                span: token.span,
                message: "expected expression".into(),
            }),
        }
    }

    fn arguments_after_open_paren(&mut self) -> Result<Vec<Expr>, Diagnostic> {
        let mut args = Vec::new();
        if !self.is(&TokenKind::RightParen) {
            loop {
                args.push(self.expression(0)?);
                if !self.take(&TokenKind::Comma) {
                    break;
                }
            }
        }
        self.expect(&TokenKind::RightParen, "expected `)` after arguments")?;
        Ok(args)
    }

    fn binary_operator(&self) -> Option<(BinaryOp, u8)> {
        let item = match self.current().kind {
            TokenKind::Ident(ref word) if word.eq_ignore_ascii_case("OR") => (BinaryOp::Or, 1),
            TokenKind::Ident(ref word) if word.eq_ignore_ascii_case("AND") => (BinaryOp::And, 2),
            TokenKind::Equal => (BinaryOp::Equal, 3),
            TokenKind::NotEqual => (BinaryOp::NotEqual, 3),
            TokenKind::Less => (BinaryOp::Less, 3),
            TokenKind::LessEqual => (BinaryOp::LessEqual, 3),
            TokenKind::Greater => (BinaryOp::Greater, 3),
            TokenKind::GreaterEqual => (BinaryOp::GreaterEqual, 3),
            TokenKind::Plus => (BinaryOp::Add, 4),
            TokenKind::Minus => (BinaryOp::Subtract, 4),
            TokenKind::Star => (BinaryOp::Multiply, 5),
            TokenKind::Slash => (BinaryOp::Divide, 5),
            TokenKind::Mod => (BinaryOp::Mod, 5),
            _ => return None,
        };
        Some(item)
    }

    fn identifier(&mut self) -> Result<String, Diagnostic> {
        match self.current().kind.clone() {
            TokenKind::Ident(name) => {
                self.advance();
                Ok(name)
            }
            _ => Err(self.error("expected identifier")),
        }
    }
    fn word(&self, expected: &str) -> bool {
        matches!(&self.current().kind, TokenKind::Ident(s) if s.eq_ignore_ascii_case(expected))
    }
    fn expect_word(&mut self, word: &str) -> Result<(), Diagnostic> {
        if self.word(word) {
            self.advance();
            Ok(())
        } else {
            Err(self.error(&format!("expected `{word}`")))
        }
    }
    fn expect(&mut self, kind: &TokenKind, message: &str) -> Result<(), Diagnostic> {
        if self.take(kind) {
            Ok(())
        } else {
            Err(self.error(message))
        }
    }
    fn take(&mut self, kind: &TokenKind) -> bool {
        if self.is(kind) {
            self.advance();
            true
        } else {
            false
        }
    }
    fn is(&self, kind: &TokenKind) -> bool {
        std::mem::discriminant(&self.current().kind) == std::mem::discriminant(kind)
    }
    fn current(&self) -> &Token {
        &self.tokens[self.at.min(self.tokens.len() - 1)]
    }
    fn advance(&mut self) {
        if !self.is(&TokenKind::Eof) {
            self.at += 1;
        }
    }
    fn separators(&mut self) {
        while self.is(&TokenKind::Newline) || self.is(&TokenKind::Semicolon) {
            self.advance();
        }
    }
    fn error(&self, message: &str) -> Diagnostic {
        Diagnostic {
            span: self.current().span,
            message: message.to_owned(),
        }
    }
}

pub fn parse(source: &str) -> Result<Program, Vec<Diagnostic>> {
    let tokens = Lexer::new(source).lex().map_err(|e| vec![e])?;
    Parser::parse(tokens).map_err(|e| vec![e])
}

#[derive(Clone, Debug)]
pub struct CheckedProgram(Program);

pub fn check(program: Program) -> Result<CheckedProgram, Vec<Diagnostic>> {
    let mut diagnostics = Vec::new();
    for procedure in program.procedures.values() {
        let mut names = std::collections::HashSet::new();
        for name in &procedure.parameters {
            if !names.insert(name.to_ascii_lowercase()) {
                diagnostics.push(Diagnostic {
                    span: Span::default(),
                    message: format!(
                        "duplicate parameter `{name}` in procedure `{}`",
                        procedure.name
                    ),
                });
            }
        }
        check_statements(&procedure.body, &program, true, &mut diagnostics);
    }
    check_statements(&program.statements, &program, false, &mut diagnostics);
    if diagnostics.is_empty() {
        Ok(CheckedProgram(program))
    } else {
        Err(diagnostics)
    }
}

fn check_statements(
    statements: &[Stmt],
    program: &Program,
    in_procedure: bool,
    diagnostics: &mut Vec<Diagnostic>,
) {
    for statement in statements {
        match statement {
            Stmt::Assign { index, value, .. } => {
                if let Some(index) = index {
                    check_expression(index, program, diagnostics);
                }
                check_expression(value, program, diagnostics);
            }
            Stmt::Expression(expression) => check_expression(expression, program, diagnostics),
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                check_expression(condition, program, diagnostics);
                check_statements(then_body, program, in_procedure, diagnostics);
                check_statements(else_body, program, in_procedure, diagnostics);
            }
            Stmt::RepeatTimes { count, body } => {
                check_expression(count, program, diagnostics);
                check_statements(body, program, in_procedure, diagnostics);
            }
            Stmt::RepeatUntil { condition, body } => {
                check_expression(condition, program, diagnostics);
                check_statements(body, program, in_procedure, diagnostics);
            }
            Stmt::ForEach { list, body, .. } => {
                check_expression(list, program, diagnostics);
                check_statements(body, program, in_procedure, diagnostics);
            }
            Stmt::Return(expression) => {
                if !in_procedure {
                    diagnostics.push(Diagnostic {
                        span: Span::default(),
                        message: "RETURN is only valid inside a procedure".into(),
                    });
                }
                check_expression(expression, program, diagnostics);
            }
        }
    }
}

fn check_expression(expression: &Expr, program: &Program, diagnostics: &mut Vec<Diagnostic>) {
    match expression {
        Expr::List(items) => {
            for item in items {
                check_expression(item, program, diagnostics);
            }
        }
        Expr::Index(list, index) => {
            check_expression(list, program, diagnostics);
            check_expression(index, program, diagnostics);
        }
        Expr::Unary(_, value) => check_expression(value, program, diagnostics),
        Expr::Binary(left, _, right) => {
            check_expression(left, program, diagnostics);
            check_expression(right, program, diagnostics);
        }
        Expr::Call(name, args) => {
            for arg in args {
                check_expression(arg, program, diagnostics);
            }
            let expected = match name.to_ascii_uppercase().as_str() {
                "DISPLAY" | "LENGTH" | "CAN_MOVE" => Some(1),
                "INPUT" | "MOVE_FORWARD" | "ROTATE_LEFT" | "ROTATE_RIGHT" => Some(0),
                "RANDOM" | "APPEND" | "REMOVE" => Some(2),
                "INSERT" => Some(3),
                _ => program
                    .procedures
                    .get(&name.to_ascii_lowercase())
                    .map(|p| p.parameters.len()),
            };
            match expected {
                Some(arity) if arity == args.len() => {}
                Some(arity) => diagnostics.push(Diagnostic {
                    span: Span::default(),
                    message: format!(
                        "`{name}` expects {arity} argument(s), received {}",
                        args.len()
                    ),
                }),
                None => diagnostics.push(Diagnostic {
                    span: Span::default(),
                    message: format!("unknown procedure `{name}`"),
                }),
            }
        }
        Expr::Number(_) | Expr::String(_) | Expr::Boolean(_) | Expr::Variable(_) => {}
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Number(f64),
    Boolean(bool),
    String(String),
    List(Vec<Value>),
    Null,
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Number(n)
                if n.fract() == 0.0
                    && *n >= i64::MIN as f64
                    && *n < 9_223_372_036_854_775_808.0 =>
            {
                write!(f, "{}", *n as i64)
            }
            Self::Number(n) => write!(f, "{n}"),
            Self::Boolean(b) => write!(f, "{}", if *b { "true" } else { "false" }),
            Self::String(s) => write!(f, "{s}"),
            Self::List(items) => {
                write!(f, "[")?;
                for (i, item) in items.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{item}")?;
                }
                write!(f, "]")
            }
            Self::Null => write!(f, "null"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
    Forward,
    Backward,
}

pub trait RobotGrid {
    fn can_move(&self, direction: Direction) -> bool;
    /// Returns `false` without moving when the next square is blocked or outside the grid.
    fn move_forward(&mut self) -> bool;
    fn rotate_left(&mut self);
    fn rotate_right(&mut self);
}

pub trait Host {
    fn input(&mut self) -> Result<Value, String>;
    fn display(&mut self, value: &Value) -> Result<(), String>;
    fn random_integer(&mut self, lower: i64, upper: i64) -> Result<i64, String>;
    fn robot(&mut self) -> Option<&mut dyn RobotGrid> {
        None
    }
}

#[derive(Clone, Debug, Default)]
pub struct Execution {
    pub variables: HashMap<String, Value>,
    pub steps: usize,
    pub terminated_by_robot: bool,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct RuntimeError {
    pub message: String,
}
impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for RuntimeError {}

#[derive(Clone, Debug)]
enum Flow {
    Continue,
    Return(Value),
    Terminate,
}

#[derive(Default)]
struct Scope {
    local: Option<HashMap<String, Value>>,
}

pub struct Interpreter<H: Host> {
    host: H,
    globals: HashMap<String, Value>,
    procedures: HashMap<String, Procedure>,
    steps: usize,
    step_limit: usize,
    terminated_by_robot: bool,
}

impl<H: Host> Interpreter<H> {
    pub fn new(host: H) -> Self {
        Self {
            host,
            globals: HashMap::new(),
            procedures: HashMap::new(),
            steps: 0,
            step_limit: 1_000_000,
            terminated_by_robot: false,
        }
    }
    pub fn with_step_limit(mut self, limit: usize) -> Self {
        self.step_limit = limit.max(1);
        self
    }
    pub fn host(&self) -> &H {
        &self.host
    }
    pub fn host_mut(&mut self) -> &mut H {
        &mut self.host
    }

    pub fn run(&mut self, program: &CheckedProgram) -> Result<Execution, RuntimeError> {
        self.globals.clear();
        self.procedures = program.0.procedures.clone();
        self.steps = 0;
        self.terminated_by_robot = false;
        let mut scope = Scope::default();
        let flow = self.execute_block(&program.0.statements, &mut scope)?;
        if matches!(flow, Flow::Return(_)) {
            return Err(self.error("RETURN cannot be used outside a procedure"));
        }
        Ok(Execution {
            variables: self.globals.clone(),
            steps: self.steps,
            terminated_by_robot: self.terminated_by_robot || matches!(flow, Flow::Terminate),
        })
    }

    fn execute_block(
        &mut self,
        statements: &[Stmt],
        scope: &mut Scope,
    ) -> Result<Flow, RuntimeError> {
        for statement in statements {
            self.tick()?;
            match self.execute(statement, scope)? {
                Flow::Continue => {}
                flow => return Ok(flow),
            }
            if self.terminated_by_robot {
                return Ok(Flow::Terminate);
            }
        }
        Ok(Flow::Continue)
    }

    fn execute(&mut self, statement: &Stmt, scope: &mut Scope) -> Result<Flow, RuntimeError> {
        match statement {
            Stmt::Assign {
                name, index, value, ..
            } => {
                let value = self.evaluate(value, scope)?;
                if let Some(index) = index {
                    let index = Self::list_index(self.evaluate(index, scope)?)?;
                    let mut target = self.variable(name, scope)?;
                    let Value::List(items) = &mut target else {
                        return Err(self.error("indexed assignment requires a list"));
                    };
                    let slot = checked_index(index, items.len())?;
                    items[slot] = value;
                    self.assign(name, target, scope);
                } else {
                    self.assign(name, value, scope);
                }
                Ok(Flow::Continue)
            }
            Stmt::Expression(expr) => {
                let _ = self.evaluate(expr, scope)?;
                Ok(Flow::Continue)
            }
            Stmt::If {
                condition,
                then_body,
                else_body,
            } => {
                if Self::boolean(self.evaluate(condition, scope)?)? {
                    self.execute_block(then_body, scope)
                } else {
                    self.execute_block(else_body, scope)
                }
            }
            Stmt::RepeatTimes { count, body } => {
                let count = Self::number(self.evaluate(count, scope)?)?;
                if count < 0.0 || count.fract() != 0.0 {
                    return Err(self.error("REPEAT TIMES requires a nonnegative integer"));
                }
                for _ in 0..(count as usize) {
                    self.tick()?;
                    match self.execute_block(body, scope)? {
                        Flow::Continue => {}
                        flow => return Ok(flow),
                    }
                }
                Ok(Flow::Continue)
            }
            Stmt::RepeatUntil { condition, body } => loop {
                self.tick()?;
                match self.execute_block(body, scope)? {
                    Flow::Continue => {}
                    flow => return Ok(flow),
                }
                if Self::boolean(self.evaluate(condition, scope)?)? {
                    return Ok(Flow::Continue);
                }
            },
            Stmt::ForEach { item, list, body } => {
                let Value::List(values) = self.evaluate(list, scope)? else {
                    return Err(self.error("FOR EACH requires a list"));
                };
                for value in values {
                    self.assign(item, value, scope);
                    match self.execute_block(body, scope)? {
                        Flow::Continue => {}
                        flow => return Ok(flow),
                    }
                }
                Ok(Flow::Continue)
            }
            Stmt::Return(expr) => Ok(Flow::Return(self.evaluate(expr, scope)?)),
        }
    }

    fn evaluate(&mut self, expr: &Expr, scope: &mut Scope) -> Result<Value, RuntimeError> {
        match expr {
            Expr::Number(n) => Ok(Value::Number(*n)),
            Expr::String(s) => Ok(Value::String(s.clone())),
            Expr::Boolean(b) => Ok(Value::Boolean(*b)),
            Expr::Variable(name) => match name.to_ascii_lowercase().as_str() {
                "left" | "right" | "forward" | "backward" => {
                    Ok(Value::String(name.to_ascii_lowercase()))
                }
                _ => self.variable(name, scope),
            },
            Expr::List(values) => values
                .iter()
                .map(|e| self.evaluate(e, scope))
                .collect::<Result<Vec<_>, _>>()
                .map(Value::List),
            Expr::Index(value, index) => {
                let Value::List(items) = self.evaluate(value, scope)? else {
                    return Err(self.error("indexing requires a list"));
                };
                let index = Self::list_index(self.evaluate(index, scope)?)?;
                Ok(items[checked_index(index, items.len())?].clone())
            }
            Expr::Unary(op, value) => {
                let value = self.evaluate(value, scope)?;
                match op {
                    UnaryOp::Negate => Ok(Value::Number(-Self::number(value)?)),
                    UnaryOp::Not => Ok(Value::Boolean(!Self::boolean(value)?)),
                }
            }
            Expr::Binary(left, op, right) => {
                let left = self.evaluate(left, scope)?;
                if matches!(op, BinaryOp::And) && !Self::boolean(left.clone())? {
                    return Ok(Value::Boolean(false));
                }
                if matches!(op, BinaryOp::Or) && Self::boolean(left.clone())? {
                    return Ok(Value::Boolean(true));
                }
                let right = self.evaluate(right, scope)?;
                self.binary(left, *op, right)
            }
            Expr::Call(name, args) => self.call(name, args, scope),
        }
    }

    fn call(
        &mut self,
        name: &str,
        args: &[Expr],
        scope: &mut Scope,
    ) -> Result<Value, RuntimeError> {
        let key = name.to_ascii_uppercase();
        match key.as_str() {
            "DISPLAY" => {
                self.arity(name, args, 1)?;
                let value = self.evaluate(&args[0], scope)?;
                self.host.display(&value).map_err(|e| self.error(&e))?;
                Ok(Value::Null)
            }
            "INPUT" => {
                self.arity(name, args, 0)?;
                self.host.input().map_err(|e| self.error(&e))
            }
            "RANDOM" => {
                self.arity(name, args, 2)?;
                let lo = Self::integer(self.evaluate(&args[0], scope)?)?;
                let hi = Self::integer(self.evaluate(&args[1], scope)?)?;
                if lo > hi {
                    return Err(self.error("RANDOM lower bound exceeds upper bound"));
                }
                Ok(Value::Number(
                    self.host
                        .random_integer(lo, hi)
                        .map_err(|e| self.error(&e))? as f64,
                ))
            }
            "LENGTH" => {
                self.arity(name, args, 1)?;
                match self.evaluate(&args[0], scope)? {
                    Value::List(items) => Ok(Value::Number(items.len() as f64)),
                    _ => Err(self.error("LENGTH requires a list")),
                }
            }
            "APPEND" | "INSERT" | "REMOVE" => self.mutate_list(&key, args, scope),
            "CAN_MOVE" => {
                self.arity(name, args, 1)?;
                let direction = Self::direction(self.evaluate(&args[0], scope)?)?;
                let robot = match self.host.robot() {
                    Some(robot) => robot,
                    None => {
                        return Err(Self::runtime_error(
                            "no robot grid was provided by the host",
                        ));
                    }
                };
                let can_move = robot.can_move(direction);
                Ok(Value::Boolean(can_move))
            }
            "MOVE_FORWARD" | "ROTATE_LEFT" | "ROTATE_RIGHT" => {
                self.arity(name, args, 0)?;
                let robot = match self.host.robot() {
                    Some(robot) => robot,
                    None => {
                        return Err(Self::runtime_error(
                            "no robot grid was provided by the host",
                        ));
                    }
                };
                match key.as_str() {
                    "MOVE_FORWARD" => {
                        if !robot.move_forward() {
                            self.terminated_by_robot = true;
                        }
                    }
                    "ROTATE_LEFT" => robot.rotate_left(),
                    _ => robot.rotate_right(),
                }
                Ok(Value::Null)
            }
            _ => {
                let Some(procedure) = self.procedures.get(&name.to_ascii_lowercase()).cloned()
                else {
                    return Err(self.error(&format!("unknown procedure `{name}`")));
                };
                self.arity(name, args, procedure.parameters.len())?;
                let mut local = HashMap::new();
                for (param, arg) in procedure.parameters.iter().zip(args) {
                    local.insert(param.to_ascii_lowercase(), self.evaluate(arg, scope)?);
                }
                let mut procedure_scope = Scope { local: Some(local) };
                match self.execute_block(&procedure.body, &mut procedure_scope)? {
                    Flow::Return(value) => Ok(value),
                    Flow::Continue => Ok(Value::Null),
                    Flow::Terminate => Ok(Value::Null),
                }
            }
        }
    }

    fn mutate_list(
        &mut self,
        kind: &str,
        args: &[Expr],
        scope: &mut Scope,
    ) -> Result<Value, RuntimeError> {
        let arity = if kind == "INSERT" { 3 } else { 2 };
        self.arity(kind, args, arity)?;
        let Expr::Variable(name) = &args[0] else {
            return Err(self.error("list operation requires a list variable as its first argument"));
        };
        let mut value = self.variable(name, scope)?;
        let Value::List(items) = &mut value else {
            return Err(self.error("list operation requires a list"));
        };
        match kind {
            "APPEND" => {
                let item = self.evaluate(&args[1], scope)?;
                items.push(item);
            }
            "INSERT" => {
                let position = Self::list_index(self.evaluate(&args[1], scope)?)?;
                let item = self.evaluate(&args[2], scope)?;
                let slot = checked_index(position, items.len())?;
                items.insert(slot, item);
            }
            "REMOVE" => {
                let position = Self::list_index(self.evaluate(&args[1], scope)?)?;
                let slot = checked_index(position, items.len())?;
                items.remove(slot);
            }
            _ => unreachable!(),
        }
        self.assign(name, value, scope);
        Ok(Value::Null)
    }

    fn binary(&self, left: Value, op: BinaryOp, right: Value) -> Result<Value, RuntimeError> {
        use BinaryOp::*;
        match op {
            Equal => Ok(Value::Boolean(left == right)),
            NotEqual => Ok(Value::Boolean(left != right)),
            Add => match (left, right) {
                (Value::Number(a), Value::Number(b)) => Ok(Value::Number(a + b)),
                (Value::String(mut a), Value::String(b)) => {
                    a.push_str(&b);
                    Ok(Value::String(a))
                }
                _ => Err(self.error("`+` requires two numbers or two strings")),
            },
            Subtract => Ok(Value::Number(Self::number(left)? - Self::number(right)?)),
            Multiply => Ok(Value::Number(Self::number(left)? * Self::number(right)?)),
            Divide => {
                let divisor = Self::number(right)?;
                if divisor == 0.0 {
                    Err(self.error("division by zero"))
                } else {
                    Ok(Value::Number(Self::number(left)? / divisor))
                }
            }
            Mod => {
                let a = Self::integer(left)?;
                let b = Self::integer(right)?;
                if a < 0 || b <= 0 {
                    Err(self.error("MOD requires a nonnegative dividend and a positive divisor"))
                } else {
                    Ok(Value::Number((a % b) as f64))
                }
            }
            Less | LessEqual | Greater | GreaterEqual => {
                let result = match (&left, &right) {
                    (Value::Number(a), Value::Number(b)) => match op {
                        Less => a < b,
                        LessEqual => a <= b,
                        Greater => a > b,
                        _ => a >= b,
                    },
                    (Value::String(a), Value::String(b)) => match op {
                        Less => a < b,
                        LessEqual => a <= b,
                        Greater => a > b,
                        _ => a >= b,
                    },
                    _ => {
                        return Err(self.error(
                            "ordered comparison requires values of the same comparable type",
                        ));
                    }
                };
                Ok(Value::Boolean(result))
            }
            And => Ok(Value::Boolean(
                Self::boolean(left)? && Self::boolean(right)?,
            )),
            Or => Ok(Value::Boolean(
                Self::boolean(left)? || Self::boolean(right)?,
            )),
        }
    }

    fn variable(&self, name: &str, scope: &Scope) -> Result<Value, RuntimeError> {
        let key = name.to_ascii_lowercase();
        if let Some(value) = scope.local.as_ref().and_then(|values| values.get(&key)) {
            return Ok(value.clone());
        }
        self.globals
            .get(&key)
            .cloned()
            .ok_or_else(|| self.error(&format!("unknown variable `{name}`")))
    }
    fn assign(&mut self, name: &str, value: Value, scope: &mut Scope) {
        let key = name.to_ascii_lowercase();
        if let Some(local) = scope.local.as_mut() {
            if local.contains_key(&key) || !self.globals.contains_key(&key) {
                local.insert(key, value);
            } else {
                self.globals.insert(key, value);
            }
        } else {
            self.globals.insert(key, value);
        }
    }
    fn list_index(value: Value) -> Result<usize, RuntimeError> {
        let n = Self::integer(value)?;
        if n < 1 {
            Err(Self::runtime_error("list indices start at 1"))
        } else {
            Ok(n as usize)
        }
    }
    fn number(value: Value) -> Result<f64, RuntimeError> {
        if let Value::Number(n) = value {
            Ok(n)
        } else {
            Err(Self::runtime_error("expected a number"))
        }
    }
    fn integer(value: Value) -> Result<i64, RuntimeError> {
        let n = Self::number(value)?;
        if !n.is_finite()
            || n.fract() != 0.0
            || n < -9_223_372_036_854_775_808.0
            || n >= 9_223_372_036_854_775_808.0
        {
            Err(Self::runtime_error("expected an integer"))
        } else {
            Ok(n as i64)
        }
    }
    fn boolean(value: Value) -> Result<bool, RuntimeError> {
        if let Value::Boolean(b) = value {
            Ok(b)
        } else {
            Err(Self::runtime_error("expected a Boolean"))
        }
    }
    fn direction(value: Value) -> Result<Direction, RuntimeError> {
        let Value::String(s) = value else {
            return Err(Self::runtime_error(
                "CAN_MOVE direction must be left, right, forward, or backward",
            ));
        };
        match s.to_ascii_lowercase().as_str() {
            "left" => Ok(Direction::Left),
            "right" => Ok(Direction::Right),
            "forward" => Ok(Direction::Forward),
            "backward" => Ok(Direction::Backward),
            _ => Err(Self::runtime_error(
                "CAN_MOVE direction must be left, right, forward, or backward",
            )),
        }
    }
    fn runtime_error(message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_owned(),
        }
    }
    fn arity(&self, name: &str, args: &[Expr], expected: usize) -> Result<(), RuntimeError> {
        if args.len() == expected {
            Ok(())
        } else {
            Err(self.error(&format!(
                "`{name}` expects {expected} argument(s), received {}",
                args.len()
            )))
        }
    }
    fn tick(&mut self) -> Result<(), RuntimeError> {
        self.steps += 1;
        if self.steps > self.step_limit {
            Err(self.error("execution step limit exceeded"))
        } else {
            Ok(())
        }
    }
    fn error(&self, message: &str) -> RuntimeError {
        RuntimeError {
            message: message.to_owned(),
        }
    }
}

fn checked_index(index: usize, len: usize) -> Result<usize, RuntimeError> {
    if index == 0 || index > len {
        Err(RuntimeError {
            message: format!("list index {index} is outside 1..={len}; program terminated"),
        })
    } else {
        Ok(index - 1)
    }
}

#[cfg(test)]
mod tests {
    use super::{Execution, Host, Interpreter, Value, check, parse};

    struct TestHost;

    impl Host for TestHost {
        fn input(&mut self) -> Result<Value, String> {
            Err("unexpected input".into())
        }

        fn display(&mut self, _value: &Value) -> Result<(), String> {
            Ok(())
        }

        fn random_integer(&mut self, lower: i64, _upper: i64) -> Result<i64, String> {
            Ok(lower)
        }
    }

    fn run(source: &str) -> Execution {
        let program = check(parse(source).expect("source should parse"))
            .expect("source should pass semantic checks");
        Interpreter::new(TestHost)
            .run(&program)
            .expect("program should run")
    }

    #[test]
    fn if_block_can_open_on_next_line() {
        let execution = run("IF (TRUE)\n{\n  result ← 7\n}");
        assert_eq!(execution.variables.get("result"), Some(&Value::Number(7.0)));
    }

    #[test]
    fn if_block_accepts_inline_open_brace() {
        let execution = run("IF (TRUE) { result ← 7 }");
        assert_eq!(execution.variables.get("result"), Some(&Value::Number(7.0)));
    }

    #[test]
    fn for_each_block_can_open_on_next_line() {
        let execution = run("total ← 0\nFOR EACH item IN [1, 2]\n{\n  total ← total + item\n}");
        assert_eq!(execution.variables.get("total"), Some(&Value::Number(3.0)));
    }

    #[test]
    fn for_each_block_accepts_inline_open_brace() {
        let execution = run("total ← 0\nFOR EACH item IN [1, 2] { total ← total + item }");
        assert_eq!(execution.variables.get("total"), Some(&Value::Number(3.0)));
    }

    #[test]
    fn repeat_until_block_can_open_on_next_line() {
        let execution = run("count ← 0\nREPEAT UNTIL (count = 2)\n{\n  count ← count + 1\n}");
        assert_eq!(execution.variables.get("count"), Some(&Value::Number(2.0)));
    }

    #[test]
    fn repeat_until_block_accepts_inline_open_brace() {
        let execution = run("count ← 0\nREPEAT UNTIL (count = 2) { count ← count + 1 }");
        assert_eq!(execution.variables.get("count"), Some(&Value::Number(2.0)));
    }
}
