#[derive(Debug, Clone, PartialEq)]
pub enum PgnCommentPosition {
Before,
After,
BeforeMove,
AfterMove,
BetweenMove,
AfterAlternative,
}
