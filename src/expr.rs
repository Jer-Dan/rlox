use crate::token::Token;

static INDENT: usize = 4;

#[derive(Debug)]
pub enum Expression<'a> {
    Binary(Binary<'a>),
    Grouping(Grouping<'a>),
    Literal(Literal<'a>),
    Unary(Unary<'a>),
}

impl<'a> Expression<'a> {
    pub fn print(&self, indent: usize) {
        match self {
            Self::Binary(b) => b.print(indent),
            Self::Grouping(g) => g.print(indent),
            Self::Literal(l) => l.print(indent),
            Self::Unary(u) => u.print(indent),
        }
    }
}

pub trait Expr<'a> {
    fn print(&self, indent: usize);
}

#[derive(Debug)]
pub struct Binary<'a> {
    left: Box<Expression<'a>>,
    operator: Token<'a>,
    right: Box<Expression<'a>>,
}

impl<'a> Binary<'a> {
    pub fn new(left: Box<Expression<'a>>, operator: Token<'a>, right: Box<Expression<'a>>) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl<'a> Expr<'a> for Binary<'a> {
    fn print(&self, indent: usize) {
        println!(
            "{}Binary:\n{}Operator:\n{}{:?}",
            " ".repeat(indent),
            " ".repeat(indent + INDENT),
            " ".repeat(indent + INDENT + INDENT),
            self.operator.token_type
        );
        println!("{}Left: ", " ".repeat(indent + INDENT));
        self.left.print(indent + INDENT + INDENT);

        println!("{}Right: ", " ".repeat(indent + INDENT));
        self.right.print(indent + INDENT + INDENT);
    }
}

#[derive(Debug)]
pub struct Grouping<'a> {
    expression: Box<Expression<'a>>,
}

impl<'a> Grouping<'a> {
    pub fn new(expression: Box<Expression<'a>>) -> Self {
        Grouping { expression }
    }
}

impl<'a> Expr<'a> for Grouping<'a> {
    fn print(&self, indent: usize) {
        println!("{}Expression:", " ".repeat(indent));
        self.expression.print(indent + INDENT);
    }
}

#[derive(Debug)]
pub struct Literal<'a> {
    value: Token<'a>,
}

impl<'a> Literal<'a> {
    pub fn new(value: Token<'a>) -> Self {
        Literal { value }
    }
}

impl<'a> Expr<'a> for Literal<'a> {
    fn print(&self, indent: usize) {
        println!(
            "{}Literal:\n{}{}",
            " ".repeat(indent),
            " ".repeat(indent + INDENT),
            self.value.to_string(indent + INDENT)
        );
    }
}

#[derive(Debug)]
pub struct Unary<'a> {
    operator: Token<'a>,
    right: Box<Expression<'a>>,
}

impl<'a> Unary<'a> {
    pub fn new(operator: Token<'a>, right: Box<Expression<'a>>) -> Self {
        Unary { operator, right }
    }
}

impl<'a> Expr<'a> for Unary<'a> {
    fn print(&self, indent: usize) {
        println!(
            "{}Unary:\n{}Operator:\n{}{}\n{}Right:",
            " ".repeat(indent),
            " ".repeat(indent + INDENT),
            " ".repeat(indent + INDENT + INDENT),
            self.operator.to_string(indent + INDENT + INDENT),
            " ".repeat(indent + INDENT)
        );
        self.right.print(indent + INDENT + INDENT);
    }
}
