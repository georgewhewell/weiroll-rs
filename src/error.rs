use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum WeirollError {
    #[error("unable to plan")]
    PlanError,

    #[error("Call with value must have a value parameter")]
    MissingValue,

    // todo: panic on these?
    #[error("internal error: missing return slot")]
    MissingReturnSlot,

    #[error("internal error: invalid return slot")]
    InvalidReturnSlot,

    #[error("internal error: missing literal value")]
    MissingLiteralValue,

    #[error("internal error: missing subplan")]
    MissingSubplan,

    #[error("argument count mismatch")]
    ArgumentCountMismatch,

    #[error("Subplans can only take one planner argument")]
    MultipleSubplans,

    #[error("Subplans can only take one state argument")]
    MultipleState,

    #[error("Subplans must take planner and state arguments")]
    MissingStateOrSubplan,
}
