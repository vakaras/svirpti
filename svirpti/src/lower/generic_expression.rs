// TODO: Merge this code with `lower.rs` in `wp`.

use crate::errors::SvirptiResult;
use svirpti_vir::{high, low};

pub trait Lowerer {
    fn lower_variable_symbol(
        &mut self,
        symbol: &high::VariableSymbol,
    ) -> SvirptiResult<low::VariableSymbol>;
    fn lower_function_symbol(
        &mut self,
        symbol: &high::FunctionSymbol,
    ) -> SvirptiResult<low::FunctionSymbol>;
    fn lower_sort_symbol(
        &mut self,
        symbol: &high::UninterpretedSortSymbol,
    ) -> SvirptiResult<low::UninterpretedSortSymbol>;
}

pub trait Lowerable<L: Lowerer> {
    type Output;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output>;
}

impl<L: Lowerer> Lowerable<L> for high::Expression {
    type Output = low::Expression;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::Expression::Variable(expr) => low::Expression::Variable(expr.lower(lowerer)?),
            high::Expression::Constant(expr) => low::Expression::Constant(expr.lower(lowerer)?),
            high::Expression::UnaryOperation(expr) => {
                low::Expression::UnaryOperation(expr.lower(lowerer)?)
            }
            high::Expression::BinaryOperation(expr) => {
                low::Expression::BinaryOperation(expr.lower(lowerer)?)
            }
            high::Expression::Conditional(expr) => {
                low::Expression::Conditional(expr.lower(lowerer)?)
            }
            high::Expression::Quantifier(expr) => low::Expression::Quantifier(expr.lower(lowerer)?),
            high::Expression::FunctionApplication(expr) => {
                low::Expression::FunctionApplication(expr.lower(lowerer)?)
            }
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Variable {
    type Output = low::Variable;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::Variable {
            name: lowerer.lower_variable_symbol(&self.name)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Constant {
    type Output = low::Constant;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::Constant::Bool(value) => low::Constant::Bool(*value),
            high::Constant::Int(value) => low::Constant::Int(*value),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::UnaryOperation {
    type Output = low::UnaryOperation;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::UnaryOperation {
            kind: self.kind.lower(lowerer)?,
            arg: Box::new(self.arg.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::UnaryOperationKind {
    type Output = low::UnaryOperationKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::UnaryOperationKind::Not => low::UnaryOperationKind::Not,
            high::UnaryOperationKind::Minus => low::UnaryOperationKind::Minus,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::BinaryOperation {
    type Output = low::BinaryOperation;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::BinaryOperation {
            kind: self.kind.lower(lowerer)?,
            left: Box::new(self.left.lower(lowerer)?),
            right: Box::new(self.right.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::BinaryOperationKind {
    type Output = low::BinaryOperationKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::BinaryOperationKind::EqCmp => low::BinaryOperationKind::EqCmp,
            high::BinaryOperationKind::NeCmp => low::BinaryOperationKind::NeCmp,
            high::BinaryOperationKind::GtCmp => low::BinaryOperationKind::GtCmp,
            high::BinaryOperationKind::GeCmp => low::BinaryOperationKind::GeCmp,
            high::BinaryOperationKind::LtCmp => low::BinaryOperationKind::LtCmp,
            high::BinaryOperationKind::LeCmp => low::BinaryOperationKind::LeCmp,
            high::BinaryOperationKind::Add => low::BinaryOperationKind::Add,
            high::BinaryOperationKind::Sub => low::BinaryOperationKind::Sub,
            high::BinaryOperationKind::Mul => low::BinaryOperationKind::Mul,
            high::BinaryOperationKind::Div => low::BinaryOperationKind::Div,
            high::BinaryOperationKind::Mod => low::BinaryOperationKind::Mod,
            high::BinaryOperationKind::And => low::BinaryOperationKind::And,
            high::BinaryOperationKind::Or => low::BinaryOperationKind::Or,
            high::BinaryOperationKind::Implies => low::BinaryOperationKind::Implies,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Conditional {
    type Output = low::Conditional;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::Conditional {
            guard: Box::new(self.guard.lower(lowerer)?),
            then_expr: Box::new(self.then_expr.lower(lowerer)?),
            else_expr: Box::new(self.else_expr.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Quantifier {
    type Output = low::Quantifier;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::Quantifier {
            kind: self.kind.lower(lowerer)?,
            variables: self.variables.lower(lowerer)?,
            triggers: self.triggers.lower(lowerer)?,
            body: Box::new(self.body.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::QuantifierKind {
    type Output = low::QuantifierKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::QuantifierKind::ForAll => low::QuantifierKind::ForAll,
            high::QuantifierKind::Exists => low::QuantifierKind::Exists,
        })
    }
}

impl<L: Lowerer, T: Lowerable<L>> Lowerable<L> for Vec<T> {
    type Output = Vec<<T as Lowerable<L>>::Output>;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        let mut vec = Vec::with_capacity(self.len());
        for e in self {
            vec.push(e.lower(lowerer)?);
        }
        Ok(vec)
    }
}

impl<L: Lowerer> Lowerable<L> for high::BoundedVariableDecl {
    type Output = low::BoundedVariableDecl;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::BoundedVariableDecl {
            name: lowerer.lower_variable_symbol(&self.name)?,
            sort: self.sort.lower(lowerer)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Type {
    type Output = low::Sort;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            high::Type::Bool => low::Sort::Bool,
            high::Type::Int => low::Sort::Int,
            high::Type::Real => low::Sort::Real,
            high::Type::Domain(high::DomainType { name }) => low::Sort::Uninterpreted {
                name: lowerer.lower_sort_symbol(name)?,
            },
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::Trigger {
    type Output = low::Trigger;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::Trigger {
            parts: self.parts.lower(lowerer)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for high::FunctionApplication {
    type Output = low::FunctionApplication;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(low::FunctionApplication {
            function: lowerer.lower_function_symbol(&self.function)?,
            args: self.args.lower(lowerer)?,
        })
    }
}
