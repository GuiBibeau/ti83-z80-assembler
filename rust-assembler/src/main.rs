use anyhow::Result;
use clap::Parser as ClapParser;
use std::fs;
use std::path::PathBuf;

use z80asm::{TI8XPGenerator, Z80Assembler};

#[derive(ClapParser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Input assembly file
    input: PathBuf,

    /// Output .8xp file (defaults to input name with .8xp extension)
    output: Option<PathBuf>,

    /// Program name (defaults to input filename, max 8 chars)
    #[arg(short, long)]
    name: Option<String>,
}

fn main() -> Result<()> {
    let args = Args::parse();

    // Determine output file
    let output_file = args.output.unwrap_or_else(|| {
        let mut output = args.input.clone();
        output.set_extension("8xp");
        output
    });

    // Determine program name
    // TI-83 Plus limitations: 8 chars max, no underscores, uppercase only
    let program_name = args.name.unwrap_or_else(|| {
        args.input
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("PROGRAM")
            .to_uppercase()
            .replace("_", "")  // Remove underscores
            .replace("-", "")  // Remove hyphens
            .chars()
            .filter(|c| c.is_ascii_alphanumeric())  // Only allow A-Z and 0-9
            .take(8)  // Max 8 characters
            .collect()
    });

    // Read source file
    let source = fs::read_to_string(&args.input)?;

    // Create assembler instance
    let mut assembler = Z80Assembler::new();

    // Add TI-83 Plus header if not present
    let processed_source = if !source.contains(".org") {
        format!(".org $9D93\n.db $BB,$6D\n{}", source)
    } else {
        source.clone()
    };

    // Assemble the code
    let code = assembler.assemble(&processed_source)?;
    println!("✓ Assembled {} bytes", code.len());

    // Generate .8xp file
    let output = TI8XPGenerator::create_8xp(&program_name, &code);
    let output_size = output.len();

    // Write output file
    fs::write(&output_file, output)?;

    println!(
        "✓ Created {} ({} bytes)",
        output_file.display(),
        output_size
    );
    println!("✓ Program name: {}", program_name);
    println!("\nTo test:");
    println!("1. Visit https://www.cemetech.net/projects/jstified/");
    println!("2. Drag {} onto the calculator", output_file.display());
    println!("3. Run with: Asm(prgm{})", program_name);

    Ok(())
}
