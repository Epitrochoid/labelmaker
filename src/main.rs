#[derive(Debug)]
struct LabelmakerArgs {
    command: std::path::PathBuf
}

fn main() {
    let args = match parse_args() {
        Ok(v) => v,
        Err(e) => {
            eprintln!("Error: {}.", e);
            std::process::exit(1);
        }
    };
    println!("{:?}", args);
}

fn parse_args() -> Result<LabelmakerArgs, pico_args::Error> {
    let mut raw_args = pico_args::Arguments::from_env();

    let args = LabelmakerArgs {
        command: raw_args.value_from_os_str("--command", parse_path)?
    };

    Ok(args)
}

fn parse_path(s: &std::ffi::OsStr) -> Result<std::path::PathBuf, &'static str> {
    Ok(s.into())
}
