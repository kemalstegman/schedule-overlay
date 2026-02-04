slint::slint! {
    import { CheckBox, ScrollView, VerticalBox } from "std-widgets.slint";

    export struct TimeSlot {
        day_index: int,
        start_hour: float,
        end_hour: float,
        label: string,
        group_id: int,
        color: brush, // Added color property
    }

    export component AppWindow inherits Window {
        title: "Rust Calendar 2026";
        preferred-width: 900px;
        preferred-height: 700px;
        background: white;

        in property <[TimeSlot]> slots: [];
        in property <[string]> group_names: [];
        in-out property <[bool]> group_visibility: [];

        property <[string]> day_names: ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
        property <float> start_view_hour: 6.0;
        property <float> end_view_hour: 22.0;

        property <length> hour_height: (root.height - 70px) / (end_view_hour - start_view_hour);
        property <length> day_width: (root.width - 180px - 70px) / 7;

        HorizontalLayout {
            spacing: 0px;
            padding: 0px;

            Rectangle {
                width: 200px;
                background: #f8f9fa;
                Rectangle { width: 1px; x: parent.width - 1px; background: #ddd; }

                VerticalLayout {
                    alignment: start;
                    spacing: 0px;
                    Rectangle {
                        height: 50px;
                        HorizontalLayout {
                            padding-left: 18px;
                            padding-top: 2px;
                            Text {
                                text: "Toggle Groups";
                                font-size: 16px;
                                font-weight: 700;
                                color: black;
                                vertical-alignment: center;
                            }
                        }
                    }

                    for visible[i] in root.group_visibility : Rectangle {
                        height: 32px;
                        HorizontalLayout {
                            padding-left: 18px;
                            padding-right: 15px;
                            spacing: 10px;
                            CheckBox {
                                checked: visible;
                                toggled => { root.group_visibility[i] = self.checked; }
                            }
                            Text {
                                text: root.group_names[i];
                                color: black;
                                font-size: 14px;
                                vertical-alignment: center;
                                font-weight: 500;
                            }
                        }
                    }
                }
            }

            VerticalLayout {
                spacing: 0px;
                Rectangle {
                    height: 30px;
                    background: #f1f1f1;
                    Rectangle { height: 1px; y: parent.height - 1px; background: #ddd; }
                    for name[i] in root.day_names : Text {
                        x: 70px + (i * root.day_width);
                        width: root.day_width;
                        text: name;
                        color: black;
                        font-weight: 700;
                        horizontal-alignment: center;
                        vertical-alignment: center;
                    }
                }

                ScrollView {
                    Rectangle {
                        background: white;
                        height: (root.end_view_hour - root.start_view_hour) * root.hour_height + 40px;
                        Rectangle {
                            y: 20px;
                            height: parent.height - 40px;
                            for i in 25 : Rectangle {
                                property <length> pos_y: (i * 1.0 - root.start_view_hour) * root.hour_height;
                                y: pos_y;
                                height: 20px;
                                width: 100%;
                                visible: i >= root.start_view_hour && i <= root.end_view_hour;
                                Rectangle { x: 65px; y: 10px; height: 1px; width: parent.width - 65px; background: #eee; }
                                Text {
                                    x: 5px; width: 55px; height: 20px;
                                    text: i + ":00"; font-size: 12px; color: #70757a;
                                    horizontal-alignment: right; vertical-alignment: center;
                                }
                            }

                            for slot[i] in root.slots : Rectangle {
                                visible: root.group_visibility[slot.group_id];
                                x: 70px + (slot.day_index * root.day_width);
                                y: (slot.start_hour - root.start_view_hour) * root.hour_height + 10px;
                                width: root.day_width - 4px;
                                height: (slot.end_hour - slot.start_hour) * root.hour_height;

                                // Dynamic color from the slot data
                                background: slot.color.transparentize(70%);
                                border-width: 2px;
                                border-color: slot.color;
                                border-radius: 4px;

                                Text {
                                    text: slot.label;
                                    font-size: 11px; color: black;
                                    overflow: elide; x: 4px; y: 2px;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    // Destructure the tuple returned by the included block
    let (group_names, calendar_data) = include!(concat!(env!("OUT_DIR"), "/generated_data.rs"));

    ui.set_group_names(std::rc::Rc::new(slint::VecModel::from(group_names.clone())).into());
    ui.set_slots(std::rc::Rc::new(slint::VecModel::from(calendar_data)).into());

    ui.set_group_visibility(
        std::rc::Rc::new(slint::VecModel::from(vec![false; group_names.len()])).into(),
    );

    ui.run()
}
