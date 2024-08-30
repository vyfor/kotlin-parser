#[derive(Debug, PartialEq, Clone)]
pub enum Stmt {
    Declaration(DeclarationKind),
    Conditional(ConditionalKind),
    Repeat(RepeatKind),
    ExprStmt(Box<Expr>),
    Scope(Block),

    Return(Option<Box<Expr>>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationKind {
    Property {
        visibility: Option<VisibilityModifier>,
        inheritance: Option<InheritanceModifier>,
        mutable: bool,
        name: String,
        ty: Option<Type>,
        init: Option<Box<Expr>>,
    },
    Function {
        visibility: Option<VisibilityModifier>,
        inheritance: Option<InheritanceModifier>,
        name: String,
        params: Vec<(String, Type)>,
        ty: Option<Type>,
        body: Block,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConditionalKind {
    If {
        branches: Vec<(Expr, Block)>,
        otherwise: Option<Block>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum RepeatKind {
    While {
        condition: Box<Expr>,
        body: Block,
        do_while: bool,
    },
    For {
        condition: Box<Expr>,
        body: Block,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum VisibilityModifier {
    Public,
    Internal,
    Protected,
    Private,
}

#[derive(Debug, PartialEq, Clone)]
pub enum InheritanceModifier {
    Open,
    Final,
    Abstract,
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
    As,
    AsNullable,
    Elvis,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Not,
    Plus,
    Minus,
    Increment,
    Decrement,
}
