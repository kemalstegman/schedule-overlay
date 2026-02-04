use serde::Deserialize;
use std::env;
use std::fs;
use std::path::Path;

#[derive(Deserialize)]
struct Config {
    groups: Vec<Group>,
    events: Vec<Event>,
}

#[derive(Deserialize)]
struct Group {
    name: String,
    color: [u8; 3],
}

#[derive(Deserialize)]
struct Event {
    day: i32,
    start: f32,
    end: f32,
    label: String,
    group_index: usize,
}

fn main() {
    // Tell Cargo to rerun this if schedule.toml changes
    println!("cargo:rerun-if-changed=schedule.toml");

    let toml_str = fs::read_to_string("src/schedule.toml").expect("Failed to read schedule.toml");
    let config: Config = toml::from_str(&toml_str).expect("Failed to parse TOML");

    let out_dir = env::var_os("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("generated_data.rs");

    let mut code = String::new();

    // Start a block
    code.push_str("{\n");

    // Generate group names
    code.push_str(&format!(
        "    let group_names: Vec<slint::SharedString> = vec![{}];\n",
        config
            .groups
            .iter()
            .map(|g| format!("slint::SharedString::from(\"{}\")", g.name))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    // Generate group colors
    code.push_str(&format!(
        "    let group_colors = vec![{}];\n",
        config
            .groups
            .iter()
            .map(|g| format!(
                "slint::Color::from_rgb_u8({}, {}, {})",
                g.color[0], g.color[1], g.color[2]
            ))
            .collect::<Vec<_>>()
            .join(", ")
    ));

    // Generate calendar data
    code.push_str("    let calendar_data = vec![\n");
    for e in config.events {
        code.push_str(&format!(
                "        TimeSlot {{ day_index: {}, start_hour: {:.1}, end_hour: {:.1}, label: \"{}\".into(), group_id: {}, color: group_colors[{}].into() }},\n",
                e.day, e.start, e.end, e.label, e.group_index, e.group_index
            ));
    }
    code.push_str("    ];\n");

    // Return the data as a tuple so the variables exist outside the block
    // but the include! macro sees it as one "thing"
    code.push_str("    (group_names, calendar_data)\n");

    // Close the block
    code.push_str("}\n");

    fs::write(&dest_path, code).unwrap();

    // Also trigger Slint build if you're using a .slint file instead of macro
    // slint_build::compile("ui/app.slint").unwrap();
}
