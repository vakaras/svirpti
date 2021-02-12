use crate::errors::SvirptiResult;
use svirpti_vir::{low, smt};

pub trait Lowerer {
    fn lower_variable_symbol(
        &mut self,
        symbol: &low::VariableSymbol,
    ) -> SvirptiResult<smt::VariableSymbol>;
    fn lower_function_symbol(
        &mut self,
        symbol: &low::FunctionSymbol,
    ) -> SvirptiResult<smt::FunctionSymbol>;
    fn lower_sort_symbol(
        &mut self,
        symbol: &low::UninterpretedSortSymbol,
    ) -> SvirptiResult<smt::UninterpretedSortSymbol>;
}

pub trait Lowerable<L: Lowerer> {
    type Output;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output>;
}

impl<L: Lowerer> Lowerable<L> for low::Expression {
    type Output = smt::Expression;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::Expression::Variable(expr) => smt::Expression::Variable(expr.lower(lowerer)?),
            low::Expression::Constant(expr) => smt::Expression::Constant(expr.lower(lowerer)?),
            low::Expression::UnaryOperation(expr) => {
                smt::Expression::UnaryOperation(expr.lower(lowerer)?)
            }
            low::Expression::BinaryOperation(expr) => {
                smt::Expression::BinaryOperation(expr.lower(lowerer)?)
            }
            low::Expression::Conditional(expr) => {
                smt::Expression::Conditional(expr.lower(lowerer)?)
            }
            low::Expression::Quantifier(expr) => smt::Expression::Quantifier(expr.lower(lowerer)?),
            low::Expression::FunctionApplication(expr) => {
                smt::Expression::FunctionApplication(expr.lower(lowerer)?)
            }
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Variable {
    type Output = smt::Variable;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::Variable {
            name: lowerer.lower_variable_symbol(&self.name)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Constant {
    type Output = smt::Constant;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::Constant::Bool(value) => smt::Constant::Bool(*value),
            low::Constant::Int(value) => smt::Constant::Int(*value),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::UnaryOperation {
    type Output = smt::UnaryOperation;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::UnaryOperation {
            kind: self.kind.lower(lowerer)?,
            arg: Box::new(self.arg.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::UnaryOperationKind {
    type Output = smt::UnaryOperationKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::UnaryOperationKind::Not => smt::UnaryOperationKind::Not,
            low::UnaryOperationKind::Minus => smt::UnaryOperationKind::Minus,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::BinaryOperation {
    type Output = smt::BinaryOperation;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::BinaryOperation {
            kind: self.kind.lower(lowerer)?,
            left: Box::new(self.left.lower(lowerer)?),
            right: Box::new(self.right.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::BinaryOperationKind {
    type Output = smt::BinaryOperationKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::BinaryOperationKind::EqCmp => smt::BinaryOperationKind::EqCmp,
            low::BinaryOperationKind::NeCmp => smt::BinaryOperationKind::NeCmp,
            low::BinaryOperationKind::GtCmp => smt::BinaryOperationKind::GtCmp,
            low::BinaryOperationKind::GeCmp => smt::BinaryOperationKind::GeCmp,
            low::BinaryOperationKind::LtCmp => smt::BinaryOperationKind::LtCmp,
            low::BinaryOperationKind::LeCmp => smt::BinaryOperationKind::LeCmp,
            low::BinaryOperationKind::Add => smt::BinaryOperationKind::Add,
            low::BinaryOperationKind::Sub => smt::BinaryOperationKind::Sub,
            low::BinaryOperationKind::Mul => smt::BinaryOperationKind::Mul,
            low::BinaryOperationKind::Div => smt::BinaryOperationKind::Div,
            low::BinaryOperationKind::Mod => smt::BinaryOperationKind::Mod,
            low::BinaryOperationKind::And => smt::BinaryOperationKind::And,
            low::BinaryOperationKind::Or => smt::BinaryOperationKind::Or,
            low::BinaryOperationKind::Implies => smt::BinaryOperationKind::Implies,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Conditional {
    type Output = smt::Conditional;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::Conditional {
            guard: Box::new(self.guard.lower(lowerer)?),
            then_expr: Box::new(self.then_expr.lower(lowerer)?),
            else_expr: Box::new(self.else_expr.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Quantifier {
    type Output = smt::Quantifier;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::Quantifier {
            kind: self.kind.lower(lowerer)?,
            variables: self.variables.lower(lowerer)?,
            triggers: self.triggers.lower(lowerer)?,
            body: Box::new(self.body.lower(lowerer)?),
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::QuantifierKind {
    type Output = smt::QuantifierKind;
    fn lower(&self, _lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::QuantifierKind::ForAll => smt::QuantifierKind::ForAll,
            low::QuantifierKind::Exists => smt::QuantifierKind::Exists,
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

impl<L: Lowerer> Lowerable<L> for low::BoundedVariableDecl {
    type Output = smt::BoundedVariableDecl;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::BoundedVariableDecl {
            name: lowerer.lower_variable_symbol(&self.name)?,
            sort: self.sort.lower(lowerer)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Sort {
    type Output = smt::Sort;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(match self {
            low::Sort::Bool => smt::Sort::Bool,
            low::Sort::Int => smt::Sort::Int,
            low::Sort::Real => smt::Sort::Real,
            low::Sort::Uninterpreted { name } => smt::Sort::Uninterpreted {
                name: lowerer.lower_sort_symbol(name)?,
            },
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::Trigger {
    type Output = smt::Trigger;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::Trigger {
            parts: self.parts.lower(lowerer)?,
        })
    }
}

impl<L: Lowerer> Lowerable<L> for low::FunctionApplication {
    type Output = smt::FunctionApplication;
    fn lower(&self, lowerer: &mut L) -> SvirptiResult<Self::Output> {
        Ok(smt::FunctionApplication {
            function: lowerer.lower_function_symbol(&self.function)?,
            args: self.args.lower(lowerer)?,
        })
    }
}
