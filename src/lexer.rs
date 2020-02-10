use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

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
    Modulo,
    PlusEquals,
    MinusEquals,
    MultiplyEquals,
    DivideEquals,
    ModuloEquals,
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
    Float { value: f32 },
    LPar,
    RPar,
    LSquare,
    RSquare,
    Colon,
    Semicolon,
    Comma,
    Newline,
}

pub enum LexicalError {
    MixedIndentation,
}

impl Error for LexicalError {}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::MixedIndentation => write!(
                f,
                "Encountered mixed indentation in the file. Please use either tabs OR spaces."
            ),
        }
    }
}

impl fmt::Debug for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::MixedIndentation => {
                write!(f, "Encountered mixed indentation in the file. Please use either tabs or spaces exclusively.")
            }
        }
    }
}

#[derive(Debug, PartialEq)]
enum IndentationChar {
    Space,
    Tab,
}

struct Indentation {
    length: Vec<usize>,
    level: usize,
    character: Option<IndentationChar>,
}

impl Indentation {
    pub fn new() -> Indentation {
        Indentation {
            length: vec![0],
            level: 0,
            character: None,
        }
    }

    pub fn get_current_length(&self) -> usize {
        self.length[self.length.len() - 1]
    }

    pub fn pop_length(&mut self) -> usize {
        self.length.pop().unwrap()
    }
}

pub struct Lexer<T: Iterator<Item = (usize, char)>> {
    chars: T,
    lookahead: Option<(usize, char)>,
    queue: VecDeque<Result<Tok, LexicalError>>,
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
            indentation: Indentation::new(),
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

        self.push_token(match ident.as_ref() {
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
        // TODO: This crashes a lot, if <number> is too large
        //
        // Solution could be to read a string and try parsing it. If it is too large,
        // truncate it or just consider it to be the max value of a u32. For example,
        // given 3x10^1000, either turn this into 2^31 - 1 or truncate until it's lower
        // than that //
        let mut number: String = self.read_while(|c| c.is_digit(10));

        if self.current_char_equals('.') {
            number.push('.');
            self.update_lookahead();
            number.push_str(&self.read_while(|c| c.is_digit(10)));
            let value: f32 = number.parse().unwrap_or_else(|_| std::f32::MAX);
            return self.push_token(Tok::Float { value });
        }

        let value: u32 = number.parse().unwrap_or_else(|_| u32::max_value());
        self.push_token(Tok::Integer { value });
    }

    /// Reads 'punctuation' from the source.
    /// Starts with single characters and moves onto multichars.
    fn read_punctuation(&mut self) {
        // We can immediately match the single character operators
        let single: Option<Tok> = match self.lookahead.map(|x| x.1) {
            Some(c) => match c {
                '(' => Some(Tok::LPar),
                ')' => Some(Tok::RPar),
                '[' => Some(Tok::LSquare),
                ']' => Some(Tok::RSquare),
                ':' => Some(Tok::Colon),
                ';' => Some(Tok::Semicolon),
                ',' => Some(Tok::Comma),
                _ => None,
            },
            None => None,
        };

        if let Some(tok) = single {
            self.push_token(tok);
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
            ('%', '=', Tok::Modulo, Tok::ModuloEquals),
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
                self.push_token(Tok::NotEqual);
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
                self.push_token(t2);
            } else {
                self.push_token(t1);
            }
        }
    }

    /// Reads a newline character from the source and updates internal variables.
    fn read_newline(&mut self) {
        self.push_token(Tok::Newline);
        self.update_lookahead();

        self.start_of_line = true;
        self.line_number += 1;
    }

    /// Checks whether the current lookahead character and IndentationChar are conflicting and thus
    /// mixed indentation has been used in the file.
    fn check_for_mixed_indentation(&self) -> bool {
        if let Some(i) = &self.indentation.character {
            if i == &IndentationChar::Space && self.current_char_equals('\t') {
                return true;
            }

            if i == &IndentationChar::Tab && self.current_char_equals(' ') {
                return true;
            }
        }

        false
    }

    /// Reads the indentation size for the current line. Infers the indentation character if it is
    /// unknown, otherwise just performs a simple match statement.
    fn read_indentation_size(&mut self) -> usize {
        if self.indentation.character.is_none() {
            if self.current_char_equals(' ') {
                self.indentation.character = Some(IndentationChar::Space);
            }

            if self.current_char_equals('\t') {
                self.indentation.character = Some(IndentationChar::Tab);
            }
        }

        if let Some(i) = &self.indentation.character {
            return match i {
                IndentationChar::Space => self.read_while(|c| c == ' ').len(),
                IndentationChar::Tab => self.read_while(|c| c == '\t').len(),
            }
        }

        0
    }

    /// Reads the indentation for a new line and deals with previous indentation levels.
    /// Updates the queue with new indents or unindents if needed.
    fn read_indentation(&mut self) {
        // Check for incorrect indentation characters
        if self.check_for_mixed_indentation() {
            self.push_error(LexicalError::MixedIndentation);
            return;
        }

        // Check for indentation type
        let indents: usize = self.read_indentation_size();
        let current: usize = self.indentation.get_current_length();

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
                self.push_token(Tok::Indent);
            } else {
                // See how far back we have gone
                while self.indentation.pop_length() > indents {
                    self.indentation.level -= 1;
                    self.push_token(Tok::Unindent);
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

    /// Checks whether the current lookahead character is equal to the argument. If lookahead is
    /// not none, it will do the comparison and return the result, otherwise it will obviously
    /// return false.
    fn current_char_equals(&self, c: char) -> bool {
        if let Some(l) = self.lookahead.map(|x| x.1) {
            return c == l;
        }

        false
    }

    fn lex_source(&mut self) {
        if self.start_of_line {
            self.read_indentation();
            self.start_of_line = false;
        } else {
            self.read_while(|c| c == ' ');
        }

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

    /// Pushes a token onto the queue.
    fn push_token(&mut self, token: Tok) {
        self.queue.push_back(Ok(token));
    }

    /// Pushes an error onto the queue.
    fn push_error(&mut self, err: LexicalError) {
        self.queue.push_back(Err(err));
    }

    /// Formats a token for LALRPOP without the need to write this ugly syntax everywhere.
    fn emit(&self, token: Tok) -> Option<Spanned<Tok, usize, LexicalError>> {
        Some(Ok((self.line_number, token, self.line_number)))
    }

    /// Formats an error for LALRPOP without the need to write this ugly syntax everywhere.
    fn emit_error(&self, err: LexicalError) -> Option<Spanned<Tok, usize, LexicalError>> {
        Some(Err(err))
    }
}

impl<T> Iterator for Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    type Item = Spanned<Tok, usize, LexicalError>;

    fn next(&mut self) -> Option<Self::Item> {
        // If there is something on the queue, return it
        if let Some(q) = self.queue.pop_front() {
            return match q {
                Ok(t) => self.emit(t),
                Err(e) => self.emit_error(e),
            };
        }

        // Otherwise, see if we need to read from the stream
        match self.lookahead {
            Some(_) => self.lex_source(),
            None => self.read_indentation(),
        }

        // There should be at least something in the queue now, so return that
        // If there isn't, we must have finished lexing the input
        match self.queue.pop_front() {
            Some(Ok(t)) => self.emit(t),
            Some(Err(e)) => self.emit_error(e),
            None => None,
        }
    }
}
