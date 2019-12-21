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

struct Indentation {
    length: Vec<usize>,
    level: usize,
}

pub struct Lexer<T: Iterator<Item = (usize, char)>> {
    chars: T,
    lookahead: Option<(usize, char)>,
    queue: VecDeque<Tok>,
    index: usize,
    line_number: usize,
    start_of_line: bool,
    indentation: Indentation,
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
            start_of_line: true,
            indentation: Indentation {
                length: vec![0],
                level: 0,
            },
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

    fn read_identifier_or_keyword(&mut self) {
        let ident: String = self.read_while(|c| c.is_alphabetic() || c == '_');

        self.queue.push_back(match ident.as_ref() {
            "pass" => Tok::Pass,
            "if" => Tok::If,
            "while" => Tok::While,
            "def" => Tok::Def,
            "return" => Tok::Return,
            "or" => Tok::LogicalOr,
            "and" => Tok::LogicalAnd,
            "not" => Tok::LogicalNot,
            _ => Tok::Identifier { name: ident },
        });
    }

    fn read_number(&mut self) {
        let number: u32 = self.read_while(|c| c.is_digit(10)).parse().unwrap();
        self.queue.push_back(Tok::Integer { value: number });
    }

    fn read_punctuation(&mut self) {
        // We can immediately match the single character operators
        let c: char = self.lookahead.unwrap().1;

        let single: Option<Tok> = match c {
            '(' => Some(Tok::LPar),
            ')' => Some(Tok::RPar),
            ':' => Some(Tok::Colon),
            ';' => Some(Tok::Semicolon),
            ',' => Some(Tok::Comma),
            _ => None,
        };

        if single.is_some() {
            self.queue.push_back(single.unwrap());
            self.update_lookahead();
            return;
        }

        // Now deal with multi-char operators
        self.read_multichar();
    }

    fn read_multichar(&mut self) {
        let multichars: Vec<(char, char, Tok, Tok)> = vec![
            ('=', '=', Tok::Assign, Tok::Equal),
            ('+', '=', Tok::Plus, Tok::PlusEquals),
            ('-', '=', Tok::Minus, Tok::MinusEquals),
            ('*', '=', Tok::Multiply, Tok::MultiplyEquals),
            ('/', '=', Tok::Divide, Tok::DivideEquals),
            ('<', '=', Tok::Less, Tok::LessOrEqual),
            ('>', '=', Tok::Greater, Tok::GreaterOrEqual),
        ];

        for m in multichars {
            self.match_multichar_op(m);
        }

        // Deal with the only multichar op that isn't splitable
        let mut c: char = self.lookahead.unwrap().1;

        if c == '!' {
            self.update_lookahead();
            c = self.lookahead.unwrap().1;

            if c == '=' {
                self.update_lookahead();
                self.queue.push_back(Tok::NotEqual);
            }
        }
    }

    fn match_multichar_op(&mut self, multichar: (char, char, Tok, Tok)) {
        // Unpack the arguments
        let (c1, c2, t1, t2): (char, char, Tok, Tok) = multichar;
        // Get the current character
        let mut c: char = self.lookahead.unwrap().1;

        if c == c1 {
            // Either t1 or t2 now
            self.update_lookahead();
            c = self.lookahead.unwrap().1;

            if c == c2 {
                self.update_lookahead();
                self.queue.push_back(t2);
            } else {
                self.queue.push_back(t1);
            }
        }
    }

    fn read_newline(&mut self) {
        self.queue.push_back(Tok::Newline);
        self.update_lookahead();

        self.start_of_line = true;
        self.line_number += 1;
    }

    fn read_indentation(&mut self) {
        let indents: usize = self.read_while(|c| c == ' ').len();
        let current: usize = *self.indentation.length.last().unwrap();

        if indents != current {
            // Indentation level has changed
            if current < indents {
                // Push the new indentation level and update
                self.indentation.length.push(indents);
                self.indentation.level += 1;
                self.queue.push_back(Tok::Indent);
            } else {
                // See how far back we have gone
                let mut len: usize = self.indentation.length.pop().unwrap();

                while len > indents {
                    self.indentation.level -= 1;
                    self.queue.push_back(Tok::Unindent);
                    len = self.indentation.length.pop().unwrap();
                }

                self.indentation.length.push(indents);
            }
        }
    }

    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
        self.index += 1;
    }

    fn lex_source(&mut self) {
        if self.start_of_line {
            self.read_indentation();
            self.start_of_line = false;
        }

        let mut c: char = self.lookahead.unwrap().1;

        while c == ' ' {
            self.update_lookahead();

            if self.lookahead.is_none() {
                return;
            }

            c = self.lookahead.unwrap().1;
        }

        // If c is a character, read an identifier
        if c.is_alphabetic() || c == '_' {
            self.read_identifier_or_keyword();
        } else if c.is_digit(10) {
            self.read_number();
        } else if c == '\n' {
            self.read_newline();
        } else {
            self.read_punctuation();
        }
    }

    fn emit(&self, token: Tok) -> Option<Spanned<Tok, usize, LexicalError>> {
        Some(Ok((self.line_number, token, self.line_number)))
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
            self.read_indentation();

            if !self.queue.is_empty() {
                let front: Tok = self.queue.pop_front().unwrap();
                return self.emit(front);
            }

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
