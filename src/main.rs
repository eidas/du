mod args;
mod format;
mod traversal;

use clap::Parser;

use args::Args;
use format::format_size;

fn main() {
    let args = Args::parse();
    let path = &args.path;

    if !path.exists() {
        eprintln!("du: {}: No such file or directory", path.display());
        std::process::exit(1);
    }

    if path.is_file() {
        let size = path.metadata().map(|m| m.len()).unwrap_or(0);
        println!("{}\t{}", format_size(size, args.human_readable), path.display());
        return;
    }

    let max_depth = if args.summarize { Some(0) } else { args.depth };

    let (entries, root_size) = traversal::compute_sizes(path, max_depth);

    if !args.summarize {
        for entry in &entries {
            println!(
                "{}\t{}",
                format_size(entry.size_bytes, args.human_readable),
                entry.path.display()
            );
        }
    }

    println!("{}\t{}", format_size(root_size, args.human_readable), path.display());
}
