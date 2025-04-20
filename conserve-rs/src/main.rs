use clap::{Command, command};
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};


// NOTE: Set your "battery conservation mode" file here:
const TARGET: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";


fn get_mode(f: &mut File) -> std::io::Result<bool> {
    let mut buf = [0u8; 2];
    f.read_exact(&mut buf)?;
    match &buf {
        b"0\n" => Ok(false),
        b"1\n" => Ok(true),
        _      => panic!("Unexpected conservation mode '{}'", String::from_utf8_lossy(&buf)),
    }
}

fn set_mode(f: &mut File, state: bool) -> std::io::Result<bool> {
    let buf = [b'0' + u8::from(state), b'\n'];
    f.write_all(&buf)?;
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

    match m.subcommand() {
        Some(("enable",  _))  => { println!("{}", set_mode(&mut f, true)?); }
        Some(("disable", _))  => { println!("{}", set_mode(&mut f, false)?); }
        Some(("toggle",  _))  => { let mode = get_mode(&mut f)?; println!("{}", set_mode(&mut f, !mode)?); }
        _                     => { println!("{}", get_mode(&mut f)?); }
    }
    Ok(())
}
