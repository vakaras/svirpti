pub trait AssumeAssertHelpers {
    type Statement;
    type Expression;
    type LabelSymbol;
    fn assume(assertion: Self::Expression) -> Self::Statement;
    fn assume_with_label(assertion: Self::Expression, label: Self::LabelSymbol) -> Self::Statement;
    fn assert(assertion: Self::Expression) -> Self::Statement;
    fn assert_with_label(assertion: Self::Expression, label: Self::LabelSymbol) -> Self::Statement;
}
