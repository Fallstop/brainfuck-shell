use crate::prelude::*;
use nu_engine::WholeStreamCommand;
use nu_errors::ShellError;
use nu_protocol::{ReturnSuccess, Signature, UntaggedValue};

pub struct Command;

impl WholeStreamCommand for Command {
    fn name(&self) -> &str {
        "random"
    }

    fn signature(&self) -> Signature {
        Signature::build("random")
    }

    fn usage(&self) -> &str {
        "Generate random values."
    }

    fn run_with_actions(&self, args: CommandArgs) -> Result<ActionStream, ShellError> {
        Ok(ActionStream::one(Ok(ReturnSuccess::Value(
            UntaggedValue::string(get_full_help(&Command, args.scope())).into_value(Tag::unknown()),
        ))))
    }
}
