use svirpti_vir::{high, low, smt};

pub trait Context {
    fn create_versioned_variable_symbol(
        &mut self,
        name: &high::VariableSymbol,
        version: usize,
    ) -> low::VariableSymbol;
    fn lower_domain_name(
        &mut self,
        name: &high::UninterpretedSortSymbol,
    ) -> low::UninterpretedSortSymbol;
    fn lower_reference_name(&mut self, name: &high::AdtNameSymbol) -> low::UninterpretedSortSymbol;
    fn lower_label(&mut self, label: &high::LabelSymbol) -> low::LabelSymbol;
    fn convert_variable_name_to_smt(&mut self, name: &low::VariableSymbol) -> smt::VariableSymbol;
    fn convert_uninterpreted_sort_to_smt(
        &mut self,
        name: &low::UninterpretedSortSymbol,
    ) -> smt::UninterpretedSortSymbol;
    fn convert_function_name_to_smt(&mut self, name: &low::FunctionSymbol) -> smt::FunctionSymbol;
    fn convert_label_name_to_smt(&mut self, name: &low::LabelSymbol) -> smt::LabelSymbol;
    fn convert_known_label_name_to_smt(&self, name: &low::LabelSymbol) -> smt::LabelSymbol;
    fn create_label_for_basic_block(&mut self, id: low::BasicBlockId) -> smt::VariableSymbol;
    fn resolve_high_label(&self, name: &low::LabelSymbol) -> high::LabelSymbol;
    fn resolve_high_variable(&self, name: &low::VariableSymbol) -> high::VariableSymbol;
    fn resolve_low_label(&self, name: &smt::LabelSymbol) -> low::LabelSymbol;
    fn resolve_low_variable(&self, name: &smt::VariableSymbol) -> low::VariableSymbol;
}

pub struct StringContext {}

impl Context for StringContext {
    fn create_versioned_variable_symbol(
        &mut self,
        name: &high::VariableSymbol,
        version: usize,
    ) -> low::VariableSymbol {
        format!("{}@{}", name, version).into()
    }
    fn lower_domain_name(
        &mut self,
        name: &high::UninterpretedSortSymbol,
    ) -> low::UninterpretedSortSymbol {
        name.as_string().into()
    }
    fn lower_reference_name(&mut self, name: &high::AdtNameSymbol) -> low::UninterpretedSortSymbol {
        name.as_string().into()
    }
    fn lower_label(&mut self, label: &high::LabelSymbol) -> low::LabelSymbol {
        label.as_string().into()
    }
    fn convert_function_name_to_smt(&mut self, name: &low::FunctionSymbol) -> smt::FunctionSymbol {
        name.as_string().into()
    }
    fn convert_label_name_to_smt(&mut self, name: &low::LabelSymbol) -> smt::LabelSymbol {
        name.as_string().into()
    }
    fn convert_known_label_name_to_smt(&self, name: &low::LabelSymbol) -> smt::LabelSymbol {
        name.as_string().into()
    }
    fn convert_uninterpreted_sort_to_smt(
        &mut self,
        name: &low::UninterpretedSortSymbol,
    ) -> smt::UninterpretedSortSymbol {
        name.as_string().into()
    }
    fn convert_variable_name_to_smt(&mut self, name: &low::VariableSymbol) -> smt::VariableSymbol {
        name.as_string().into()
    }
    fn create_label_for_basic_block(&mut self, id: low::BasicBlockId) -> smt::VariableSymbol {
        format!("BB@{}", id.index()).into()
    }
    fn resolve_high_label(&self, name: &low::LabelSymbol) -> high::LabelSymbol {
        name.as_string().into()
    }
    fn resolve_high_variable(&self, name: &low::VariableSymbol) -> high::VariableSymbol {
        let mut s = name.as_string();
        let index = s.rfind('@').unwrap();
        s.truncate(index);
        s.into()
    }
    fn resolve_low_label(&self, name: &smt::LabelSymbol) -> low::LabelSymbol {
        name.as_string().into()
    }
    fn resolve_low_variable(&self, name: &smt::VariableSymbol) -> low::VariableSymbol {
        name.as_string().into()
    }
}
