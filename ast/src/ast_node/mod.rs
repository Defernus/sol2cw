use sol2cw_lexer::{Lexer, Token};

use crate::AstResult;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    StructuredDocumentation,
    PragmaDirective,
    ImportDirective,
    ContractKind,
    ContractDefinition,
    InheritanceSpecifier,
    OverrideSpecifier,
    StructDefinition,
    EnumDefinition,
    UserDefinedValueTypeDefinition,
    EnumValue,
    VariableDeclaration,
    ModifierDefinition,
    EventDefinition,
    ErrorDefinition,
    UsingForDirective,
    ModifierInvocation,
    Identifier,
    UserDefinedTypeName,
    IdentifierPath,
    TypeName,
    FunctionTypeName,
    Mapping,
    ParameterList,
    Block,
    Statement,
    InlineAssembly,
    IfStatement,
    TryStatement,
    TryCatchClause,
    WhileStatement,
    ForStatement,
    EmitStatement,
    RevertStatement,
    VariableDeclarationStatement,
    ExpressionStatement,
    Expression,
}

impl AstNode {
    pub fn from_tokens(_start_token: Token, _lexer: &mut Lexer) -> AstResult<Self> {
        todo!("AstNode::from_tokens")
    }
}
