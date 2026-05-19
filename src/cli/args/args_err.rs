


#[derive(Debug)]
pub enum ArgsErr {
    MissingArgs,
    ArgDoesNotExist(usize)
}

pub fn handle_arg_err(err: ArgsErr) {
    match err {
        ArgsErr::MissingArgs => {
            eprintln!("missing args, please run 'gt help' for available commands");
        },
        ArgsErr::ArgDoesNotExist(pos) => {
            eprintln!("arg does not exist at position: {}", pos);
        }
    }
}