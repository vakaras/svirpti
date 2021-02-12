use super::{SatResult, SmtSolverError, SmtSolverResult};
use rsmt2::{
    parse::{IdentParser, ModelParser},
    SmtConf, SmtRes, Solver,
};
use std::str::FromStr;
use svirpti_vir::smt as ast;

#[derive(Clone, Copy)]
struct Parser {}

/// A hidden type that ensures that our implementation is unique enough to not
/// conflict with the default one.
#[derive(Clone, Copy, Debug)]
struct ParserToken;

impl<'a, 'b, C: ast::Context>
    IdentParser<ast::IdentSymbol, ast::Sort, (ParserToken, &'b C), &'a str> for Parser
{
    fn parse_ident(
        self,
        input: &'a str,
        (_, context): (ParserToken, &'b C),
    ) -> SmtRes<ast::IdentSymbol> {
        context.resolve_ident(input)
    }
    fn parse_type(self, input: &'a str, (_, context): (ParserToken, &'b C)) -> SmtRes<ast::Sort> {
        match input {
            "Bool" => Ok(ast::Sort::Bool),
            "Int" => Ok(ast::Sort::Int),
            "Real" => Ok(ast::Sort::Real),
            name => Ok(ast::Sort::Uninterpreted {
                name: context.resolve_ident(name)?,
            }),
        }
    }
}

impl<'a, Br: ::std::io::BufRead>
    ModelParser<ast::IdentSymbol, ast::Sort, ast::Value, &'a mut rsmt2::parse::SmtParser<Br>>
    for Parser
{
    fn parse_value(
        self,
        parser: &'a mut rsmt2::parse::SmtParser<Br>,
        _name: &ast::IdentSymbol,
        _args: &[(ast::IdentSymbol, ast::Sort)],
        out_sort: &ast::Sort,
    ) -> SmtRes<ast::Value> {
        match out_sort {
            ast::Sort::Bool => parser.bool().map(ast::Value::Bool),
            ast::Sort::Int => {
                let value = parser.try_int(|input, positive| {
                    i64::from_str(input).map(|num| if positive { num } else { -num })
                })?;
                if let Some(number) = value {
                    Ok(ast::Value::Int(number))
                } else {
                    parser.fail_with("expected integer")
                }
            }
            x => unimplemented!("{:?}", x),
        }
    }
}

pub struct Configuration {
    smt_conf: SmtConf,
    /// Attributes fed into solver's `set_info` method.
    attributes: Vec<String>,
    /// Options fed into solver's `set_option` method.
    options: Vec<(String, String)>,
    tee_path: Option<String>,
}

impl Configuration {
    pub fn new(
        smt_conf: SmtConf,
        attributes: Vec<String>,
        options: Vec<(String, String)>,
        tee_path: Option<String>,
    ) -> Self {
        Self {
            smt_conf,
            attributes,
            options,
            tee_path,
        }
    }
}

impl Default for Configuration {
    fn default() -> Self {
        let smt_conf = SmtConf::z3(get_z3_path());
        let attributes = vec![(":smt-lib-version 2.0")]
            .into_iter()
            .map(|attribute| (attribute.into()))
            .collect();
        let options = vec![
            (":AUTO_CONFIG", "false"),
            (":smt.MBQI", "false"),
            (":TYPE_CHECK", "true"),
        ]
        .into_iter()
        .map(|(option, value)| (option.into(), value.into()))
        .collect();
        // let tee_path = Some("/tmp/test.smt2".into());
        let tee_path = None;
        Self {
            smt_conf,
            options,
            attributes,
            tee_path,
        }
    }
}

// TODO: Rename to not mention Z3 in its name.
pub struct Z3SmtSolver {
    solver: Solver<Parser>,
}

impl std::convert::From<rsmt2::errors::Error> for SmtSolverError {
    fn from(error: rsmt2::errors::Error) -> Self {
        Self::GenericSolverError {
            source: Box::new(error),
        }
    }
}

impl Z3SmtSolver {
    pub fn new(conf: Configuration) -> SmtSolverResult<Self> {
        let parser = Parser {};
        let mut solver = Solver::new(conf.smt_conf, parser)?;
        if let Some(tee_path) = conf.tee_path {
            solver.path_tee(tee_path).unwrap();
        }
        for attribute in &conf.attributes {
            solver.set_info(attribute)?;
        }
        for (option, value) in &conf.options {
            solver.set_option(option, value)?;
        }
        Ok(Self { solver })
    }
    /// We cannot use the `Default` trait because this is potentially failing
    /// operation.
    pub fn default() -> SmtSolverResult<Self> {
        Self::new(Default::default())
    }
}

fn get_z3_path() -> String {
    std::env::var("Z3_EXE").unwrap_or_else(|_err| "z3".into())
}

impl super::SmtSolver for Z3SmtSolver {
    // type SmtContext = C;
    type Conf = Configuration;
    type Error = rsmt2::errors::Error;
    fn new(conf: Configuration) -> SmtSolverResult<Self> {
        Self::new(conf)
    }
    /// We cannot use the `Default` trait because this is potentially failing
    /// operation.
    fn default() -> SmtSolverResult<Self> {
        Self::default()
    }
    fn declare_sort(
        &mut self,
        sort: &ast::UninterpretedSortDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult {
        self.solver.declare_sort_with(sort, 0, context)?;
        Ok(())
    }
    fn declare_function(
        &mut self,
        function: &ast::FunctionDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult {
        self.solver.declare_fun_with::<_, _, ast::Sort, _, _>(
            &function.name,
            &function
                .parameters
                .iter()
                .map(|parameter| parameter.sort.clone())
                .collect::<Vec<_>>(),
            &function.return_sort,
            context,
        )?;
        Ok(())
    }
    fn declare_label(
        &mut self,
        label: &ast::LabelDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult {
        self.solver.declare_fun_with::<_, _, ast::Sort, _, _>(
            &label.name,
            &[],
            &ast::Sort::Bool,
            context,
        )?;
        Ok(())
    }
    fn declare_variable(
        &mut self,
        variable: &ast::VariableDeclaration,
        context: &impl ast::Context,
    ) -> SmtSolverResult {
        self.solver.declare_fun_with::<_, _, ast::Sort, _, _>(
            &variable.name,
            &[],
            &variable.sort,
            context,
        )?;
        Ok(())
    }
    fn push(&mut self) -> SmtSolverResult {
        self.solver.push(1)?;
        Ok(())
    }
    fn pop(&mut self) -> SmtSolverResult {
        self.solver.pop(1)?;
        Ok(())
    }
    fn assert(
        &mut self,
        assertion: &ast::Expression,
        context: &impl ast::Context,
    ) -> SmtSolverResult {
        self.solver.assert_with(assertion, context)?;
        Ok(())
    }
    fn check_sat(&mut self) -> SmtSolverResult<SatResult> {
        let result = match self.solver.check_sat_or_unk()? {
            Some(true) => SatResult::Sat,
            Some(false) => SatResult::Unsat,
            None => SatResult::Unknown,
        };
        Ok(result)
    }
    fn get_labels(
        &mut self,
        context: &impl ast::Context,
    ) -> SmtSolverResult<Vec<ast::LabelSymbol>> {
        let labels = self.solver.labels((ParserToken, context))?; //.into_iter().map(|(_, label)| label).collect();
        Ok(labels)
    }
    fn get_model(&mut self, context: &impl ast::Context) -> SmtSolverResult<ast::Model> {
        let mut model = ast::Model { items: Vec::new() };
        for (name, args, sort, value) in self.solver.get_model_with((ParserToken, context))? {
            model.items.push(ast::ModelItem {
                name,
                args: args
                    .into_iter()
                    .map(|(name, sort)| ast::ModelItemArg { name, sort })
                    .collect(),
                sort,
                value,
            })
        }
        Ok(model)
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::{ast, Z3SmtSolver};
    use crate::solvers::SmtSolver;
    use svirpti_vir::smt::{
        BinaryOperationHelpers, FunctionApplicationHelpers, LabelledExpressionHelpers,
        QuantifierHelpers, UnaryOperationHelpers, VariableHelpers,
    };

    #[derive(Default)]
    struct StringContext {
        variables: HashMap<ast::VariableSymbol, ast::VariableDeclaration>,
        functions: HashMap<ast::FunctionSymbol, ast::FunctionDeclaration>,
    }
    impl ast::Context for StringContext {
        fn write_uninterpreted_sort_name<Writer: std::io::Write>(
            &self,
            writer: &mut Writer,
            symbol: &ast::UninterpretedSortSymbol,
        ) -> rsmt2::SmtRes<()> {
            write!(writer, "{}", symbol)?;
            Ok(())
        }
        fn write_variable_symbol<Writer: std::io::Write>(
            &self,
            writer: &mut Writer,
            symbol: &ast::VariableSymbol,
        ) -> rsmt2::SmtRes<()> {
            write!(writer, "{}", symbol)?;
            Ok(())
        }
        fn write_label_symbol<Writer: std::io::Write>(
            &self,
            writer: &mut Writer,
            symbol: &ast::VariableSymbol,
        ) -> rsmt2::SmtRes<()> {
            write!(writer, "{}", symbol)?;
            Ok(())
        }
        fn get_variable_sort<'a>(&'a self, variable: &'a ast::VariableSymbol) -> &'a ast::Sort {
            &self.variables[variable].sort
        }
        fn get_function_sort<'a>(&'a self, function: &'a ast::FunctionSymbol) -> &'a ast::Sort {
            &self.functions[function].return_sort
        }
        fn resolve_ident(&self, ident: &str) -> rsmt2::SmtRes<ast::IdentSymbol> {
            Ok(ident.into())
        }
    }
    #[test]
    fn check_z3_installation() {
        let mut context = StringContext::default();
        let mut z3 = Z3SmtSolver::default().unwrap();
        let x = ast::VariableDeclaration {
            name: "x".into(),
            sort: ast::Sort::Int,
        };
        z3.declare_variable(&x, &context).unwrap();
        context.variables.insert(x.name.clone(), x);
        z3.assert(
            &ast::Expression::equals(
                ast::Expression::add(ast::Expression::variable("x".into()), 2.into()),
                4.into(),
            ),
            &context,
        )
        .unwrap();

        assert_eq!(z3.check_sat().unwrap(), crate::solvers::SatResult::Sat);

        let model = z3.get_model(&context).unwrap();
        let labels = z3.get_labels(&context).unwrap();
        insta::assert_yaml_snapshot!((model, labels));
    }
    #[test]
    fn check_quantifiers() {
        let mut context = StringContext::default();
        let mut z3: Z3SmtSolver = Z3SmtSolver::default().unwrap();
        z3.declare_sort(
            &ast::UninterpretedSortDeclaration { name: "Nat".into() },
            &context,
        )
        .unwrap();
        let nat = ast::Sort::Uninterpreted { name: "Nat".into() };
        let zero = ast::FunctionDeclaration {
            name: "zero".into(),
            parameters: vec![],
            return_sort: nat.clone(),
        };
        z3.declare_function(&zero, &context).unwrap();
        context.functions.insert("zero".into(), zero);

        let succ = ast::FunctionDeclaration {
            name: "succ".into(),
            parameters: vec![ast::VariableDeclaration {
                name: "num".into(),
                sort: nat.clone(),
            }],
            return_sort: nat.clone(),
        };
        z3.declare_function(&succ, &context).unwrap();
        context.functions.insert("succ".into(), succ);

        let count = ast::FunctionDeclaration {
            name: "count".into(),
            parameters: vec![ast::VariableDeclaration {
                name: "num".into(),
                sort: nat.clone(),
            }],
            return_sort: ast::Sort::Int,
        };
        z3.declare_function(&count, &context).unwrap();
        context.functions.insert("count".into(), count);

        z3.push().unwrap();

        // count(zero()) == 0
        z3.assert(
            &ast::Expression::equals(
                ast::Expression::call(
                    "count".into(),
                    vec![ast::Expression::call("zero".into(), vec![])],
                ),
                0.into(),
            ),
            &context,
        )
        .unwrap();

        // forall n: Nat :: {count(succ(n))} count(succ(n)) == count(n) + 1
        z3.assert(
            &ast::Expression::forall(
                vec![ast::BoundedVariableDecl {
                    name: "n".into(),
                    sort: nat,
                }],
                vec![ast::Trigger {
                    parts: vec![ast::Expression::call(
                        "count".into(),
                        vec![ast::Expression::call(
                            "succ".into(),
                            vec![ast::Expression::variable("n".into())],
                        )],
                    )],
                }],
                ast::Expression::equals(
                    ast::Expression::call(
                        "count".into(),
                        vec![ast::Expression::call(
                            "succ".into(),
                            vec![ast::Expression::variable("n".into())],
                        )],
                    ),
                    ast::Expression::add(
                        ast::Expression::call(
                            "count".into(),
                            vec![ast::Expression::variable("n".into())],
                        ),
                        1.into(),
                    ),
                ),
            ),
            &context,
        )
        .unwrap();

        // !(count(zero()) == 0)
        z3.push().unwrap();
        z3.assert(
            &ast::Expression::not(ast::Expression::equals(
                ast::Expression::call(
                    "count".into(),
                    vec![ast::Expression::call("zero".into(), vec![])],
                ),
                0.into(),
            )),
            &context,
        )
        .unwrap();
        assert_eq!(z3.check_sat().unwrap(), crate::solvers::SatResult::Unsat);
        z3.pop().unwrap();

        // !(count(succ(zero())) == 1)
        z3.assert(
            &ast::Expression::not(ast::Expression::equals(
                ast::Expression::call(
                    "count".into(),
                    vec![ast::Expression::call(
                        "succ".into(),
                        vec![ast::Expression::call("zero".into(), vec![])],
                    )],
                ),
                1.into(),
            )),
            &context,
        )
        .unwrap();

        assert_eq!(z3.check_sat().unwrap(), crate::solvers::SatResult::Unsat);
    }
    #[test]
    fn check_z3_labels() {
        let mut context = StringContext::default();
        let mut z3: Z3SmtSolver = Z3SmtSolver::default().unwrap();
        let k = ast::VariableDeclaration {
            name: "k".into(),
            sort: ast::Sort::Int,
        };
        z3.declare_variable(&k, &context).unwrap();
        context.variables.insert(k.name.clone(), k);
        let array_access_1 = ast::LabelDeclaration {
            name: "ArrayAccess1".into(),
        };
        let array_access_2 = ast::LabelDeclaration {
            name: "ArrayAccess2".into(),
        };
        z3.declare_label(&array_access_1, &context).unwrap();
        z3.declare_label(&array_access_2, &context).unwrap();
        let expr = ast::Expression::not(ast::Expression::and(
            ast::Expression::implies(
                ast::Expression::and(
                    ast::Expression::not(
                        // (k < 10)
                        ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            10.into(),
                        ),
                    ),
                    // k < 20
                    ast::Expression::less_than(ast::Expression::variable("k".into()), 20.into()),
                ),
                ast::Expression::label_negative(
                    "ArrayAccess1".into(),
                    ast::Expression::and(
                        // !(k < 0)
                        ast::Expression::not(ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            100.into(),
                        )),
                        // k < 100
                        ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            100.into(),
                        ),
                    ),
                ),
            ),
            ast::Expression::implies(
                ast::Expression::not(ast::Expression::and(
                    ast::Expression::not(
                        // (k < 10)
                        ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            10.into(),
                        ),
                    ),
                    // k < 20
                    ast::Expression::less_than(ast::Expression::variable("k".into()), 20.into()),
                )),
                ast::Expression::label_negative(
                    "ArrayAccess2".into(),
                    ast::Expression::and(
                        // !(k < 0)
                        ast::Expression::not(ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            100.into(),
                        )),
                        // k < 100
                        ast::Expression::less_than(
                            ast::Expression::variable("k".into()),
                            100.into(),
                        ),
                    ),
                ),
            ),
        ));
        insta::assert_display_snapshot!(expr);
        z3.push().unwrap();
        z3.assert(&expr, &context).unwrap();

        assert_eq!(z3.check_sat().unwrap(), crate::solvers::SatResult::Sat);
        let model = z3.get_model(&context).unwrap();
        let labels = z3.get_labels(&context).unwrap();
        insta::assert_yaml_snapshot!((model, labels));
    }
}
