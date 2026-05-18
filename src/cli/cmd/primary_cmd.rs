

pub enum PrimaryCmd {
    Help,
    Run,
}

pub fn primary_cmd_as_str(cmd: PrimaryCmd) -> String {
    match cmd {
        PrimaryCmd::Help => {
            return "help".to_string();
        },
        PrimaryCmd::Run => {
            return "run".to_string();
        }
    }
}