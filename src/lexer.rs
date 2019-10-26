use std::str::CharIndices;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Copy, Clone, Debug)]
pub enum Tok<'input> {
    Identifier { name: &'input str },
    Assign,
    Integer { value: u32 },
    SemiColon,
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

    fn read_identifier(&mut self) -> &'input str {
        let start: usize = self.index - 1;

        while let Some((_i, c)) = self.lookahead {
            if c.is_alphabetic() {
                self.update_lookahead();
            } else {
                break;
            }
        }

        &self.input[start..self.index - 1]
    }

    fn read_integer(&mut self) -> &'input str {
        let start: usize = self.index - 1;

        while let Some((_i, c)) = self.lookahead {
            if c.is_digit(10) {
                self.update_lookahead();
            } else {
                break;
            }
        }

        &self.input[start..self.index - 1]
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

        while let Some(c) = self.lookahead {
            if c.1.is_alphabetic() {
                let ident = self.read_identifier();
                return Some(Ok((0, Tok::Identifier { name: ident }, 0)));
            } else if c.1 == '=' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Assign, 0)));
            } else if c.1 == ';' {
                self.update_lookahead();
                return Some(Ok((0, Tok::SemiColon, 0)));
            } else if c.1.is_digit(10) {
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
