use rand::prelude::thread_rng;
use rand::prelude::Rng;

#[derive(Debug, PartialEq)]
pub enum OpKind {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Roll,
}

pub enum Hoge{
    T1(i32),
    T2(Vec<i32>),
}

#[derive(Debug, PartialEq)]
pub struct BinaryOp {
    op_kind: OpKind,
    left_expr: Expr,
    right_expr: Expr,
}

impl BinaryOp {
    pub fn new(op_kind: OpKind, left_expr: Expr, right_expr: Expr) -> BinaryOp {
        BinaryOp {
            op_kind,
            left_expr,
            right_expr,
        }
    }

    pub fn eval(&self) -> Hoge {
        let right = self.right_expr.eval();
        let left = self.left_expr.eval();
        match self.op_kind {
            OpKind::Add => Hoge::T1(right + left),
            OpKind::Sub => Hoge::T1(right - left),
            OpKind::Mul => Hoge::T1(right * left),
            OpKind::Div => Hoge::T1(right / left),
            OpKind::Mod => Hoge::T1(right % left),
            OpKind::Roll => Hoge::T2(roll(right, left)),
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Expr {
    ConstantVal(ConstantVal),
    BinaryOp(Box<BinaryOp>),
}

impl Expr {
    pub fn eval(&self) -> i32 {
        match self {
            Expr::ConstantVal(e) => e.eval(),
            Expr::BinaryOp(e) => e.eval(),
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct ConstantVal(i32);

impl ConstantVal {
    pub fn new(val: i32) -> ConstantVal {
        ConstantVal(val)
    }
    pub fn eval(&self) -> i32 {
        self.0
    }
}

#[test]
fn binary_op_test() {
    //13*(5+1)の式を生成
    let binary_op = BinaryOp::new(
        OpKind::Mul,
        Expr::ConstantVal(ConstantVal::new(13)),
        Expr::BinaryOp(Box::new(BinaryOp::new(
            OpKind::Add,
            Expr::ConstantVal(ConstantVal::new(5)),
            Expr::ConstantVal(ConstantVal::new(1)),
        ))),
    );
    let expect = 13 * (5 + 1);
    assert_eq!(binary_op.eval(), expect);
}

// fn roll(n: i32, m: i32) -> i32 {
fn roll(n: i32, m: i32) -> Vec<i32> {
    let mut rng = thread_rng();
    let distr = rand::distributions::Uniform::new(1, m + 1);
    let mut nums = vec![0i32; n as usize];
    for x in &mut nums {
        *x = rng.sample(distr);
    }
    // nums.iter().sum()
    return nums;
}
