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
    index: usize,
    prev_spaces: usize,
    curr_spaces: usize,
    indentation_level: usize,
    start_of_line: bool,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    pub fn new(input: T) -> Self {
        let mut lexer = Lexer {
            chars: input,
            lookahead: None,
            index: 0,
            prev_spaces: 0,
            curr_spaces: 0,
            indentation_level: 0,
            start_of_line: true,
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

    fn check_operator(&mut self) -> Option<Tok> {
        if let Some((_i, c)) = self.lookahead {
            return match c {
                '(' => Some(Tok::LPar),
                ')' => Some(Tok::RPar),
                ':' => Some(Tok::Colon),
                ';' => Some(Tok::Semicolon),
                ',' => Some(Tok::Comma),
                _ => None,
            };
        }

        None
    }

    fn check_multiple_character_operators(&mut self) -> Option<Tok> {
        if let Some((_i, c)) = self.lookahead {
            if c == '<' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::LessOrEqual)
                        },
                        _ => Some(Tok::Less),
                    };
                }
            }

            if c == '>' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::GreaterOrEqual)
                        },
                        _ => Some(Tok::Greater),
                    };
                }
            }

            if c == '!' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::NotEqual)
                        },
                        _ => None,
                    };
                }
            }

            if c == '=' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::Equal)
                        },
                        _ => Some(Tok::Assign),
                    };
                }
            }

            if c == '+' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::PlusEquals)
                        },
                        _ => Some(Tok::Plus),
                    };
                }
            }

            if c == '-' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::MinusEquals)
                        },
                        _ => Some(Tok::Minus),
                    };
                }
            }

            if c == '*' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::MultiplyEquals)
                        },
                        _ => Some(Tok::Multiply),
                    };
                }
            }

            if c == '/' {
                self.update_lookahead();

                if let Some((_i, c)) = self.lookahead {
                    return match c {
                        '=' => {
                            self.update_lookahead();
                            Some(Tok::DivideEquals)
                        },
                        _ => Some(Tok::Divide),
                    };
                }
            }
        }

        None
    }

    fn check_for_indentation(&mut self) -> Option<Tok> {
        if let Some((_i, c)) = self.lookahead {
            // Check for whitespace
            if c == ' ' {
                self.curr_spaces = self.read_while(|c| c == ' ').len();
                self.start_of_line = false;

                // If prev < curr, we indented
                if self.prev_spaces < self.curr_spaces {
                    self.indentation_level += 1;
                    return Some(Tok::Indent);
                }

                // If prev > curr, we unindented
                if self.prev_spaces > self.curr_spaces {
                    self.indentation_level -= 1;
                    return Some(Tok::Unindent);
                }
            } else {
                // No whitespace, so check if this is an unindent
                self.start_of_line = false;

                if self.prev_spaces > self.curr_spaces {
                    self.indentation_level -= 1;
                    return Some(Tok::Unindent);
                }
            }
        }

        None
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((_i, c)) = self.lookahead {
            if self.start_of_line {
                if let Some(tok) = self.check_for_indentation() {
                    return Some(Ok((0, tok, 0)));
                }
            }

            if c.is_alphabetic() {
                let ident = self.read_identifier();

                let token: Tok = match ident.as_ref() {
                    "pass" => Tok::Pass,
                    "or" => Tok::LogicalOr,
                    "and" => Tok::LogicalAnd,
                    "not" => Tok::LogicalNot,
                    "if" => Tok::If,
                    "while" => Tok::While,
                    "def" => Tok::Def,
                    "return" => Tok::Return,
                    _ => Tok::Identifier { name: ident },
                };

                return Some(Ok((0, token, 0)));
            } else if c == '\n' {
                self.prev_spaces = self.curr_spaces;
                self.curr_spaces = 0;
                self.start_of_line = true;
                self.update_lookahead();
                return Some(Ok((0, Tok::Newline, 0)));
            } else if c.is_digit(10) {
                let value: u32 = self.read_integer().parse().unwrap();
                return Some(Ok((0, Tok::Integer { value: value }, 0)));
            } else {
                // Check for an operator
                let op = self.check_operator();

                if op.is_some() {
                    self.update_lookahead();
                    return Some(Ok((0, op.unwrap(), 0)));
                }

                let op = self.check_multiple_character_operators();

                if op.is_some() {
                    return Some(Ok((0, op.unwrap(), 0)));
                }

                self.update_lookahead();
                continue;
            }
        }

        // Check whether we are currently indented
        if self.indentation_level > 0 {
            self.indentation_level -= 1;
            return Some(Ok((0, Tok::Unindent, 0)));
        }

        return None;
    }
}
