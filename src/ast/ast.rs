use crate::lexer::token::TokenType;

#[derive(Debug, Clone)]
pub enum ASTNode {
    Program(Box<Vec<ASTNode>>),

    FunctionExpression(FunctionExpression),
    FunctionCall(FunctionCall),

    ReturnExpression(Box<ASTNode>),
    VariableExpression(VariableExpression),
    BinaryExpression(BinaryExpression),
    UnaryExpression(Box<ASTNode>),

    IfStatement(IfStatement),
    BlockStatement(BlockStatement),

    Variable(String),
    Number(f64),
    Boolean(bool),
    String(String),

    Command(Box<Vec<ASTNode>>),
}

#[derive(Debug, Clone)]
pub struct BinaryExpression {
    pub left: Box<ASTNode>,
    pub operator: TokenType,
    pub right: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct VariableExpression {
    pub name: String,
    pub rhs: Box<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct FunctionExpression {
    pub name: String,
    pub body: Box<ASTNode>,
    pub args: Vec<String>,
}

impl PartialEq for FunctionExpression {
    fn eq(&self, _: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub struct FunctionCall {
    pub name: String,
    pub args: Vec<ASTNode>,
}

#[derive(Debug, Clone)]
pub struct IfStatement {
    pub condition: Box<ASTNode>,
    pub consequence: Box<ASTNode>,
    pub alternative: Option<Box<ASTNode>>,
}

#[derive(Debug, Clone)]
pub struct BlockStatement {
    pub body: Box<Vec<ASTNode>>,
}
