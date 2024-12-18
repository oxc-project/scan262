use std::path::PathBuf;

use bpaf::Bpaf;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Command {
    #[bpaf(switch)]
    pub quiet: bool,

    #[bpaf(positional("PATH"), many)]
    pub paths: Vec<PathBuf>,
}
