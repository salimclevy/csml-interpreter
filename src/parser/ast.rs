pub type Program = Vec<BlockStmt>;

pub struct BlockStmt { 
    Label: String,
    Block: Vec<Stmt>,
}

#[derive(PartialEq, Debug, Clone)]
pub enum Stmt {
    ExprStmt(Expr),
    MemStmt(Ident, Expr),
}

#[derive(PartialEq, Debug, Clone)]
pub enum Expr {
    Action { fun: Literal, act: Box<Expr> },
    IdentExpr(Ident),
    LitExpr(Literal),
    IfExpr {
        cond: Box<Expr>,
        consequence: Vec<Expr>,
    },
    ArrayExpr(Vec<Expr>),
    // PrefixExpr(Prefix, Box<Expr>),
    // InfixExpr(Infix, Box<Expr>, Box<Expr>),
    // FnExpr { params: Vec<Ident>, body: BlockStmt },
    // HashExpr(Vec<(Literal, Expr)>),
    // IndexExpr { array: Box<Expr>, index: Box<Expr> },
}

#[derive(PartialEq, Debug, Clone)]
pub enum Literal {
    IntLiteral(i64),
    StringLiteral(String),
    // BoolLiteral(bool),
}

#[derive(PartialEq, Debug, Eq, Clone)]
pub struct Ident(pub String);

// #[derive(PartialEq, Debug, Clone)]
// pub enum Prefix {
//     PrefixPlus,
//     PrefixMinus,
//     Not,
// }

#[derive(PartialEq, Debug, Clone)]
pub enum Infix {
    // Plus,
    // Minus,
    // Divide,
    // Multiply,
    Equal,
    // NotEqual,
    GreaterThanEqual,
    LessThanEqual,
    GreaterThan,
    LessThan,
}

// #[derive(PartialEq, PartialOrd, Debug, Clone)]
// pub enum Precedence {
//     PLowest,
//     PEquals,
//     PLessGreater,
//     PSum,
//     PProduct,
//     PCall,
//     PIndex,
// }
