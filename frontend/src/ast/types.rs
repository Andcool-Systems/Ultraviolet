use crate::{
    ast::traits::{GetType, IsAssignable, StringToUVMathOp, StringToUVType},
    types::{Span, Spanned},
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
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum UVType {
    Int,
    Float,
    String,
    Boolean,
    Null,

    Union(Vec<UVType>),
}

impl UVType {
    /// Create new union type
    pub fn new_union(types: Vec<UVType>) -> UVType {
        let mut flat = Vec::new();

        for t in types {
            t.flatten_into(&mut flat);
        }

        flat.sort();
        flat.dedup();

        if flat.len() == 1 {
            flat.into_iter().next().unwrap()
        } else {
            UVType::Union(flat)
        }
    }

    /// Flat Union type to provided output vector
    pub fn flatten_into(&self, out: &mut Vec<Self>) {
        match self {
            Self::Union(types) => {
                types.iter().for_each(|t| t.flatten_into(out));
            }
            t => out.push(t.clone()),
        }
    }
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

impl StringToUVType for str {
    fn to_uvtype(&self) -> Option<UVType> {
        match self {
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
    VariableAssignment(VariableAssign),
    VariableAccess(VariableAccess),

    ConditionalOp(),

    MathOp(MathOp),
    LogicalOp(),
    CompareOp(),

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
    pub name: Spanned<String>,
    pub value: Spanned<Box<ASTBlockType>>,
    pub is_const: bool,

    pub span: Span,
}

impl GetType for VariableDefinition {
    fn get_type(&self) -> UVType {
        todo!()
    }
}

// ------------------------- Variable Assign ---------------------------------

#[derive(Debug)]
pub struct VariableAssign {
    pub name: String,
    pub value: Spanned<Box<ASTBlockType>>,

    pub span: Span,
}

// ------------------------ Variable Access ----------------------------------

#[derive(Debug)]
pub struct VariableAccess {
    pub name: String,
    pub span: Span,
}

// ------------------------ Math Operations ----------------------------------
#[derive(Debug)]
pub struct MathOp {
    pub op_type: MathOpType,
    pub operands: Vec<ASTBlockType>,
}

#[derive(Debug)]
pub enum MathOpType {
    Sum,
    Sub,
    Mul,
    Div,
    Mod,
}

impl StringToUVMathOp for str {
    fn to_uvmath(&self) -> Option<MathOpType> {
        Some(match self {
            "sum" => MathOpType::Sum,
            "sub" => MathOpType::Sub,
            "mul" => MathOpType::Mul,
            "div" => MathOpType::Div,
            "mod" => MathOpType::Mod,
            _ => return None,
        })
    }
}

impl MathOpType {
    /// If math operation can handle more than two arguments
    pub fn is_variadic(&self) -> bool {
        match self {
            MathOpType::Sum | MathOpType::Mul => true,
            MathOpType::Div | MathOpType::Mod | MathOpType::Sub => false,
        }
    }
}

// ---------------------------- TESTS ----------------------------------------

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
