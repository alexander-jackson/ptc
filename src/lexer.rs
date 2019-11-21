pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, Debug)]
pub enum Tok {
    Identifier { name: String },
    Pass,

    // Operators
    Equals,
    Plus,
    Minus,
    Multiply,
    Divide,
    LogicalOr,
    LogicalAnd,

    Integer { value: u32 },
    LPar,
    RPar,
    Newline,
}

#[derive(Debug)]
pub enum LexicalError {
    // Not possible
}

pub struct Lexer<T: Iterator<Item = (usize, char)>> {
    chars: T,
    lookahead: Option<(usize, char)>,
    index: usize,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    pub fn new(input: T) -> Self {
        Lexer {
            chars: input,
            lookahead: None,
            index: 0,
        }
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

    fn read_identifier(&mut self) -> String {
        self.read_while(|c| c.is_alphabetic() || c == '_')
    }

    fn read_integer(&mut self) -> String {
        self.read_while(|c| c.is_digit(10))
    }

    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
        self.index += 1;
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index == 0 {
            self.update_lookahead();
        }

        while let Some((_i, c)) = self.lookahead {
            if c.is_alphabetic() {
                let ident = self.read_identifier();

                let token: Tok = match ident.as_ref() {
                    "pass" => Tok::Pass,
                    "or" => Tok::LogicalOr,
                    "and" => Tok::LogicalAnd,
                    _ => Tok::Identifier { name: ident },
                };

                return Some(Ok((0, token, 0)));
            } else if c == '=' {
                self.update_lookahead();
                return Some(Ok((0, Tok::Equals, 0)));
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
