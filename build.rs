use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    groups: Vec<Group>,
}

#[derive(Deserialize)]
struct Group {
    name: String,
    color: [u8; 3],
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Event {
    label: String,
    day: u32,
    start_hour: f32,
    end_hour: f32,
}

fn main() {
    println!("cargo:rerun-if-changed=config.toml");

    let toml_str =
        std::fs::read_to_string("src/schedules.toml").expect("Failed to read src/schedules.toml");
    let config: Config = toml::from_str(&toml_str).expect("Failed to parse TOML");

    let mut group_names = Vec::new();
    let mut code = String::from("pub const EVENTS: &[Event] = &[\n");
    for (i, g) in config.groups.iter().enumerate() {
        group_names.push(&g.name);
        for e in g.events.iter() {
            code.push_str(&format!("    Event {{ label: \"{}\", group: {}, color: {:?}, day: {}, start_hour: {:?}, end_hour: {:?} }},\n", e.label, i, g.color, e.day, e.start_hour, e.end_hour));
        }
    }
    code.push_str("];\n");
    code.push_str("pub const OVERLAYS: &[&'static str] = &[\n");
    for name in group_names {
        code.push_str(&format!("    \"{}\",\n", name));
    }
    code.push_str("];\n");
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let dest_path = std::path::Path::new(&out_dir).join("schedules.rs");
    std::fs::write(&dest_path, code).expect("Failed to write schedules.rs");

    slint_build::compile("ui/schedule.slint").expect("Slint build failed");
}
