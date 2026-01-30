use std::fs;

use crate::vlc::encode::encode;

pub struct PackArgs {
    pub input: String,
    pub output: Option<String>,
    pub verbose: bool,
}

pub fn run(argv: Vec<String>) -> Result<(), String> {
    if argv.is_empty() || argv.iter().any(|a| a == "-h" || a == "--help") {
        print_help();
        return Ok(());
    }

    let args = parse_args(argv)?;
    pack(args)
}

fn parse_args(argv: Vec<String>) -> Result<PackArgs, String> {
    let mut input: Option<String> = None;
    let mut output: Option<String> = None;
    let mut verbose = false;

    let mut it = argv.into_iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "-o" | "--output" => {
                let v = it.next().ok_or("Missing value for -o/--output")?;
                output = Some(v);
            }
            "-v" | "--verbose" => verbose = true,
            s if s.starts_with('-') => return Err(format!("Unknown option: {s}")),
            s => {
                if input.is_none() {
                    input = Some(s.to_string());
                } else {
                    return Err(format!("Unexpected argument: {s}"));
                }
            }
        }
    }

    let input = input.ok_or("Missing <input> argument")?;
    Ok(PackArgs {
        input,
        output,
        verbose,
    })
}

fn pack(args: PackArgs) -> Result<(), String> {
    let file_path = args.input;
    let output_path = args.output.ok_or("Missing -o/--output argument")?;

    let content = fs::read_to_string(&file_path)
        .map_err(|e| format!("failed to read file '{}': {}", file_path, e))?;

    let encoded = encode(content);

    fs::write(&output_path, &encoded)
        .map_err(|e| format!("failed to write file '{}': {}", output_path, e))?;

    if args.verbose {
        println!("Packed '{}' -> '{}' ({} bytes)", file_path, output_path, encoded.len());
    }

    Ok(())
}

fn print_help() {
    eprintln!(
        "\
USAGE:
  packrs pack <input> [OPTIONS]

OPTIONS:
  -o, --output <file>   Output archive path
  -v, --verbose         Verbose output
  -h, --help            Print help
"
    );
}
