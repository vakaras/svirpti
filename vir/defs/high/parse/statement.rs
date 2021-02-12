use super::expression::Expression;
use super::expression::Variable;

vir_include! { statement::parse_ast =>
    use Assert;
    use Assume;
    use Havoc;
    use Assign;
    derive PartialEq, Eq, Debug, Clone;
}
vir_include! { statement::parse =>
    use kw;
    use Assert;
    use Assume;
    use Havoc;
    use Assign;
}

pub enum Statement {
    Assume(Assume),
    Assert(Assert),
    Havoc(Havoc),
    Assign(Assign),
    Hole(syn::Ident),
}

impl syn::parse::Parse for Statement {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let lookahead = input.lookahead1();
        if lookahead.peek(kw::assume) {
            Ok(Statement::Assume(input.parse()?))
        } else if lookahead.peek(kw::assert) {
            Ok(Statement::Assert(input.parse()?))
        } else if lookahead.peek(kw::havoc) {
            Ok(Statement::Havoc(input.parse()?))
        } else if lookahead.peek(kw::assign) {
            Ok(Statement::Assign(input.parse()?))
        } else if lookahead.peek(syn::Token![#]) {
            input.parse::<syn::Token![#]>()?;
            Ok(Statement::Hole(input.parse()?))
        } else {
            Err(lookahead.error())
        }
    }
}

impl quote::ToTokens for Statement {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Statement::Assume(statement) => tokens.extend(quote::quote! {
                Statement::Assume(#statement)
            }),
            Statement::Assert(statement) => tokens.extend(quote::quote! {
                Statement::Assert(#statement)
            }),
            Statement::Havoc(statement) => tokens.extend(quote::quote! {
                Statement::Havoc(#statement)
            }),
            Statement::Assign(statement) => tokens.extend(quote::quote! {
                Statement::Assign(#statement)
            }),
            Statement::Hole(ident) => tokens.extend(quote::quote! {
                #ident
            }),
        }
    }
}
