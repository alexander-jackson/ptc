use std::collections::VecDeque;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Indent,
    Unindent,
    Identifier { name: String },
    Pass,
    If,
    While,
    Def,
    Return,

    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    LogicalOr,
    LogicalAnd,
    LogicalNot,
    Less,
    Greater,
    LessOrEqual,
    GreaterOrEqual,
    Equal,
    NotEqual,

    Integer { value: u32 },
    LPar,
    RPar,
    Colon,
    Semicolon,
    Comma,
    Newline,
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

pub struct Lexer<T: Iterator<Item = (usize, char)>> {
    chars: T,
    lookahead: Option<(usize, char)>,
    queue: VecDeque<Tok>,
    index: usize,
    line_number: usize,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    pub fn new(input: T) -> Self {
        let mut lexer = Lexer {
            chars: input,
            lookahead: None,
            queue: VecDeque::new(),
            index: 0,
            line_number: 1,
        };

        lexer.update_lookahead();
        lexer.index = 0;
        lexer
    }

    fn read_while<F>(&mut self, mut pred: F) -> String
    where
        F: FnMut(char) -> bool,
    {
        let mut value: String = String::new();

        while let Some((_i, c)) = self.lookahead {
            if pred(c) {
                value.push(c);
                self.update_lookahead();
            } else {
                break;
            }
        }

        value
    }

    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
        self.index += 1;
    }

    fn lex_source(&mut self) {
        let (_i, c) = self.lookahead.unwrap();

        if c == '\n' {
            self.queue.push_back(Tok::Newline);
        }

        self.update_lookahead();
    }

    fn emit(&self, token: Tok) -> Option<Spanned<Tok, usize, LexicalError>> {
        Some(Ok((
            self.line_number,
            token,
            self.line_number,
        )))
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If lookahead is None, we are at EOF
        if self.lookahead.is_none() {
            return None;
        }

        // If there are tokens in the queue, return the first one
        if !self.queue.is_empty() {
            let front: Tok = self.queue.pop_front().unwrap();
            return self.emit(front);
        }

        // Otherwise, do some lexing and return the next token
        self.lex_source();
        let front: Tok = self.queue.pop_front().unwrap();
        return self.emit(front);
    }
}
