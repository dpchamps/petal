pub struct IdentifierNode {
    name: String,
}

pub enum VariableDeclarationKind {
    Let,
    Const
}

pub struct VariableDeclarationNode {
    kind: VariableDeclarationKind
}

pub struct DirectiveNode {
    expression: LiteralNode,
    directive: String
}

pub struct FunctionBodyNode {
    body: FunctionBodyBody
}

pub enum ProgramNodeSourceType {
    Script,
    Module,
}

pub enum ProgramNodeBody {
    Statement(Statement),
    Directive(DirectiveNode),
    ModuleDeclaration,
}

pub struct ProgramNode {
    source_type: ProgramNodeSourceType,
    body: Vec<ProgramNodeBody>,
}

pub enum Statement {
    BlockStatement(Box<BlockStatementNode>),
    BreakableStatement(BreakableStatement),
    ExpressionStatement(Box<ExpressionStatementNode>),
    IfStatement(Box<IfStatementNode>),
    ContinueStatement(Box<ContinueStatementNode>),
    BreakStatement(Box<BreakStatementNode>),
    LabelledStatement(Box<LabelledStatementNode>),
    ReturnStatement(Box<ReturnStatementNode>),
    WithStatement(Box<WithStatementNode>),
    ThrowStatement(Box<ThrowStatementNode>),
    TryStatement(Box<TryStatementNode>),
    EmptyStatement,
    DebuggerStatement,
}

pub struct BlockStatementNode {
    body: Vec<Statement>,
}

pub enum BreakableStatement {
    IterationStatement(IterationStatement),
    SwitchStatement(Box<SwitchStatementNode>),
}

pub enum IterationStatement {
    DoWhileStatement(Box<DoWhileStatementNode>),
    WhileStatement(Box<WhileStatementNode>),
    ForStatement(Box<ForStatementNode>),
    ForInStatement(Box<ForInStatementNode>),
    ForOfStatement(Box<ForOfStatementNode>),
}

pub struct SwitchStatementNode {
    discriminant: Expression,
    cases: Vec<SwitchCaseNode>,
}

pub struct SwitchCaseNode {
    test: Option<Expression>,
    consequent: Vec<Statement>,
}

pub struct DoWhileStatementNode {
    body: Statement,
    test: Expression,
}

pub struct WhileStatementNode {
    body: Statement,
    test: Expression,
}

pub enum ForStatementInit {
    Expression(Expression),
    VariableDeclaration(Box<VariableDeclarationNode>)
}

pub struct ForStatementNode {
    init: Option<ForStatementInit>,
    test: Option<Expression>,
    update: Option<Expression>,
    body: Statement,
}

pub enum ForInStatementLeft {
    Pattern(Box<PatternNode>),
    VariableDeclaration(Box<VariableDeclarationNode>)
}

pub struct ForInStatementNode {
    left: ForInStatementLeft,
    right: Expression,
    body: Statement,
}

pub struct ForOfStatementNode {
    left: ForInStatementLeft,
    right: Expression,
    body: Statement,
    is_await: bool,
}

pub struct ExpressionStatementNode {
    expression: Expression,
}

pub struct IfStatementNode {
    test: Expression,
    consequent: Statement,
    alternate: Option<Statement>,
}

pub struct ContinueStatementNode {
    label: Option<IdentifierNode>,
}

pub struct BreakStatementNode {
    label: Option<IdentifierNode>,
}

pub struct LabelledStatementNode {
    label: IdentifierNode,
    body: Statement,
}

pub struct ReturnStatementNode {
    argument: Option<Expression>,
}

pub struct WithStatementNode {
    object: Expression,
    body: Statement,
}

pub struct ThrowStatementNode {
    argument: Expression,
}

pub struct CatchClauseNode {
    param: Option<PatternNode>,
    body: BlockStatementNode,
}

pub struct TryStatementNode {
    block: BlockStatementNode,
    handler: Option<CatchClauseNode>,
    finalizer: Option<BlockStatementNode>,
}

pub struct SpreadElementNode {
    argument: Expression
}

pub enum Expression {
    This,
    Identifier(Box<IdentifierNode>),
    ArrayLiteral(Box<ArrayLiteralNode>),
    ObjectLiteral(Box<ObjectLiteralNode>),
    FunctionExpression(Box<FunctionExpressionNode>),
    ClassExpression(Box<ClassExpressionNode>),
    TemplateLiteral(Box<TaggedTemplateExpressionNode>),
    ArrowFunctionExpression(Box<ArrowFunctionExpressionNode>),
}

pub enum LiteralNode {
    NullLiteral,
    BooleanLiteral(bool),
    NumericLiteral(i64),
    StringLiteral(String),
    // (pattern, flags)
    RegExpLiteralNode(String, String)
}

pub struct ArrayLiteralNode{
    elements: Vec<Expression>
}

pub struct ObjectLiteralNode{
    properties: Vec<PropertyNode>
}

pub enum FunctionBodyBody{
    Directive(Box<DirectiveNode>),
    Statement(Statement)
}

pub struct FunctionNode {
    id: Option<IdentifierNode>,
    params: Vec<PatternNode>,
    body: FunctionBodyNode,
    is_async: bool,
    is_gen: bool,
}

pub struct FunctionExpressionNode {
    id: Option<IdentifierNode>,
    params: Vec<PatternNode>,
    body: FunctionBodyBody,
    is_async: bool,
    is_gen: bool,
}

pub enum ArrowFunctionExpressionBody{
    FunctionBody(FunctionBodyBody),
    Expression(Expression)
}

pub struct ArrowFunctionExpressionNode {
    id: Option<IdentifierNode>,
    params: Vec<PatternNode>,
    body: ArrowFunctionExpressionBody,
    is_async: bool,
    is_gen: bool,
    is_expression: bool
}

pub enum PropertyKey {
    Literal(LiteralNode),
    Identifier(Box<IdentifierNode>)
}

pub enum PropertyKind{
    Init,
    Get,
    Set
}

pub struct PropertyNode {
    key: PropertyKey,
    value: Expression,
    kind: PropertyKind
}

pub struct ClassExpressionNode {
    id: Option<IdentifierNode>,
    super_class: Option<Expression>,
    body: ClassBodyNode
}

pub struct ClassBodyNode {
    body: Vec<MethodDefinitionNode>
}

pub enum MethodDefinitionKind {
    Constructor,
    Method,
    Get,
    Set
}

pub struct MethodDefinitionNode {
    key: Expression,
    value: FunctionExpressionNode,
    kind: MethodDefinitionKind,
    is_computed: bool,
    is_static: bool
}

pub struct ClassDeclarationNode {
    id: IdentifierNode,
    super_class: Option<Expression>,
    body: ClassBodyNode
}

pub struct ClassExpression {
    id: Option<IdentifierNode>,
    super_class: Option<Expression>,
    body: ClassBodyNode
}

pub struct YieldExpressionNode {
    argument: Option<Expression>,
    delegate: bool
}

pub struct AwaitExpressionNode {
    argument: Expression
}

pub struct TemplateLiteralNode {
    quasis: Vec<TemplateElementNode>,
    expressions: Vec<Expression>
}

pub struct TaggedTemplateExpressionNode {
    tag: Expression,
    quasi: TemplateLiteralNode
}

pub struct TemplateElementValue {
    cooked: String,
    raw: String
}

pub struct TemplateElementNode {
    tail: bool,
    value: TemplateElementValue
}

// Note to self, need to implement pattern nodes still.
//  This probably is an enum that drills down into
//  different pattern nodes.
pub struct PatternNode {}

trait ASTNode: Sized {}

impl ASTNode for FunctionBodyNode {}
impl ASTNode for FunctionNode {}
impl ASTNode for Statement {}
impl ASTNode for ProgramNodeBody {}
impl ASTNode for ProgramNode {}
impl ASTNode for BlockStatementNode {}
impl ASTNode for BreakableStatement {}
impl ASTNode for IterationStatement {}
impl ASTNode for SwitchStatementNode {}
impl ASTNode for SwitchCaseNode {}
impl ASTNode for DoWhileStatementNode {}
impl ASTNode for WhileStatementNode {}
impl ASTNode for ForStatementInit {}
impl ASTNode for ForStatementNode {}
impl ASTNode for ExpressionStatementNode {}
impl ASTNode for IfStatementNode {}
impl ASTNode for ContinueStatementNode {}
impl ASTNode for BreakStatementNode {}
impl ASTNode for LabelledStatementNode {}
impl ASTNode for ReturnStatementNode {}
impl ASTNode for WithStatementNode {}
impl ASTNode for ThrowStatementNode {}
impl ASTNode for CatchClauseNode {}
impl ASTNode for TryStatementNode {}
impl ASTNode for VariableDeclarationNode {}
impl ASTNode for LiteralNode {}
impl ASTNode for ArrayLiteralNode {}
impl ASTNode for ObjectLiteralNode {}
impl ASTNode for PropertyNode {}
impl ASTNode for ClassExpressionNode {}
impl ASTNode for ClassBodyNode {}
impl ASTNode for MethodDefinitionNode {}
impl ASTNode for ClassDeclarationNode {}
impl ASTNode for YieldExpressionNode {}
impl ASTNode for AwaitExpressionNode {}
impl ASTNode for TemplateLiteralNode {}
impl ASTNode for TemplateElementNode {}

impl ASTNode for Expression {}

impl ASTNode for PatternNode {}
