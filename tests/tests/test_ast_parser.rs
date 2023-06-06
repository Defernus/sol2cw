use sol2cw_ast::{
    ast_node::{ContractDefinition, PragmaDirective},
    AstNode, AstParser,
};

#[test]
fn test_empty() {
    let parser = AstParser::new("");

    let result = parser.parse();

    assert_eq!(result, Ok(vec![]));
}

#[test]
fn test_empty_contract() {
    let parser = AstParser::new(
        r#"
        contract C {}
        "#,
    );

    let result = parser.parse();

    assert_eq!(result, Ok(vec![ContractDefinition {}.into()]));
}

#[test]
fn test_pragma() {
    let parser = AstParser::new(
        r#"
        pragma solidity >=0.0;
        "#,
    );

    let result = parser.parse();

    assert_eq!(result, Ok(vec![PragmaDirective {}.into()]));
}
