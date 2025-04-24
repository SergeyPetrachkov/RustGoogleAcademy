// This is an implementation of this exercise: 
// https://google.github.io/comprehensive-rust/pattern-matching/exercise.html


/// An operation to perform on two subexpressions.
#[derive(Debug)]
enum Operation {
    Add,
    Sub,
    Mul,
    Div,
}

/// An expression, in tree form.
#[derive(Debug)]
enum Expression {
    /// An operation on two subexpressions.
    Op { op: Operation, left: Box<Expression>, right: Box<Expression> },

    /// A literal value
    Value(i64),
}


fn main() {
    println!("Hello, world!");
}
