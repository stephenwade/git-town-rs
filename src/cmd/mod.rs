mod cmd_helpers;
mod hack;
mod root;

pub fn execute() -> Result<(), Box<dyn std::error::Error>> {
    let cmd = root::root_cmd().subcommand(hack::hack_cmd());

    let matches = cmd.get_matches();
    match matches.subcommand() {
        Some((hack::HACK_NAME, sub_m)) => hack::execute_hack(sub_m),
        Some(_) => {
            eprintln!("Unknown subcommand");
            std::process::exit(1);
        }
        None => {
            unreachable!("root_cmd uses subcommand_required(true)")
        }
    }
}
