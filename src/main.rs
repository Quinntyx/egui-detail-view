use egui::CentralPanel;
use egui_detail_view::*;

use std::sync::Arc;

#[derive(Clone)]
struct Entry {
    title: String,
}

impl DetailListEntry for Entry {
    fn get_field_entry(&self, title: &str) -> Option<String> {
        if title == "title" {
            Some(self.title.clone())
        } else if title == "genre" {
            Some(String::from("Electro"))
        } else if title == "date" {
            Some(String::from("1993"))
        } else {
            None
        }
    }

    #[allow(unused_variables)]
    fn write_field(&mut self, title: &str, input_value: String) -> bool {
        false
    }
    
    fn sort_key(self, title: &str) -> Arc<Option<impl Ord>> {
        if title == "title" {
            Arc::new(Some(self.title.clone()))
        } else {
            Arc::new(None)
        }
    }
}

struct App {
    state: DetailListState,
    entries: Vec<Entry>,
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        CentralPanel::default().show(ctx, |ui| {
            ui.add(DetailList::<Entry>::new(&mut self.entries, &mut self.state))
        });
    }
}

fn main() {
    eframe::run_native(
        "egui-detail-view example",
        Default::default(),
        Box::new(|_| {
            let field_config = vec![
                "title".into(),
                FieldConfig {
                    title: String::from("t"),
                    editable: false,
                    sortable: false,
                    fixed_width: Some(30.),
                    sort_enabled: false,
                },
                "artist".into(),
                "date".into(),
                "genre".into(),
            ];

            let entries = vec![
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
                Entry {
                    title: String::from("Foo"),
                },
                Entry {
                    title: String::from("Bar"),
                },
            ];

            Box::new(App {
                state: DetailListState::new(field_config),
                entries,
            })
        }),
    )
    .unwrap();
}
