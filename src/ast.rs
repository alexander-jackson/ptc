#[derive(Debug)]
pub enum Type {
    Int,
}

impl Type {
    fn to_string(&self) -> &str {
        use ast::Type::*;

        match *self {
            Int => "int",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Stmt<'input> {
    Statement(Identifier<'input>, Operator, Expression),
}

impl<'input> Stmt<'input> {
    fn generate(&self) {
        println!("Generating code: {:?}", self);
        match &*self {
            Stmt::Statement(_, _, _) => generate_assignment(&*self),
        };
    }
}

#[derive(Debug, PartialEq)]
pub enum Identifier<'input> {
    Name { name: &'input str },
}

impl<'input> Identifier<'input> {
    fn to_string(&self) -> &'input str {
        match *self {
            Identifier::Name { name } => name
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Operator {
    Assign,
    Plus,
    Minus,
    Multiply,
    Divide,
}

impl Operator {
    fn to_string(&self) -> &str {
        use ast::Operator::*;

        match *self {
            Assign => "=",
            Plus => "+",
            Minus => "-",
            Multiply => "*",
            Divide => "/",
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expression {
    BinaryOperation {
        left: Box<Expression>,
        op: Operator,
        right: Box<Number>,
    },
    ParenExpr { expr: Box<Expression> },
    Literal { value: Number },
}

impl Expression {
    fn to_string(&self) -> String {
        use ast::Expression::*;

        match &*self {
            BinaryOperation { left, op, right } =>
                left.to_string() + op.to_string() + &right.to_string(),
            ParenExpr { expr } => format!("({})", expr.to_string()),
            Literal { value } => value.to_string(),
        }
    }

    fn get_type(&self) -> Type {
        Type::Int
    }
}

#[derive(Debug, PartialEq)]
pub enum Number {
    Integer { value: u32 },
}

impl Number {
    fn to_string(&self) -> String {
        match *self {
            Number::Integer { value } => value.to_string(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Program<'input> {
    pub stmts: Vec<Stmt<'input>>,
}

impl<'input> Program<'input> {
    pub fn new(stmts: Vec<Stmt<'input>>) -> Self {
        Program { stmts }
    }

    pub fn dump(&self) {
        for ident in self.stmts.iter() {
            println!("{:?}", ident);
        }
    }

    pub fn generate(&self) {
        for stmt in &self.stmts {
            stmt.generate();
        }
    }
}

pub fn number(value: u32) -> Number {
    Number::Integer { value }
}

pub fn identifier<'input>(name: &'input str) -> Identifier<'input> {
    Identifier::Name { name: name }
}

pub fn generate_assignment(stmt: &Stmt) {
    let (ident, op, value) = match stmt {
        Stmt::Statement(ident, op, value) => (ident, op, value),
    };

    let variable_type: Type = value.get_type();

    println!(
        "{} {} {} {}",
        variable_type.to_string(),
        ident.to_string(),
        op.to_string(),
        value.to_string()
    );
}
