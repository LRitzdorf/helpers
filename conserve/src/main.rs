use clap::{Command, command};


// NOTE: Set your "battery conservation mode" file here:
const TARGET: &str = "/sys/bus/platform/drivers/ideapad_acpi/VPC2004:00/conservation_mode";


fn get_mode() -> Result<bool, std::io::Error> {
    println!("get_state() called! Would read from {TARGET}");
    Ok(true)
}

fn set_mode(state: bool) -> Result<bool, std::io::Error> {
    println!("set_state({state}) called! Would write to {TARGET}");
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

    match m.subcommand() {
        Some(("enable",  _))     => { println!("{:#?}", set_mode(true)); }
        Some(("disable", _))     => { println!("{:#?}", set_mode(false)); }
        Some(("toggle",  _))     => { println!("{:#?}", set_mode(!get_mode()?)); }
        Some(("query",   _)) | _ => { println!("{:#?}", get_mode()?); }
    }
    Ok(())
}
