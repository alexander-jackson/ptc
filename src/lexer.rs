use std::str::CharIndices;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    Identifier { name: &'input str },

    // Operators
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,

    Integer { value: u32 },
    LPar,
    RPar,
    Newline,
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

pub struct Lexer<'input> {
    input: &'input str,
    chars: CharIndices<'input>,
    lookahead: Option<(usize, char)>,
    index: usize,
}

impl<'input> Lexer<'input> {
    pub fn new(input: &'input str) -> Self {
        Lexer {
            input: input,
            chars: input.char_indices(),
            lookahead: None,
            index: 0,
        }
    }

    fn read_while<F>(&mut self, mut pred: F) -> &'input str
    where F: FnMut(char) -> bool,
    {
        let start: usize = self.index - 1;

        while let Some((_i, c)) = self.lookahead {
            if pred(c) {
                self.update_lookahead();
            } else {
                break;
            }
        }

        &self.input[start..self.index - 1]
    }

    fn read_identifier(&mut self) -> &'input str {
        self.read_while(|c| c.is_alphabetic() || c == '_')
    }

    fn read_integer(&mut self) -> &'input str {
        self.read_while(|c| c.is_digit(10))
    }

    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
        self.index += 1;
    }
}

impl<'input> Iterator for Lexer<'input> {
    type Item = Spanned<Tok<'input>, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.update_lookahead();
        }

        while let Some((_i, c)) = self.lookahead {
            if c.is_alphabetic() {
                let ident = self.read_identifier();
                return Some(Ok((0, Tok::Identifier { name: ident }, 0)));
            } else if c == '=' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Assign, 0)));
            } else if c == '+' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Plus, 0)));
            } else if c == '-' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Minus, 0)));
            } else if c == '*' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Multiply, 0)));
            } else if c == '/' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Divide, 0)));
            } else if c == '(' {
                self.update_lookahead();
                return Some(Ok((0, Tok::LPar, 0)));
            } else if c == ')' {
                self.update_lookahead();
                return Some(Ok((0, Tok::RPar, 0)));
            } else if c == '\n' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Newline, 0)));
            } else if c.is_digit(10) {
                let value: u32 = self.read_integer().parse().unwrap();
                return Some(Ok((0, Tok::Integer { value: value }, 0)));
            } else {
                self.update_lookahead();
                continue;
            }
        }

        return None;
    }
}
