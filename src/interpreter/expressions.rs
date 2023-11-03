use crate::interpreter::memory;
use std::collections::HashMap;

#[derive(Copy, Clone)]
pub enum ExprType {
    // Int
    ConstInt,
    NegInt,
    ReadInt,
    BinaryInt,
    Var,

    // Bool
    ConstBool,
    SingleBool,
    NotBool,
}

#[derive(Copy, Clone)]
pub enum IntOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
}

#[derive(Copy, Clone)]
pub enum BoolOp {
    Equal,
    NotEqual,
    Lower,
    Greater,
    LowerEqual,
    GreaterEqual,
}

#[derive(Clone)]
pub enum Expression {
    Ex(Expr),
    Bi(BinaryInt),
    Sb(SingleBool),
}

#[derive(Clone)]
pub struct Expr {
    pub expr: ExprType,
    pub value: i32,
    pub name: String,
}

#[derive(Clone)]
pub struct BinaryInt {
    pub left: Expr,
    pub op: IntOp,
    pub right: Expr,
}

#[derive(Clone)]
pub struct SingleBool {
    pub left: Expr,
    pub op: BoolOp,
    pub right: Expr,
}

impl Expression {
    pub fn exec(&self, map: &mut HashMap<String, i32>) -> i32 {
        match self {
            Expression::Ex(expr) => expr.exec(map),
            Expression::Bi(binary_int) => binary_int.exec(map),
            Expression::Sb(single_bool) => single_bool.exec(map),
        }
    }
}

impl Expr {
    fn exec(&self, map: &mut HashMap<String, i32>) -> i32 {
        match self.expr {
            ExprType::ConstBool | ExprType::ConstInt => self.value,
            ExprType::ReadInt => {
                let sin = std::io::stdin();
                let mut s = String::new();

                sin.read_line(&mut s).expect("Failed to read line");
                s = s.trim().to_string();

                s.parse::<i32>().expect("Bad input")
            }
            ExprType::NegInt => -self.value,
            ExprType::NotBool => !self.value,
            ExprType::Var => memory::read(map, &self.name),
            _ => panic!("Invalid type"),
        }
    }
}

impl BinaryInt {
    fn exec(&self, map: &mut HashMap<String, i32>) -> i32 {
        match self.op {
            IntOp::Add => self.left.exec(map) + self.right.exec(map),
            IntOp::Sub => self.left.exec(map) - self.right.exec(map),
            IntOp::Mul => self.left.exec(map) * self.right.exec(map),
            IntOp::Div => self.left.exec(map) / self.right.exec(map),
            IntOp::Mod => self.left.exec(map) % self.right.exec(map),
            IntOp::Pow => self
                .left
                .exec(map)
                .pow(self.right.exec(map).try_into().unwrap()),
        }
    }
}

impl SingleBool {
    fn exec(&self, map: &mut HashMap<String, i32>) -> i32 {
        match self.op {
            BoolOp::Equal => (self.left.exec(map) == self.right.exec(map)) as i32,
            BoolOp::NotEqual => (self.left.exec(map) != self.right.exec(map)) as i32,
            BoolOp::Greater => (self.left.exec(map) > self.right.exec(map)) as i32,
            BoolOp::Lower => (self.left.exec(map) < self.right.exec(map)) as i32,
            BoolOp::GreaterEqual => (self.left.exec(map) >= self.right.exec(map)) as i32,
            BoolOp::LowerEqual => (self.left.exec(map) <= self.right.exec(map)) as i32,
        }
    }
}
