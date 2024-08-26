#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    VarDecl {
        mutable: bool,
        name: String,
        ty: Option<Type>,
        init: Option<Box<Expr>>,
    },
    FunDecl {
        name: String,
        params: Vec<(String, Type)>,
        ty: Option<Type>,
        body: Block,
    },
    ExprStmt(Box<Expr>),

    If {
        branches: Vec<(Expr, Block)>,
        otherwise: Option<Block>,
    },
    While {
        condition: Box<Expr>,
        body: Block,
        do_while: bool,
    },
    For {
        condition: Box<Expr>,
        body: Block,
    },
    Return(Option<Box<Expr>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr {
    Literal(Literal),
    Variable(String),
    BinaryOp(Box<Expr>, BinaryOperator, Box<Expr>),
    UnaryOp(UnaryOperator, Box<Expr>),
    Call(String, Vec<Type>, Vec<Expr>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Box<Stmt>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    Integer(isize),
    Float(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Type {
    pub kind: TypeKind,
    pub nullable: bool,
    pub attributes: Vec<Box<Type>>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TypeKind {
    // Numeric
    Byte,
    UByte,
    Short,
    UShort,
    Int,
    UInt,
    Long,
    ULong,
    Float,
    Double,

    // String
    Char,
    String,

    // Collection
    Array,
    ByteArray,
    UByteArray,
    ShortArray,
    UShortArray,
    IntArray,
    UIntArray,
    LongArray,
    ULongArray,
    FloatArray,
    DoubleArray,
    BooleanArray,
    CharArray,
    Map,
    MutableMap,
    EmptyMap,
    Set,
    MutableSet,
    EmptySet,
    List,
    MutableList,
    EmptyList,

    // Other
    Boolean,
    Any,
    Nothing,
    Unit,
    Object(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Assign,
    AddAssign,
    SubtractAssign,
    MultiplyAssign,
    DivideAssign,
    ModuloAssign,
    Equal,
    NotEqual,
    ReferenceEqual,
    ReferenceNotEqual,
    LessThan,
    LessThanOrEqual,
    GreaterThan,
    GreaterThanOrEqual,
    And,
    Or,
    Xor,
    Shl,
    Shr,
    UShr,
    In,
    NotIn,
    Is,
    IsNot,
    RangeTo,
    RangeUntil,
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
    Increment,
    Decrement,
}
