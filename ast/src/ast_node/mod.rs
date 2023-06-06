use sol2cw_lexer::{Keyword, Lexer, Token};

use crate::AstResult;

pub use self::{
    block::Block, contract_definition::ContractDefinition, contract_kind::ContractKind,
    emit_statement::EmitStatement, enum_definition::EnumDefinition, enum_value::EnumValue,
    error_definition::ErrorDefinition, event_definition::EventDefinition, expression::Expression,
    expression_statement::ExpressionStatement, for_statement::ForStatement,
    function_type_name::FunctionTypeName, identifier::Identifier, identifier_path::IdentifierPath,
    if_statement::IfStatement, import_directive::ImportDirective,
    inheritance_specifier::InheritanceSpecifier, inline_assembly::InlineAssembly, mapping::Mapping,
    modifier_definition::ModifierDefinition, modifier_invocation::ModifierInvocation,
    override_specifier::OverrideSpecifier, parameter_list::ParameterList,
    pragma_directive::PragmaDirective, revert_statement::RevertStatement, statement::Statement,
    struct_definition::StructDefinition, structured_documentation::StructuredDocumentation,
    try_catch_clause::TryCatchClause, try_statement::TryStatement, type_name::TypeName,
    user_defined_type_name::UserDefinedTypeName,
    user_defined_value_type_definition::UserDefinedValueTypeDefinition,
    using_for_directive::UsingForDirective, variable_declaration::VariableDeclaration,
    variable_declaration_statement::VariableDeclarationStatement, while_statement::WhileStatement,
};

pub mod block;
pub mod contract_definition;
pub mod contract_kind;
pub mod emit_statement;
pub mod enum_definition;
pub mod enum_value;
pub mod error_definition;
pub mod event_definition;
pub mod expression;
pub mod expression_statement;
pub mod for_statement;
pub mod function_type_name;
pub mod identifier;
pub mod identifier_path;
pub mod if_statement;
pub mod import_directive;
pub mod inheritance_specifier;
pub mod inline_assembly;
pub mod mapping;
pub mod modifier_definition;
pub mod modifier_invocation;
pub mod override_specifier;
pub mod parameter_list;
pub mod pragma_directive;
pub mod revert_statement;
pub mod statement;
pub mod struct_definition;
pub mod structured_documentation;
pub mod try_catch_clause;
pub mod try_statement;
pub mod type_name;
pub mod user_defined_type_name;
pub mod user_defined_value_type_definition;
pub mod using_for_directive;
pub mod variable_declaration;
pub mod variable_declaration_statement;
pub mod while_statement;

#[derive(Debug, Clone, PartialEq)]
pub enum AstNode {
    StructuredDocumentation(StructuredDocumentation),
    PragmaDirective(PragmaDirective),
    ImportDirective(ImportDirective),
    ContractKind(ContractKind),
    ContractDefinition(ContractDefinition),
    InheritanceSpecifier(InheritanceSpecifier),
    OverrideSpecifier(OverrideSpecifier),
    StructDefinition(StructDefinition),
    EnumDefinition(EnumDefinition),
    UserDefinedValueTypeDefinition(UserDefinedValueTypeDefinition),
    EnumValue(EnumValue),
    VariableDeclaration(VariableDeclaration),
    ModifierDefinition(ModifierDefinition),
    EventDefinition(EventDefinition),
    ErrorDefinition(ErrorDefinition),
    UsingForDirective(UsingForDirective),
    ModifierInvocation(ModifierInvocation),
    Identifier(Identifier),
    UserDefinedTypeName(UserDefinedTypeName),
    IdentifierPath(IdentifierPath),
    TypeName(TypeName),
    FunctionTypeName(FunctionTypeName),
    Mapping(Mapping),
    ParameterList(ParameterList),
    Block(Block),
    Statement(Statement),
    InlineAssembly(InlineAssembly),
    IfStatement(IfStatement),
    TryStatement(TryStatement),
    TryCatchClause(TryCatchClause),
    WhileStatement(WhileStatement),
    ForStatement(ForStatement),
    EmitStatement(EmitStatement),
    RevertStatement(RevertStatement),
    VariableDeclarationStatement(VariableDeclarationStatement),
    ExpressionStatement(ExpressionStatement),
    Expression(Expression),
}

impl AstNode {
    pub fn from_tokens(start_token: Token, lexer: &mut Lexer) -> AstResult<Self> {
        match start_token {
            Token::Keyword(Keyword::Pragma) => {
                Ok(PragmaDirective::parse(start_token, lexer)?.into())
            }
            _ => unimplemented!("token \"{:?}\" not implemented yet", start_token),
        }
    }
}
