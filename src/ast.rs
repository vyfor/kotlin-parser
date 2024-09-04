#[derive(Debug, PartialEq, Clone)]
pub struct KotlinFile {
    pub package: Option<Package>,
    pub imports: Vec<Import>,
    pub declarations: Vec<Declaration>,
    pub annotations: Vec<AnnotationSet>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Package {
    pub modifiers: Vec<Modifier>,
    pub names: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Import {
    pub names: Vec<String>,
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
    pub ty: DeclarationType,
}

#[derive(Debug, PartialEq, Clone)]
pub enum DeclarationType {
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
    pub type_params: Vec<TypeParam>,
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
    pub ty: ConstructorDelegateType,
    pub args: Vec<CallArg>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum ConstructorDelegateType {
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
    pub ty: Option<Type>,
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
    pub vars: Tuple,
    pub receiver: Option<Type>,
    pub bounds: Vec<TypeBound>,
    pub init: Option<Expression>,
    pub accessors: Vec<PropertyAccessor>,
}

#[derive(Debug, PartialEq, Clone)]
pub enum PropertyAccessor {
    Getter {
        modifiers: Vec<Modifier>,
        ty: Option<Type>,
        body: Option<Block>,
    },
    Setter {
        modifiers: Vec<Modifier>,
        field: Option<String>,
        body: Option<Block>,
    },
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
    Annotated(AnnotatedExpression),
    ArrayAccess(ArrayAccessExpression),
    BinaryOp(BinaryOperation),
    Break(BreakExpression),
    Call(CallExpression),
    Continue(ContinueExpression),
    For(ForExpression),
    If(IfExpression),
    Lambda(LambdaBlock),
    Labeled(LabeledExpression),
    Object(ObjectExpression),
    PropertyReference(PropertyReferenceExpression),
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
    pub then: Box<Expression>,
    pub otherwise: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ForExpression {
    pub vars: Tuple,
    pub iterable: Box<Expression>,
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct WhileExpression {
    pub expr: Box<Expression>,
    pub body: Box<Expression>,
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
    pub body: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ObjectExpression {
    pub extends: Vec<EntityDeclaration>,
    pub inner: Vec<Declaration>,
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
    pub parts: Vec<String>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LabeledExpression {
    pub label: String,
    pub expr: Box<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotatedExpression {
    pub expr: Box<Expression>,
    pub anns: Vec<AnnotationSet>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallExpression {
    pub expr: Box<Expression>,
    pub args: Vec<CallArg>,
    pub type_args: Vec<Type>,
    pub lambda: Option<Box<Expression>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LambdaBlock {
    pub label: Option<String>,
    pub vars: Tuple,
    pub body: Option<Block>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct ArrayAccessExpression {
    pub expr: Box<Expression>,
    pub index: Vec<Expression>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct PropertyReferenceExpression {
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
    pub name: Option<String>,
    pub type_args: Vec<Type>,
    pub is_nullable: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionType {
    pub receiver: Option<Type>,
    pub params: Vec<AnonymousParam>,
    pub ty: Type,
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
    pub bounds: Vec<TypeBound>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct TypeBound {
    pub ty: Type,
    pub kind: BoundKind,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BoundKind {
    Unconstrained,
    Covariant,
    Contravariant,
}

#[derive(Debug, PartialEq, Clone)]
pub struct AnnotationSet {
    pub site: Option<AnnotationSite>,
    pub annotations: Vec<Annotation>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Annotation {
    pub parts: Vec<String>,
    pub args: Vec<CallArg>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct CallArg {
    pub name: Option<String>,
    pub value: Box<Expression>,
    pub is_spread: bool,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Tuple {
    pub is_destructured: bool,
    pub vars: Vec<VarDefinition>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarDefinition {
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
