use crate::{
    ast::traits::{GetType, GetTypeScope, IsAssignable, StringToUVType},
    types::Span,
};

/// Typed value container
#[derive(Debug)]
pub enum UVValue {
    Int(i64),
    Float(f64),
    String(String),
    Boolean(bool),
    Null,
}

impl GetType for UVValue {
    fn get_type(&self) -> UVType {
        match self {
            UVValue::Int(_) => UVType::Int,
            UVValue::Float(_) => UVType::Float,
            UVValue::String(_) => UVType::String,
            UVValue::Boolean(_) => UVType::Boolean,
            UVValue::Null => UVType::Null,
        }
    }
}

/// Ultraviolet primitive types
#[derive(PartialEq, Debug)]
pub enum UVType {
    Int,
    Float,
    String,
    Boolean,
    Null,

    Union(Vec<UVType>),
}

impl IsAssignable for UVType {
    fn is_assignable_from(&self, other: &UVType) -> bool {
        if self == other {
            return true;
        }

        match (self, other) {
            (_, UVType::Union(types)) => types.iter().all(|t| self.is_assignable_from(t)),
            (UVType::Union(types), _) => types.iter().any(|t| t.is_assignable_from(other)),

            _ => false,
        }
    }
}

// -------------------- String-Type conversion --------------
impl StringToUVType for String {
    fn to_uvtype(&self) -> Option<UVType> {
        match self.as_str() {
            "int" => Some(UVType::Int),
            "float" => Some(UVType::Float),
            "str" => Some(UVType::String),
            "bool" => Some(UVType::Boolean),
            "null" => Some(UVType::Null),
            _ => None,
        }
    }
}

// ---------------
/*
#[derive(Debug)]
pub enum Symbol {
    /// Primitive type
    Primitive(UVValue),

    /// Name of the variable in scope
    Variable(String),
}

impl GetTypeScope for Symbol {
    fn get_type_from_scope(&self, scope: Option<usize>) -> UVType {
        match self {
            Self::Primitive(val) => val.get_type(),
            // Scope-based search of the final primitive
            Self::Variable(var) => todo!(),
        }
    }
}
*/

// --------------------------- AST-TYPES ---------------------------
#[derive(Debug)]
pub enum ASTBlockType {
    Program(Box<ProgramBlock>),

    HeadBlock(Vec<ASTBlockType>),
    MainBlock(Vec<ASTBlockType>),

    VariableDefinition(VariableDefinition),
    FunctionDefinition(),

    FunctionCall(),
    VariableAssignment(),

    ConditionalOp(),

    MathOp(),
    LogicalOp(),

    ForLoop(),
    WhileLoop(),

    Value(UVValue),
    Type(UVType),

    GroupBlock(),
}

// --------------------------- PROGRAM BLOCK ------------------------

#[derive(Debug)]
pub struct ProgramBlock {
    pub head: Option<ASTBlockType>,
    pub main: ASTBlockType,

    pub span: Span,
}

// --------------------------- VariableDefinition BLOCK ------------------------

#[derive(Debug)]
pub struct VariableDefinition {
    pub name: String,
    pub value: Box<ASTBlockType>,
    pub is_const: bool,
}

impl GetType for VariableDefinition {
    fn get_type(&self) -> UVType {
        todo!()
    }
}

// ---------------------------- TESTS -------------------------------

#[cfg(test)]
mod tests {
    use crate::ast::{
        traits::{IsAssignable, StringToUVType},
        types::UVType,
    };

    #[test]
    fn parse_type() {
        assert_eq!(String::from("int").to_uvtype(), Some(UVType::Int));
        assert_eq!(String::from("bool").to_uvtype(), Some(UVType::Boolean));
        assert_eq!(String::from("float").to_uvtype(), Some(UVType::Float));
        assert_eq!(String::from("null").to_uvtype(), Some(UVType::Null));
        assert_eq!(String::from("str").to_uvtype(), Some(UVType::String));

        assert_eq!(String::from("unknown").to_uvtype(), None);
    }

    #[test]
    fn type_compatible_with() {
        assert_eq!(
            UVType::Union(vec![UVType::Int, UVType::Null]).is_assignable_from(&UVType::Null),
            true
        );

        assert_eq!(
            UVType::Int.is_assignable_from(&UVType::Union(vec![UVType::Int, UVType::Null])),
            false
        );

        assert_eq!(UVType::Int.is_assignable_from(&UVType::Boolean), false);
    }
}
