vir_include! { expression::parse_ast =>
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
    derive PartialEq, Eq, Debug, Clone;
}
vir_include! { expression::parse =>
    use kw;
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
    use Expression;
}

use super::sort::Sort;

#[derive(PartialEq, Eq, Debug, Clone)]
pub enum Expression {
    Variable(Variable),
    Constant(Constant),
    UnaryOperation(UnaryOperation),
    BinaryOperation(BinaryOperation),
    Conditional(Conditional),
    Quantifier(Quantifier),
    FunctionApplication(FunctionApplication),
    Hole(syn::Ident),
}

impl quote::ToTokens for Expression {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Expression::Variable(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::Variable(#expr)
                });
            }
            Expression::Constant(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::Constant(#expr)
                });
            }
            Expression::UnaryOperation(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::UnaryOperation(#expr)
                });
            }
            Expression::BinaryOperation(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::BinaryOperation(#expr)
                });
            }
            Expression::Conditional(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::Conditional(#expr)
                });
            }
            Expression::Quantifier(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::Quantifier(#expr)
                });
            }
            Expression::FunctionApplication(expr) => {
                tokens.extend(quote::quote! {
                    svirpti_vir::high::expression::Expression::FunctionApplication(#expr)
                });
            }
            Expression::Hole(ident) => {
                tokens.extend(quote::quote! {
                    #ident
                });
            }
        }
    }
}

fn parse_function_like(input: syn::parse::ParseStream) -> syn::Result<Expression> {
    if input.peek(kw::forall) || input.peek(kw::exists) {
        Ok(Expression::Quantifier(input.parse()?))
    } else {
        Ok(Expression::FunctionApplication(input.parse()?))
    }
}
