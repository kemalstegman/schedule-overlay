use std::rc::Rc;

use slint::{Color, SharedString, VecModel};

slint::include_modules!();

include!(concat!(env!("OUT_DIR"), "/schedules.rs"));

fn main() -> Result<(), slint::PlatformError> {
    let main_window = MainWindow::new()?;
    main_window.set_spans(
        Rc::new(VecModel::from(
            EVENTS
                .iter()
                .map(|e| TimeSpan::from(e))
                .collect::<Vec<TimeSpan>>(),
        ))
        .into(),
    );
    main_window.set_names(
        Rc::new(VecModel::from(
            OVERLAYS
                .iter()
                .map(|s| SharedString::from(*s))
                .collect::<Vec<SharedString>>(),
        ))
        .into(),
    );
    main_window.set_group_visibility(Rc::new(VecModel::from(vec![false; OVERLAYS.len()])).into());
    main_window.run()
}

#[derive(Debug)]
pub struct Event {
    pub label: &'static str,
    pub group: i32,
    pub color: [u8; 3],
    pub day: i32,
    pub start_hour: f32,
    pub end_hour: f32,
}

impl From<&Event> for TimeSpan {
    fn from(value: &Event) -> Self {
        TimeSpan {
            color: Color::from_rgb_u8(value.color[0], value.color[1], value.color[2]),
            day: value.day,
            end_hours: value.end_hour,
            group: value.group,
            label: SharedString::from(value.label),
            start_hours: value.start_hour,
        }
    }
}
