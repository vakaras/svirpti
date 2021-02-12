use super::context::*;
use super::declaration::*;
use index_vec::IndexVec;

vir_include! { cfg =>
    use GuardedBasicBlock as BasicBlock;
    use Ids;
    use BasicBlockWithSuccessors;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramFragment {
    pub sorts: Vec<UninterpretedSortDeclaration>,
    pub axioms: Vec<AxiomDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub procedure: ProcedureDeclaration,
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcedureDeclaration {
    pub variables: IndexVec<VariableId, VariableDeclaration>,
    pub basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
}

index_vec::define_index_type! {
    pub struct VariableId = usize;
}

impl crate::common::cfg::Cfg for ProcedureDeclaration {
    type BasicBlock = BasicBlock;
    fn basic_blocks(&self) -> &IndexVec<BasicBlockId, BasicBlock> {
        &self.basic_blocks
    }
}

impl std::fmt::Display for ProgramFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "program {{")?;
        writeln!(f, "  sorts: {:?}", self.sorts)?;
        writeln!(f, "  functions: {:?}", self.functions)?;
        writeln!(f, "  axioms: {:?}", self.axioms)?;
        writeln!(f, "  procedure:")?;
        writeln!(f, "    variables:")?;
        for variable in &self.procedure.variables {
            writeln!(f, "      {}: {}", variable.name, variable.sort)?;
        }
        writeln!(f, "    blocks:",)?;
        for (id, basic_block) in self.procedure.basic_blocks.iter_enumerated() {
            writeln!(f, "      [{:?}] {}:", id, basic_block.label)?;
            writeln!(f, "        guard: {}", basic_block.guard)?;
            for statement in &basic_block.statements {
                writeln!(f, "        {}", statement)?;
            }
            writeln!(f, "        successors: {:?}", basic_block.successors)?;
        }
        writeln!(f, "}}")
    }
}