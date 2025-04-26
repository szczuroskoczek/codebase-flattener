use clap::{ArgAction, Parser};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

/// Flattens a codebase into one file for LLM ingestion.
#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Extra ignore files to search for (like `.myignore`)
    #[arg(short, long, action = ArgAction::Append)]
    ignore: Vec<String>,

    /// Disable default ignores (`.gitignore` + `.llmignore`)
    #[arg(long, action = ArgAction::SetTrue)]
    no_default_ignores: bool,

    /// Don’t follow symlinks
    #[arg(short, long, action = ArgAction::SetTrue)]
    no_follow: bool,

    /// Output filename (will be excluded from the dump)
    #[arg(short, long, default_value = "./codebase.txt")]
    output: String,
}

fn main() {
    let args = Args::parse();

    // Prepare the walker
    let mut builder = WalkBuilder::new("./");

    if !args.no_default_ignores {
        builder
            .add_custom_ignore_filename(".gitignore")
            .add_custom_ignore_filename(".llmignore");
    } else {
        builder
            .hidden(false)
            .git_exclude(false)
            .git_global(false)
            .git_ignore(false);
    }
    if !args.no_follow {
        builder.follow_links(true);
    }
    for ig in &args.ignore {
        builder.add_custom_ignore_filename(ig);
    }
    let walker = builder.build();

    // Open the output file
    let out_path = Path::new(&args.output);
    let file = File::create(&out_path).unwrap_or_else(|e| {
        eprintln!("Failed to create {}: {}", args.output, e);
        std::process::exit(1);
    });
    let mut writer = BufWriter::new(file);

    // Dump each file, skipping the output itself
    for result in walker {
        match result {
            Ok(entry) if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) => {
                let path = entry.path();
                if let Ok(text) = fs::read_to_string(path) {
                    // Separator
                    writeln!(writer, "<<< FILE: {} >>>", path.display()).unwrap();
                    // Contents
                    writer.write_all(text.as_bytes()).unwrap();
                    writeln!(writer).unwrap();
                    writeln!(writer).unwrap();
                }
            }
            Ok(_) => {}
            Err(err) => eprintln!("Walk error: {}", err),
        }
    }
}
