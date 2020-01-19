use std::collections::VecDeque;

pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    Indent,
    Unindent,
    Identifier { name: String },
    Pass,
    If,
    Else,
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

enum IndentationChar {
    Unknown,
    Space,
    Tab,
}

struct Indentation {
    length: Vec<usize>,
    level: usize,
    character: IndentationChar,
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
                character: IndentationChar::Unknown,
            },
        };

        lexer.update_lookahead();
        lexer.index = 0;
        lexer
    }

    /// Generic read function that reads from the source while a predicate `pred` holds.
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

    /// Reads a string of characters from the source.
    /// Determines whether it is a keyword and adds to the queue accordingly.
    fn read_identifier_or_keyword(&mut self) {
        let ident: String = self.read_while(|c| c.is_alphabetic() || c == '_');

        self.queue.push_back(match ident.as_ref() {
            "pass" => Tok::Pass,
            "if" => Tok::If,
            "else" => Tok::Else,
            "while" => Tok::While,
            "def" => Tok::Def,
            "return" => Tok::Return,
            "or" => Tok::LogicalOr,
            "and" => Tok::LogicalAnd,
            "not" => Tok::LogicalNot,
            _ => Tok::Identifier { name: ident },
        });
    }

    /// Reads a number specified in Base10 from the source.
    fn read_number(&mut self) {
        let number: u32 = self.read_while(|c| c.is_digit(10)).parse().unwrap();
        self.queue.push_back(Tok::Integer { value: number });
    }

    /// Reads 'punctuation' from the source.
    /// Starts with single characters and moves onto multichars.
    fn read_punctuation(&mut self) {
        // We can immediately match the single character operators
        let single: Option<Tok> = match self.lookahead.map(|x| x.1) {
            Some(c) => {
                match c {
                    '(' => Some(Tok::LPar),
                    ')' => Some(Tok::RPar),
                    ':' => Some(Tok::Colon),
                    ';' => Some(Tok::Semicolon),
                    ',' => Some(Tok::Comma),
                    _ => None,
                }
            }
            None => None
        };

        if let Some(tok) = single {
            self.queue.push_back(tok);
            self.update_lookahead();
            return;
        }

        // Now deal with multi-char operators
        self.read_multichar();
    }

    /// Attempts to read a multichar token from the source.
    /// Sets up the multichars and calls `match_multichar_op` for each.
    /// Deals with the single case of `!=` where `!` is not a valid token.
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
        if self.current_char_equals('!') {
            self.update_lookahead();

            if self.current_char_equals('=') {
                self.update_lookahead();
                self.queue.push_back(Tok::NotEqual);
            }
        }
    }

    /// Generically matches any token that could be either 1 or 2 characters.
    ///
    /// For example, this can be used to match + and += by specifying multichar = (+, =, Plus,
    /// PlusEqual). Allows for much easier matching.
    fn match_multichar_op(&mut self, multichar: (char, char, Tok, Tok)) {
        // Unpack the arguments
        let (c1, c2, t1, t2): (char, char, Tok, Tok) = multichar;

        if self.current_char_equals(c1) {
            self.update_lookahead();

            if self.current_char_equals(c2) {
                self.update_lookahead();
                self.queue.push_back(t2);
            } else {
                self.queue.push_back(t1);
            }
        }
    }

    /// Reads a newline character from the source and updates internal variables.
    fn read_newline(&mut self) {
        self.queue.push_back(Tok::Newline);
        self.update_lookahead();

        self.start_of_line = true;
        self.line_number += 1;
    }

    /// Reads the indentation for a new line and deals with previous indentation levels.
    /// Updates the queue with new indents or unindents if needed.
    fn read_indentation(&mut self) {
        let indents: usize = match self.indentation.character {
            IndentationChar::Unknown => {
                if let Some(c) = self.lookahead.map(|x| x.1) {
                    match c {
                        ' ' => {
                            self.indentation.character = IndentationChar::Space;
                            self.read_while(|c| c == ' ').len()
                        }
                        '\t' => {
                            self.indentation.character = IndentationChar::Tab;
                            self.read_while(|c| c == '\t').len()
                        }
                        _ => 0,
                    }
                } else {
                    0
                }
            }
            IndentationChar::Space => self.read_while(|c| c == ' ').len(),
            IndentationChar::Tab => self.read_while(|c| c == '\t').len(),
        };

        let current: usize = *self.indentation.length.last().unwrap();

        if self.current_char_equals('\n') {
            self.read_newline();
            self.read_indentation();
            return;
        }

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

    /// Updates the lexer lookahead.
    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
        self.index += 1;
    }

    fn current_char_equals(&self, c: char) -> bool {
        return if let Some(l) = self.lookahead.map(|x| x.1) {
            c == l
        } else {
            false
        }
    }

    fn lex_source(&mut self) {
        if self.start_of_line {
            self.read_indentation();
            self.start_of_line = false;
        }

        self.read_while(|c| c == ' ');

        if let Some(c) = self.lookahead.map(|x| x.1) {
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
    }

    /// Formats a token for LALRPOP without the need to write this ugly syntax everywhere.
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
        match self.lookahead {
            Some(_) => {
                self.lex_source();
            }
            None => {
                if let Some(t) = self.queue.pop_front() {
                    return self.emit(t);
                }

                self.read_indentation();
            }
        }

        match self.queue.pop_front() {
            Some(t) => self.emit(t),
            None => None
        }
    }
}
