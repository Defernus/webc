use serde::Serialize;

use crate::{
    lexer::{
        source::{position::WebcSourcePosition, TokensIter},
        tokens::TokenData,
    },
    AstError, AstResult,
};

use super::{
    assignment::AstAssignment, binary_operation::AstBinaryOperation,
    function_call::AstFunctionCall, function_declaration::AstFunctionDeclaration,
    identifier::AstIdentifier, if_statement::AstIfStatement, left_expression::AstLeftExpression,
    literal::AstLiteral, loop_interrupt::AstLoopInterrupt, loop_statement::AstLoopStatement,
    return_statement::AstReturnStatement, right_expression::AstRightExpression, scope::AstScope,
    type_declaration::AstTypeDeclaration, type_definition::AstTypeDefinition,
    variable_declaration::AstVariableDeclaration,
};

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub enum AstNode {
    Scope(AstScope),
    Assignment(AstAssignment),
    BinaryOperation(AstBinaryOperation),
    LeftExpression(AstLeftExpression),
    RightExpression(AstRightExpression),
    Literal(AstLiteral),
    Identifier(AstIdentifier),
    FunctionCall(AstFunctionCall),
    FunctionDeclaration(AstFunctionDeclaration),
    VariableDeclaration(AstVariableDeclaration),
    TypeDeclaration(AstTypeDeclaration),
    TypeDefinition(AstTypeDefinition),
    ReturnStatement(AstReturnStatement),
    IfStatement(AstIfStatement),
    LoopStatement(AstLoopStatement),
    LoopInterrupt(AstLoopInterrupt),
    Noop,
}

pub trait NodeParser {
    fn parse(tokens: TokensIter) -> Option<(AstResult<Self>, TokensIter)>
    where
        Self: Sized;

    fn parse_as_node(tokens: TokensIter) -> Option<(AstResult<AstNode>, TokensIter)>
    where
        Self: Sized + Into<AstNode>,
    {
        let (node, tokens) = Self::parse(tokens)?;
        let node: AstResult<AstNode> = node.map(Into::into);

        Some((node, tokens))
    }
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstNodeData {
    node: AstNode,
    start: TokenData,
    end: TokenData,
}

impl AstNodeData {
    /// The start token of the node
    pub fn start(&self) -> &TokenData {
        &self.start
    }

    /// The token where of the node
    pub fn end(&self) -> &TokenData {
        &self.end
    }

    /// Position in the source file of the node
    pub fn start_position(&self) -> WebcSourcePosition {
        self.start.start()
    }

    /// Position in the source file of the node
    pub fn end_position(&self) -> WebcSourcePosition {
        self.end.end()
    }

    /// Parses a node from the given tokens
    pub fn parse(tokens: &mut TokensIter) -> AstResult<Self> {
        macro_rules! try_parse_node {
            ($tokens:expr, $($ast:ident),*) => {
                {
                    let result =$(
                        if let Some(v) = $ast::parse_as_node(tokens.clone()) {
                            v
                        } else
                    )*
                    {
                        return Err(AstError::UnexpectedToken);
                    };

                    result
                }
            };
        }

        let (node, new_tokens) = try_parse_node!(
            tokens,
            AstScope,
            AstAssignment,
            AstBinaryOperation,
            AstLeftExpression,
            AstRightExpression,
            AstLiteral,
            AstIdentifier,
            AstFunctionCall,
            AstFunctionDeclaration,
            AstVariableDeclaration,
            AstTypeDeclaration,
            AstTypeDefinition,
            AstReturnStatement,
            AstIfStatement,
            AstLoopStatement,
            AstLoopInterrupt
        );

        let node = node?;
        *tokens = new_tokens;

        let start = tokens.get_next().unwrap().clone();
        let end = tokens.get_prev().unwrap().clone();

        Ok(AstNodeData { node, start, end })
    }
}

#[derive(Debug, Clone, Serialize, Eq, PartialEq)]
pub struct AstNodeScope {}
