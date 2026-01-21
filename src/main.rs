mod extractor;
mod xxtea;

use extractor::Extractor;
use std::env;
use std::fs::File;
use std::io::Write;

struct Args {
    input: String,
    output: String,
}

impl Args {
    fn parse() -> Self {
        let args: Vec<String> = env::args().collect();
        // Check for help
        if args.iter().any(|arg| arg == "--help" || arg == "-h") {
            println!("Usage: byd-metadata-dumper [INPUT] [OUTPUT]");
            println!("\nArguments:");
            println!("  [INPUT]   Input PE file path (default: GameAssembly.dll)");
            println!("  [OUTPUT]  Output decrypted file path (default: global-metadata.dat)");
            std::process::exit(0);
        }

        let input = args
            .get(1)
            .cloned()
            .unwrap_or_else(|| "GameAssembly.dll".to_string());
        let output = args
            .get(2)
            .cloned()
            .unwrap_or_else(|| "global-metadata.dat".to_string());

        Args { input, output }
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    println!("  Preparing...");

    let mut extractor = Extractor::new(&args.input);

    if let Err(e) = extractor.process() {
        eprintln!("✗ Error: {}", e);
        std::process::exit(1);
    }

    let data = extractor.get_valid_data();
    println!("✓ Successfully extracted encrypted data");
    println!("  Length: {}", data.len());

    println!("  Decrypting...");

    let key = {
        let mut k = vec![0u8; 16];
        k[0] = b'E';
        k[1] = b'8';
        k[2] = b'F';
        k[3] = b'F';
        k
    };

    let start_time = std::time::Instant::now();
    match xxtea::decrypt(data, &key) {
        Ok(decrypted) => {
            let duration = start_time.elapsed();
            println!("✓ Successfully decrypted data");
            println!("  Decryption time: {:.3} seconds", duration.as_secs_f64());

            let magic = if decrypted.len() >= 4 {
                format!("{:?}", &decrypted[..4])
            } else {
                format!("{:?}", decrypted)
            };
            println!("  Header magic: {}\n", magic);

            let mut out_file = File::create(&args.output)
                .map_err(|e| format!("Error creating output file: {}", e))?;
            out_file
                .write_all(&decrypted)
                .map_err(|e| format!("Error writing output file: {}", e))?;

            println!("✓ Successfully saved decrypted data to {}", args.output);
            println!("  File size: {}\n", decrypted.len());
        }
        Err(e) => {
            eprintln!("✗ Error: {}", e);
            std::process::exit(1);
        }
    }

    Ok(())
}
