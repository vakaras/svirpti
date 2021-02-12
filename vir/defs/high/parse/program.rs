use super::declaration::*;
use super::statement::Statement;

vir_include! { cfg::parse =>
    use GuardedBasicBlock;
}

pub mod kw2 {
    syn::custom_keyword!(procedure);
    syn::custom_keyword!(locals);
}

pub struct ProgramFragment {
    pub sorts: Vec<UninterpretedSortDeclaration>,
    pub axioms: Vec<AxiomDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub procedure: ProcedureDeclaration,
}

pub struct ProcedureDeclaration {
    pub variables: Vec<VariableDeclaration>,
    pub basic_blocks: Vec<BasicBlock>,
}

impl syn::parse::Parse for ProgramFragment {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut sorts = Vec::new();
        let mut axioms = Vec::new();
        let mut functions = Vec::new();
        let mut procedure = None;
        while !input.is_empty() {
            let lookahead = input.lookahead1();
            if lookahead.peek(kw::sort) {
                sorts.push(input.parse()?);
            } else if lookahead.peek(kw::axiom) {
                axioms.push(input.parse()?);
            } else if lookahead.peek(syn::Token![fn]) {
                functions.push(input.parse()?);
            } else if lookahead.peek(kw2::procedure) {
                if procedure.is_some() {
                    return Err(syn::Error::new(
                        input.span(),
                        "exactly one procedure must be defined",
                    ));
                }
                procedure = Some(input.parse()?);
            } else {
                return Err(lookahead.error());
            }
        }
        let procedure = if let Some(procedure) = procedure {
            procedure
        } else {
            return Err(syn::Error::new(input.span(), "missing procedure"));
        };
        Ok(Self {
            sorts,
            axioms,
            functions,
            procedure,
        })
    }
}

impl syn::parse::Parse for ProcedureDeclaration {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        input.parse::<kw2::procedure>()?;
        let procedure_content;
        syn::braced!(procedure_content in input);
        procedure_content.parse::<kw2::locals>()?;
        let variable_decls;
        syn::braced!(variable_decls in procedure_content);
        let parsed_variables: syn::punctuated::Punctuated<VariableDeclaration, syn::Token![,]> =
            syn::punctuated::Punctuated::parse_terminated(&variable_decls)?;
        let variables = parsed_variables.into_iter().collect();
        let mut basic_block_map = std::collections::HashMap::new();
        let mut block_order = Vec::new();
        while !procedure_content.is_empty() {
            let block: BasicBlock = procedure_content.parse()?;
            let label = block.label.clone();
            block_order.push(label.clone());
            if basic_block_map.contains_key(&label) {
                return Err(syn::Error::new(label.span(), "duplicate label"));
            }
            basic_block_map.insert(label, block);
        }
        let mut label_indices: std::collections::HashMap<_, _> = block_order
            .iter()
            .enumerate()
            // We need to shift all indices by 1 because the first is reserved
            // for the sentinel entry block.
            .map(|(index, label)| (label.clone(), index + 1))
            .collect();
        label_indices.insert(
            syn::Ident::new("exit", proc_macro2::Span::call_site()),
            basic_block_map.len() + 1,
        );
        let mut basic_blocks = Vec::new();
        for label in &block_order {
            let mut block: BasicBlock = basic_block_map.remove(label).unwrap();
            for label in &block.successor_labels {
                if let Some(index) = label_indices.get(label) {
                    block.successors.push(*index);
                } else {
                    return Err(syn::Error::new(label.span(), "no block with this label"));
                }
            }
            basic_blocks.push(block);
        }
        Ok(Self {
            variables,
            basic_blocks,
        })
    }
}

impl quote::ToTokens for ProgramFragment {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut sort_decls = proc_macro2::TokenStream::new();
        for sort in &self.sorts {
            sort_decls.extend(quote::quote! { #sort, });
        }
        let mut axiom_decls = proc_macro2::TokenStream::new();
        for axiom in &self.axioms {
            axiom_decls.extend(quote::quote! { #axiom, });
        }
        let mut function_decls = proc_macro2::TokenStream::new();
        for function in &self.functions {
            function_decls.extend(quote::quote! { #function, });
        }
        let procedure = &self.procedure;
        tokens.extend(quote::quote! {
            {
                let sorts = vec![#sort_decls];
                let axioms = vec![#axiom_decls];
                let functions = vec![#function_decls];
                let procedure = #procedure;
                svirpti_vir::high::program::ProgramFragment {
                    sorts,
                    axioms,
                    functions,
                    procedure,
                }
            }
        });
    }
}

impl quote::ToTokens for ProcedureDeclaration {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let mut variable_tokens = proc_macro2::TokenStream::new();
        for variable in &self.variables {
            variable_tokens.extend(quote::quote! {
                #variable,
            });
        }
        let mut basic_block_tokens = proc_macro2::TokenStream::new();
        basic_block_tokens.extend(quote::quote! {
            svirpti_vir::high::program::BasicBlock {
                label: "entry".into(),
                guard: true.into(),
                statements: Vec::new().into(),
                successors: vec![1.into()],
            },
        });
        for basic_block in &self.basic_blocks {
            basic_block_tokens.extend(quote::quote! {
                #basic_block,
            });
        }
        basic_block_tokens.extend(quote::quote! {
            BasicBlock {
                label: "exit".into(),
                guard: true.into(),
                statements: Vec::new().into(),
                successors: Vec::new(),
            },
        });
        tokens.extend(quote::quote! {
            {
                ProcedureDeclaration {
                    variables: vec![#variable_tokens].into(),
                    basic_blocks: vec![#basic_block_tokens].into(),
                }
            }
        });
    }
}
