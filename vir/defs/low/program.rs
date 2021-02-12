use super::context::*;
use super::declaration::*;
use index_vec::IndexVec;

vir_include! { cfg =>
    use BasicBlock;
    use Ids;
    use BasicBlockWithSuccessors;
    derive PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize;
}

#[derive(PartialEq, Eq, Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProgramFragment {
    pub uninterpreted_sorts: Vec<UninterpretedSortDeclaration>,
    pub variables: Vec<VariableDeclaration>,
    pub functions: Vec<FunctionDeclaration>,
    pub axioms: Vec<AxiomDeclaration>,
    pub basic_blocks: IndexVec<BasicBlockId, BasicBlock>,
}

impl crate::common::cfg::Cfg for ProgramFragment {
    type BasicBlock = BasicBlock;
    fn basic_blocks(&self) -> &IndexVec<BasicBlockId, BasicBlock> {
        &self.basic_blocks
    }
}

impl std::fmt::Display for ProgramFragment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "program {{")?;
        writeln!(f, "  uninterpreted_sorts: {:?}", self.uninterpreted_sorts)?;
        writeln!(f, "  variables:")?;
        for variable in &self.variables {
            writeln!(f, "    {}: {}", variable.name, variable.sort)?;
        }
        writeln!(f, "  functions: {:?}", self.functions)?;
        writeln!(f, "  axioms: {:?}", self.axioms)?;
        writeln!(f, "  blocks:",)?;
        for (id, basic_block) in self.basic_blocks.iter_enumerated() {
            writeln!(f, "    {:?}:", id)?;
            for statement in &basic_block.statements {
                writeln!(f, "      {}", statement)?;
            }
            writeln!(f, "      successors: {:?}", basic_block.successors)?;
        }
        writeln!(f, "}}")
    }
}