//--------------------------------------------------------------------
// Description:
// Date: xxxx-xx-xx
//--------------------------------------------------------------------
// Notes:
// FIXME:
//--------------------------------------------------------------------

use crate::levels::LevelContainer;
use anyhow::Result;
use std::env;
use std::process::exit;

mod levels;

fn test(value: &str) -> Result<()> {
    println!("INFO: value: {:?}", value);

    // TryFrom expects a simple string
    let level = levels::LevelContainer::try_from(value);

    println!("INFO: TryFrom slog log level: {:?}", level);

    // But the serde deserializer expects a quoted (JSON) value, so let's
    // quote it!
    let quoted_value = format!("{:?}", value);
    let level_container: LevelContainer = serde_json::from_str(&quoted_value)?;

    println!("INFO: LevelContainer slog log level: {:?}", level_container);

    Ok(())
}

fn real_main() -> Result<()> {
    let args: Vec<String> = env::args().collect();

    let program_name = &args[0];

    if args.len() < 2 {
        println!("ERROR: {}: specify slog log level", program_name);
        println!("Hint: Try running as follows:");
        println!("");
        println!("$ {} 'critical'", program_name);
        exit(1);
    }

    let value = &args[1];

    test(value)
}

fn main() {
    if let Err(e) = real_main() {
        eprintln!("ERROR: {:#}", e);
        exit(1);
    }
}
