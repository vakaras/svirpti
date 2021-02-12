use super::context::*;

vir_include! { expression =>
    use Variable;
    use Constant;
    use UnaryOperation;
    use UnaryOperationKind;
    use BinaryOperation;
    use BinaryOperationKind;
    use Conditional;
    use Quantifier;
    use QuantifierKind;
    use Trigger;
    use BoundedVariableDecl;
    use FunctionApplication;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { expression::helpers =>
    use VariableHelpers;
    use ConstantHelpers;
    use UnaryOperationHelpers;
    use BinaryOperationHelpers;
    use QuantifierHelpers;
    use FunctionApplicationHelpers;
}
pub use crate::common::expression::{
    BinaryOperationHelpers, ConstantHelpers, FunctionApplicationHelpers, QuantifierHelpers,
    UnaryOperationHelpers, VariableHelpers,
};
vir_include! { expression::display =>
    use Variable;
    use Constant;
    use UnaryOperation;
    use BinaryOperation;
    use BinaryOperationKind;
    use Conditional;
    use Quantifier;
    use QuantifierKind;
    use BoundedVariableDecl;
    use Trigger;
    use FunctionApplication;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum Expression {
    Variable(Variable),
    Constant(Constant),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    Quantifier(Quantifier),
    FunctionApplication(FunctionApplication),
}

impl std::fmt::Display for Expression {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expression::Variable(expr) => expr.fmt(f),
            Expression::Constant(expr) => expr.fmt(f),
            Expression::UnaryOperation(expr) => expr.fmt(f),
            Expression::BinaryOperation(expr) => expr.fmt(f),
            Expression::Conditional(expr) => expr.fmt(f),
            Expression::Quantifier(expr) => expr.fmt(f),
            Expression::FunctionApplication(expr) => expr.fmt(f),
        }
    }
}
