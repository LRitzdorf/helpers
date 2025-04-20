use clap::{Command, command};
use std::path::Path;
use std::fs::File;
use std::io::{self, prelude::*};


// NOTE: Set your "battery conservation mode" file here:
const TARGET: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";


const MODE_OFF: &[u8; 2] = b"0\n";
const MODE_ON:  &[u8; 2] = b"1\n";

fn get_mode(f: &mut File) -> io::Result<bool> {
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf)?;
    match &buf {
        MODE_OFF => Ok(false),
        MODE_ON  => Ok(true),
        _        => panic!("Unexpected conservation mode '{}'", String::from_utf8_lossy(&buf)),
    }
}

fn set_mode(f: &mut File, state: bool) -> io::Result<bool> {
    let buf = if state { MODE_ON } else { MODE_OFF };
    f.write_all(buf)?;
    Ok(state)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    include_str!("../Cargo.toml");  // Recompile when Cargo.toml changes...
    let m = command!()              // ...so command details here are updated
        .author("LRitzdorf")
        .subcommand(Command::new("query")
            .about("Display the current battery conservation status")
            .visible_alias("status")
            .visible_alias("?")
        )
        .subcommand(Command::new("enable")
            .about("Enable battery conservation mode")
            .visible_alias("true")
            .visible_alias("on")
            .visible_alias("yes")
            .visible_alias("1")
        )
        .subcommand(Command::new("disable")
            .about("Disable battery conservation mode")
            .visible_alias("false")
            .visible_alias("off")
            .visible_alias("no")
            .visible_alias("0")
        )
        .subcommand(Command::new("toggle")
            .about("Toggle battery conservation mode")
            .visible_alias("switch")
            .visible_alias("-1")
        )
        .infer_subcommands(true)
        .get_matches();

    let p = Path::new(TARGET);
    let mut f = match File::options().read(true).write(true).open(p) {
        Err(why) => panic!("Couldn't open conservation control file '{}': {}", p.display(), why),
        Ok(file) => file,
    };

    let result = match m.subcommand() {
        Some(("enable",  _))  => { set_mode(&mut f, true)? }
        Some(("disable", _))  => { set_mode(&mut f, false)? }
        Some(("toggle",  _))  => { let mode = get_mode(&mut f)?; set_mode(&mut f, !mode)? }
        _                     => { get_mode(&mut f)? }
    };
    println!("{}", result);
    Ok(())
}
