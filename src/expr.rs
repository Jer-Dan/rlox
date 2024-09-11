use crate::token::Token;

static INDENT: usize = 4;

trait Expr<'a> {
    fn print(&self, indent: usize);
}

pub struct Binary<'a, T: Expr<'a>> {
    left: &'a T,
    operator: &'a Token<'a>,
    right: &'a T,
}

impl<'a, T: Expr<'a>> Binary<'a, T> {
    fn new(left: &'a T, operator: &'a Token, right: &'a T) -> Self {
        Binary {
            left,
            operator,
            right,
        }
    }
}

impl<'a, T: Expr<'a>> Expr<'a> for Binary<'a, T> {
    fn print(&self, indent: usize) {
        println!(
            "{}Binary:\n{}Operator: {:?}",
            " ".repeat(indent),
            " ".repeat(indent + INDENT),
            self.operator.token_type
        );
        println!("{}Left: ", " ".repeat(indent + INDENT));
        self.left.print(indent + INDENT + INDENT);

        println!("{}Right: ", " ".repeat(indent + INDENT));
        self.right.print(indent + INDENT + INDENT);
    }
}

pub struct Grouping<'a, T: Expr<'a>> {
    expression: &'a T,
}

impl<'a, T: Expr<'a>> Grouping<'a, T> {
    pub fn new(expression: &'a T) -> Self {
        Grouping { expression }
    }
}

impl<'a, T: Expr<'a>> Expr<'a> for Grouping<'a, T> {
    fn print(&self, indent: usize) {
        println!("{}Expression:", " ".repeat(indent));
        self.expression.print(indent + INDENT);
    }
}

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
            self.value.to_string()
        );
    }
}

pub struct Unary<'a, T: Expr<'a>> {
    operator: Token<'a>,
    right: &'a T,
}

impl<'a, T: Expr<'a>> Unary<'a, T> {
    pub fn new(operator: Token<'a>, right: &'a T) -> Self {
        Unary { operator, right }
    }
}

impl<'a, T: Expr<'a>> Expr<'a> for Unary<'a, T> {
    fn print(&self, indent: usize) {
        println!(
            "{}Unary:\n{}Operator:\n{}{}\n{}Right:",
            " ".repeat(indent),
            " ".repeat(indent + INDENT),
            " ".repeat(indent + INDENT + INDENT),
            self.operator.to_string(),
            " ".repeat(indent + INDENT)
        );
        self.right.print(indent + INDENT + INDENT);
    }
}
