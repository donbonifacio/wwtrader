use actions::error::ActionError;

pub type ActionResult<T> = std::result::Result<T, ActionError>;
