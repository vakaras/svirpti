use super::context::*;
use super::sort::WithSort;

vir_include! { expression =>
    use Variable;
    use Constant;
    use UnaryOperation;
    use BinaryOperation;
    use Conditional;
    use Quantifier;
    use QuantifierKind;
    use Trigger;
    use BoundedVariableDecl;
    use FunctionApplication;
    use LabelledExpression;
    use LabelPositivity;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}
vir_include! { expression =>
    use UnaryOperationKind;
    use BinaryOperationKind;
    derive PartialEq, Eq, Debug, Clone, Copy, serde::Serialize, serde::Deserialize;
}
vir_include! { expression::sort =>
    use Variable;
    use Constant;
    use UnaryOperation;
    use BinaryOperation;
    use Conditional;
    use Quantifier;
    use FunctionApplication;
    use LabelledExpression;
}
vir_include! { expression::helpers =>
    use VariableHelpers;
    use ConstantHelpers;
    use UnaryOperationHelpers;
    use BinaryOperationHelpers;
    use QuantifierHelpers;
    use FunctionApplicationHelpers;
    use LabelledExpressionHelpers;
}
pub use crate::common::expression::{
    BinaryOperationHelpers, ConstantHelpers, FunctionApplicationHelpers, LabelledExpressionHelpers,
    QuantifierHelpers, UnaryOperationHelpers, VariableHelpers,
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
    use LabelledExpression;
}
vir_include! { expression::rsmt =>
    use Variable;
    use Constant;
    use UnaryOperation;
    use BinaryOperation;
    use Conditional;
    use Quantifier;
    use Trigger;
    use FunctionApplication;
    use LabelledExpression;
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
    LabelledExpression(LabelledExpression),
}

impl<C: Context> WithSort<C> for Expression {
    fn sort<'a>(&'a self, context: &'a C) -> &'a Sort {
        match self {
            Expression::Variable(expr) => expr.sort(context),
            Expression::Constant(expr) => expr.sort(context),
            Expression::UnaryOperation(expr) => expr.sort(context),
            Expression::BinaryOperation(expr) => expr.sort(context),
            Expression::Conditional(expr) => expr.sort(context),
            Expression::Quantifier(expr) => expr.sort(context),
            Expression::FunctionApplication(expr) => expr.sort(context),
            Expression::LabelledExpression(expr) => expr.sort(context),
        }
    }
}

impl<'a, C: Context> ::rsmt2::print::Expr2Smt<&'a C> for Expression {
    fn expr_to_smt2<Writer: std::io::Write>(
        &self,
        writer: &mut Writer,
        context: &'a C,
    ) -> ::rsmt2::SmtRes<()> {
        match self {
            Expression::Variable(expr) => expr.expr_to_smt2(writer, context),
            Expression::Constant(expr) => expr.expr_to_smt2(writer, context),
            Expression::UnaryOperation(expr) => expr.expr_to_smt2(writer, context),
            Expression::BinaryOperation(expr) => expr.expr_to_smt2(writer, context),
            Expression::Conditional(expr) => expr.expr_to_smt2(writer, context),
            Expression::Quantifier(expr) => expr.expr_to_smt2(writer, context),
            Expression::FunctionApplication(expr) => expr.expr_to_smt2(writer, context),
            Expression::LabelledExpression(expr) => expr.expr_to_smt2(writer, context),
        }
    }
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
            Expression::LabelledExpression(expr) => expr.fmt(f),
        }
    }
}
