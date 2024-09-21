#[derive(Debug, PartialEq, Clone)]
pub struct KotlinFile {
    pub package: Option<Package>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
    pub annotations: Vec<AnnotationSet>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    pub path: Path,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub path: Path,
    pub is_wildcard: bool,
    pub alias: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement {
    Declaration(Declaration),
    Expression(Expression),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Declaration {
    pub annotations: Vec<AnnotationSet>,
    pub kind: DeclarationKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationKind {
    Constructor(ConstructorDeclaration),
    Entity(EntityDeclaration),
    EnumEntry(EnumEntryDeclaration),
    Function(FunctionDeclaration),
    InitBlock(Block),
    Property(PropertyDeclaration),
    TypeAlias(TypeAliasDeclaration),
}

#[derive(Debug, PartialEq, Clone)]
pub struct EntityDeclaration {
    pub modifiers: Vec<Modifier>,
    pub kind: EntityDeclarationKind,
    pub name: String,
    pub type_params: Vec<BoundedTypeParam>,
    pub primary_constructor: Option<PrimaryConstructorDeclaration>,
    pub constructors: Vec<ConstructorDeclaration>,
    pub bounds: Vec<TypeBound>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum EntityDeclarationKind {
    Class,
    Interface,
    Object,
    CompanionObject,
    Enum,
    ObjectInstance,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PrimaryConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub params: Vec<Param>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstructorDeclaration {
    pub modifiers: Vec<Modifier>,
    pub params: Vec<Param>,
    pub delegate: Option<ConstructorDelegate>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ConstructorDelegate {
    pub kind: ConstructorDelegateKind,
    pub args: Vec<CallArg>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstructorDelegateKind {
    This,
    Super,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionDeclaration {
    pub modifiers: Vec<Modifier>,
    pub type_params: Vec<TypeParam>,
    pub receiver: Option<Type>,
    pub name: Option<String>,
    pub params: Vec<Param>,
    pub return_ty: Option<Type>,
    pub bounds: Vec<TypeBound>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Block {
    pub statements: Vec<Statement>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertyDeclaration {
    pub modifiers: Vec<Modifier>,
    pub is_const: bool,
    pub is_mutable: bool,
    pub is_delegated: bool,
    pub type_params: Vec<TypeParam>,
    pub vars: Vars,
    pub receiver: Option<Type>,
    pub bounds: Vec<TypeBound>,
    pub init: Option<Expression>,
    pub accessors: Vec<PropertyAccessor>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyAccessor {
    Getter {
        annotations: Vec<AnnotationSet>,
        modifiers: Vec<Modifier>,
        return_ty: Option<Type>,
        body: Option<Block>,
    },
    Setter {
        annotations: Vec<AnnotationSet>,
        modifiers: Vec<Modifier>,
        field: PropertySetterField,
        body: Option<Block>,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertySetterField {
    pub name: String,
    pub ty: Option<Type>,
    pub return_ty: Option<Type>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeAliasDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub type_params: Vec<TypeParam>,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct EnumEntryDeclaration {
    pub modifiers: Vec<Modifier>,
    pub name: String,
    pub args: Vec<CallArg>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Literal(Literal),
    Bracket(BracketExpression),
    BinaryOp(BinaryOperation),
    Break(BreakExpression),
    Call(CallExpression),
    Continue(ContinueExpression),
    For(ForExpression),
    If(IfExpression),
    Lambda(LambdaBlock),
    Labeled(LabeledExpression),
    Object(ObjectExpression),
    Parenthesized(ParenthesizedExpression),
    MemberReference(MemberReferenceExpression),
    Reference(ReferenceExpression),
    Return(ReturnExpression),
    StringTemplate(StringTemplateExpression),
    Super(SuperExpression),
    This(ThisExpression),
    Throw(ThrowExpression),
    Try(TryExpression),
    UnaryOp(UnaryOperation),
    When(WhenExpression),
    While(WhileExpression),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Literal {
    UnsignedInteger(u64),
    Integer(i64),
    Decimal(f64),
    String(String),
    Char(char),
    Boolean(bool),
    Null,
}

#[derive(Debug, PartialEq, Clone)]
pub struct IfExpression {
    pub expr: Box<Expression>,
    pub then: Block,
    pub otherwise: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression {
    pub vars: Vars,
    pub iterable: Box<Expression>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpression {
    pub expr: Box<Expression>,
    pub body: Block,
    pub is_do_while: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TryExpression {
    pub body: Block,
    pub catches: Vec<CatchExpression>,
    pub finally: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CatchExpression {
    pub param: Param,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinaryOperation {
    pub lhs: Box<Expression>,
    pub op: BinaryOperator,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOperator {
    Operator(BinaryOp),
    Infix(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryOp {
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
    Dot,
    DotSafe,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOperation {
    pub op: UnaryOperator,
    pub expr: Box<Expression>,
    pub is_prefix: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOperator {
    Plus,
    Minus,
    Increment,
    Decrement,
    Not,
    NullDeref,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThisExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct SuperExpression {
    pub label: Option<String>,
    pub type_arg: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhenExpression {
    pub expr: Option<Box<Expression>>,
    pub entries: Vec<WhenEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhenEntry {
    pub exprs: Vec<Expression>,
    pub body: Block,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub annotations: Vec<AnnotationSet>,
    pub extends: Vec<EntityDeclaration>,
    pub inner: Vec<Declaration>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParenthesizedExpression {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ThrowExpression {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReturnExpression {
    pub label: Option<String>,
    pub expr: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ContinueExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BreakExpression {
    pub label: Option<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ReferenceExpression {
    pub path: Path,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledExpression {
    pub label: String,
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub path: Path,
    pub args: Vec<CallArg>,
    pub type_args: Vec<Type>,
    pub lambda: Option<Box<LambdaBlock>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaBlock {
    pub label: Option<String>,
    pub vars: Option<Vars>,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BracketExpression {
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct MemberReferenceExpression {
    pub lhs: Option<Box<Expression>>,
    pub rhs: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum StringTemplateExpression {
    Simple(String),
    Block(Block),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Simple(Box<SimpleType>),
    Function(Box<FunctionType>),
}

#[derive(Debug, PartialEq, Clone)]
pub struct SimpleType {
    pub name: String,
    pub type_args: Vec<Type>,
    pub is_nullable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    pub receiver: Option<Type>,
    pub params: Vec<AnonymousParam>,
    pub return_ty: Type,
    pub is_nullable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnonymousParam {
    pub name: Option<String>,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Param {
    pub annotations: Vec<AnnotationSet>,
    pub name: String,
    pub ty: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeParam {
    pub annotations: Vec<AnnotationSet>,
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoundedTypeParam {
    pub annotations: Vec<AnnotationSet>,
    pub bounds: Vec<TypeBound>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBound {
    pub name: String,
    pub ty: Option<Type>,
    pub kind: Option<BoundKind>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoundKind {
    In,
    Out,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationSet {
    pub site: Option<AnnotationSite>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub path: Path,
    pub args: Vec<InvocationArg>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
    pub is_spread: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct InvocationArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Vars {
    pub is_destructured: bool,
    pub vars: Vec<Var>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Var {
    pub name: String,
    pub ty: Option<Type>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum AnnotationSite {
    Field,
    Property,
    Get,
    Set,
    Receiver,
    Param,
    SetParam,
    Delegate,
}

pub type Path = Vec<String>;

#[derive(Debug, PartialEq, Clone)]
pub enum Modifier {
    Abstract,
    Final,
    Open,
    Annotation,
    Sealed,
    Data,
    Override,
    Lateinit,
    Inner,
    Private,
    Protected,
    Public,
    Internal,
    In,
    Out,
    NoInline,
    CrossInline,
    Vararg,
    Reified,
    Tailrec,
    Operator,
    Infix,
    Inline,
    External,
    Suspend,
    Const,
    Actual,
    Expect,
}
