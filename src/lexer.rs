//! A lexer for Python program inputs.
//!
//! Deals with the tokenisation of the input source file. Implements the maximal munch approach,
//! where we should always read as much of the input as possible at each time. `Tok` contains all
//! of the valid tokens for `ptc` and the parser will expect these as output.

use std::collections::VecDeque;
use std::error::Error;
use std::fmt;

/// Represents the output type for the lexer.
///
/// The lexer can output either a token with some positional information about where in the program
/// we are or an error message.
pub type Spanned<Tok, Loc, Error> = Result<(Loc, Tok, Loc), Error>;

/// A token that can be output by the lexer and understood by the parser.
#[derive(Clone, Debug, PartialEq)]
pub enum Tok {
    /// An indentation token
    Indent,
    /// An unindent token
    Unindent,
    /// An identifier token
    Identifier {
        /// The name of the identifier
        name: String,
    },
    /// The `pass` keyword
    Pass,
    /// The `if` keyword
    If,
    /// The `else` keyword
    Else,
    /// The `while` keyword
    While,
    /// The `def` keyword
    Def,
    /// The `return` keyword
    Return,
    /// The `global` keyword
    Global,
    /// The `del` keyword
    Del,

    // Operators
    /// The `=` operator
    Assign,
    /// The `+` operator
    Plus,
    /// The `-` operator
    Minus,
    /// The `*` operator
    Multiply,
    /// The `/` operator
    Divide,
    /// The `%` operator
    Modulo,
    /// The `+=` operator
    PlusEquals,
    /// The `-=` operator
    MinusEquals,
    /// The `*=` operator
    MultiplyEquals,
    /// The `/=` or `//=` operator
    DivideEquals,
    /// The `%=` operator
    ModuloEquals,
    /// The `or` operator
    LogicalOr,
    /// The `and` operator
    LogicalAnd,
    /// The `not` operator
    LogicalNot,
    /// The `<` operator
    Less,
    /// The `>` operator
    Greater,
    /// The `<=` operator
    LessOrEqual,
    /// The `>=` operator
    GreaterOrEqual,
    /// The `==` operator
    Equal,
    /// The `!=` operator
    NotEqual,
    /// An integer token
    Integer {
        /// The value of the integer
        value: u32,
    },
    /// A float token
    Float {
        /// The value of the float
        value: f32,
    },
    /// The `(` token
    LPar,
    /// The `)` token
    RPar,
    /// The `[` token
    LSquare,
    /// The `]` token
    RSquare,
    /// The `:` token
    Colon,
    /// The `;` token
    Semicolon,
    /// The `,` token
    Comma,
    /// The `.` token
    Dot,
    /// The `->` token
    Arrow,
    /// The `\n` token
    Newline,
}

/// Represents any type of error that the lexer can encounter.
#[derive(Debug)]
pub enum LexicalError {
    /// The input file contains both tabs and spaces as indentation.
    MixedIndentation(usize),
    /// The input file contains malformed indentation.
    ///
    /// This occurs when a line does not contain the same amount of indentation as something
    /// previously in the stack.
    InconsistentIndentation(usize),
}

impl Error for LexicalError {}

impl fmt::Display for LexicalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            LexicalError::MixedIndentation(line) => write!(
                f,
                "Encountered mixed indentation on line {}. Please use either tabs OR spaces.",
                line,
            ),
            LexicalError::InconsistentIndentation(line) => write!(
                f,
                "Encountered malformed indentation on line {}. Please ensures statements are always correctly aligned.",
                line
            ),
        }
    }
}

/// The characters that can be used for indentation.
#[derive(Debug, PartialEq)]
enum IndentationChar {
    /// Space indented
    Space,
    /// Tab indented
    Tab,
}

/// Tracks indentation related information for the program.
///
/// Tracks the stack of indentation sizes that have been used currently, the current level of
/// indentation and the indentation character being used if one exists.
struct Indentation {
    /// The stack of indentation we have read so far
    length: Vec<usize>,
    /// The current level of indentation, distinct from length, this tracks the number of times we
    /// have indented regardless of character used. For example, reading indentation with 4 spaces
    /// and then indentation with 2 spaces would result in (length = 6, level = 2)
    level: usize,
    /// The indentation character used for this file (if known)
    character: Option<IndentationChar>,
}

impl Indentation {
    /// Initialises the indentation at the start of the program.
    pub fn new() -> Indentation {
        Indentation {
            length: vec![0],
            level: 0,
            character: None,
        }
    }

    /// Gets the top of the stack of indentation
    pub fn get_current_length(&self) -> usize {
        self.length[self.length.len() - 1]
    }

    /// Pops a level off the stack and returns it
    pub fn pop_length(&mut self) -> usize {
        self.length.pop().unwrap()
    }
}

/// Stores the state of the lexer.
///
/// The lexer iterates over characters and their positions in the program. It returns either tokens
/// from `Tok` or errors from `LexicalError`. It is used to turn the input program into tokens that
/// the parser will understand.
pub struct Lexer<T: Iterator<Item = (usize, char)>> {
    /// The input stream
    chars: T,
    /// The current lookahead token if it exists
    lookahead: Option<(usize, char)>,
    /// The queue of tokens/errors to be output
    queue: VecDeque<Result<Tok, LexicalError>>,
    /// The current line number we are on
    line_number: usize,
    /// Whether we have just read a newline token
    start_of_line: bool,
    /// The current state of the program indentation
    indentation: Indentation,
}

impl<T> Lexer<T>
where
    T: Iterator<Item = (usize, char)>,
{
    /// Creates a new instance of the Lexer given an iterable stream of characters.
    ///
    /// # Example
    ///
    /// ```
    /// use ptc::lexer::Lexer;
    ///
    /// let input = "x = 1";
    /// let lexer = Lexer::new(input.char_indices());
    /// ```
    pub fn new(input: T) -> Self {
        let mut lexer = Lexer {
            chars: input,
            lookahead: None,
            queue: VecDeque::new(),
            line_number: 1,
            start_of_line: true,
            indentation: Indentation::new(),
        };

        lexer.update_lookahead();
        lexer
    }

    /// Generic read function that reads from the source while a predicate `pred` holds.
    ///
    /// Simply reads from the source file while the predicate `pred` returns true. Returns the
    /// string it has read. Can be used to read numbers: `read_while(|c| c.is_digit(10))` or
    /// up until the next newline: `read_while(|c| c != '\n')`.
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

    /// Reads a string of characters from the source that may be an identifier or a keyword.
    ///
    /// Read the next string of characters from the source, assuming that it has found an
    /// identifier. Performs a match on the result, checking whether it has found a keyword, and
    /// pushes the relevant token to the queue.
    fn read_identifier_or_keyword(&mut self) {
        let ident: String = self.read_while(|c| c.is_alphabetic() || c == '_' || c.is_numeric());

        self.push_token(match ident.as_ref() {
            "pass" => Tok::Pass,
            "if" => Tok::If,
            "else" => Tok::Else,
            "while" => Tok::While,
            "def" => Tok::Def,
            "return" => Tok::Return,
            "global" => Tok::Global,
            "del" => Tok::Del,
            "or" => Tok::LogicalOr,
            "and" => Tok::LogicalAnd,
            "not" => Tok::LogicalNot,
            _ => Tok::Identifier { name: ident },
        });
    }

    /// Reads a number specified in Base10 from the source.
    ///
    /// Initially reads while there are digits in the source. If it finishes on a '.' character, it
    /// will read that, then any more numbers, assuming it is a float now. Otherwise, it will
    /// assume it is an integer.
    fn read_number(&mut self) {
        // TODO: This crashes a lot, if <number> is too large
        //
        // Solution could be to read a string and try parsing it. If it is too large,
        // truncate it or just consider it to be the max value of a u32. For example,
        // given 3x10^1000, either turn this into 2^31 - 1 or truncate until it's lower
        // than that //
        let mut number: String = self.read_while(|c| c.is_digit(10));

        // If we end on a '.', try and read a float
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
    ///
    /// Initially tries to read single character tokens from the source. If it finds one, it will
    /// push it to the queue and then return. Otherwise, it will try and read multichar tokens.
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
                '.' => Some(Tok::Dot),
                _ => None,
            },
            None => None,
        };

        // If we matched something before, push it and return
        if let Some(tok) = single {
            self.push_token(tok);
            self.update_lookahead();
            return;
        }

        // Now deal with multi-char operators
        self.read_multichar();
    }

    /// Attempts to read a multichar token from the source.
    ///
    /// Multichars are typically things like '=' and '==', where we want to perform maximal munch
    /// and read as much as possible, even if a substring would be a valid token. It also deals
    /// with some longer tokens, such as '//=' being able to be split into 3 tokens.
    fn read_multichar(&mut self) {
        let multichars: Vec<(char, char, Tok, Tok)> = vec![
            ('=', '=', Tok::Assign, Tok::Equal),
            ('+', '=', Tok::Plus, Tok::PlusEquals),
            ('*', '=', Tok::Multiply, Tok::MultiplyEquals),
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

        // Deal with the case of '-', '-=' and '->'
        if self.current_char_equals('-') {
            self.update_lookahead();

            if self.current_char_equals('=') {
                self.update_lookahead();
                self.push_token(Tok::MinusEquals);
            } else if self.current_char_equals('>') {
                self.update_lookahead();
                self.push_token(Tok::Arrow);
            } else {
                self.push_token(Tok::Minus);
            }
        }

        // Deal with the monstrosity that is '/', '/=', '//' and '//='
        if self.current_char_equals('/') {
            self.update_lookahead();

            if self.current_char_equals('/') {
                self.update_lookahead();

                if self.current_char_equals('=') {
                    self.update_lookahead();
                    self.push_token(Tok::DivideEquals);
                } else {
                    self.push_token(Tok::Divide);
                }
            } else if self.current_char_equals('=') {
                self.update_lookahead();
                self.push_token(Tok::DivideEquals)
            } else {
                self.push_token(Tok::Divide);
            }
        }
    }

    /// Generically matches any token that could be either 1 or 2 characters.
    ///
    /// For example, this can be used to match `+` and `+=` by specifying multichar as `('+', '=', Plus,
    /// PlusEqual)`. Allows for much easier matching.
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

    /// Reads a newline character from the source.
    ///
    /// Reads a newline character from the source and updates the lookahead. Sets that we are now
    /// at the start of a line and increments the line number.
    fn read_newline(&mut self) {
        self.push_token(Tok::Newline);
        self.update_lookahead();

        self.start_of_line = true;
        self.line_number += 1;
    }

    /// Checks for mixed indentation in the source file.
    ///
    /// Checks whether the current lookahead character and `IndentationChar` are conflicting and
    /// thus mixed indentation has been used in the file.
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

    /// Reads the indentation size for the current line.
    ///
    /// Infers the indentation character if it is unknown, otherwise just performs a simple match
    /// statement.
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
            };
        }

        // We have no indentation character, so we must have not indented yet
        0
    }

    /// Reads the indentation for a new line and deals with previous indentation levels.
    ///
    /// Checks the current position for mixed indentation, then checks for a new indentation level
    /// or a return to a previous one.
    fn read_indentation(&mut self) {
        // Check for incorrect indentation characters
        if self.check_for_mixed_indentation() {
            self.push_error(LexicalError::MixedIndentation(self.line_number));
            return;
        }

        // Check for indentation type
        let indents: usize = self.read_indentation_size();
        let current: usize = self.indentation.get_current_length();

        // Check if this line is blank
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
                let mut level = self.indentation.pop_length();

                while level != indents {
                    self.indentation.level -= 1;
                    self.push_token(Tok::Unindent);
                    level = self.indentation.pop_length();

                    if level < indents {
                        self.push_error(LexicalError::InconsistentIndentation(self.line_number));
                        return;
                    }
                }

                self.indentation.length.push(indents);
            }
        }
    }

    /// Reads a comment from the source file.
    ///
    /// Assumes the current token is a hash and reads everything up until the next newline
    /// character, ignoring it and not outputting any tokens.
    fn read_comment(&mut self) {
        // Read the hash
        self.update_lookahead();
        // Read while we are not at a newline
        self.read_while(|c| c != '\n');
    }

    /// Updates the lexer lookahead.
    ///
    /// Moves the lexer position forwards one character and updates the lookahead character.
    fn update_lookahead(&mut self) {
        self.lookahead = self.chars.next();
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

    /// The entry point for lexing.
    ///
    /// Calls each of the lexing functions based on the current position in the source. Begins by
    /// reading indentation or whitespace, then comments, then any other token that might be in the
    /// source file.
    fn lex_source(&mut self) {
        if self.start_of_line {
            self.read_indentation();
            self.start_of_line = false;
        } else {
            self.read_while(|c| c == ' ');
        }

        if self.current_char_equals('#') {
            self.read_comment();
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
