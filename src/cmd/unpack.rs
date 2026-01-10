pub struct UnpackArgs {
    pub archive: String,
    pub dir: String,
    pub verbose: bool,
}

pub fn run(argv: Vec<String>) -> Result<(), String> {
    if argv.is_empty() || argv.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return Ok(());
    }

    let args = parse_args(argv)?;
    unpack(args)
}

fn parse_args(argv: Vec<String>) -> Result<UnpackArgs, String> {
    let mut archive: Option<String> = None;
    let mut dir = ".".to_string();
    let mut verbose = false;

    let mut it = argv.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-d" | "--dir" => {
                dir = it.next().ok_or("Missing value for -d/--dir")?;
            }
            "-v" | "--verbose" => verbose = true,
            "-o" | "--output" => {
                return Err("Option -o/--output is not supported for unpack (use -d/--dir)".into());
            }
            s if s.starts_with('-') => return Err(format!("Unknown option: {s}")),
            s => {
                if archive.is_none() {
                    archive = Some(s.to_string());
                } else {
                    return Err(format!("Unexpected argument: {s}"));
                }
            }
        }
    }

    let archive = archive.ok_or("Missing <archive>")?;
    Ok(UnpackArgs {
        archive,
        dir,
        verbose,
    })
}

fn unpack(args: UnpackArgs) -> Result<(), String> {
    if args.verbose {
        eprintln!("Unpacking: archive={} dir={}", args.archive, args.dir);
    } else {
        println!("Unpacking {}", args.archive);
    }
    Ok(())
}

fn print_help() {
    eprintln!(
        "\
USAGE:
  packrs unpack <archive> [OPTIONS]

OPTIONS:
  -d, --dir <path>      Destination directory (default: .)
  -v, --verbose         Verbose output
  -h, --help            Print help
"
    );
}
