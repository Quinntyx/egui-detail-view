pub mod render;

use egui::{epaint::RectShape, LayerId, Response, Ui};
#[allow(unused_imports)]
use egui::{
    pos2, vec2, Align, Align2, Color32, FontId, Layout, Mesh, Rect, Rounding, Sense, Stroke,
    TextStyle, Vec2, Widget, WidgetText,
};

use std::collections::HashSet;
#[allow(unused_imports)]
use std::sync::{Arc, Mutex};

use crate::render::*;

#[derive(Debug, Clone)]
pub struct FieldConfig {
    pub title: String,
    pub editable: bool,
    pub fixed_width: Option<f32>,
    pub sort_enabled: bool,
    pub sortable: bool, // if it impl's Ord or not
}

impl FieldConfig {
    pub fn title(&self) -> String {
        self.title.clone()
    }

    pub fn editable(&self) -> bool {
        self.editable
    }

    pub fn fixed_width(&self) -> Option<f32> {
        self.fixed_width
    }

    pub fn sort_enabled(&self) -> bool {
        self.sort_enabled
    }

    pub fn sortable(&self) -> bool {
        self.sortable
    }
}

impl Into<FieldConfig> for String {
    fn into(self) -> FieldConfig {
        FieldConfig {
            title: self.clone(),
            editable: true,
            fixed_width: None,
            sort_enabled: true,
            sortable: true,
        }
    }
}

impl Into<FieldConfig> for &str {
    fn into(self) -> FieldConfig {
        FieldConfig {
            title: self.to_string(),
            editable: true,
            fixed_width: None,
            sort_enabled: true,
            sortable: true,
        }
    }
}

pub trait DetailListEntry: Clone {
    /// get the data that should be shown for this specific entry under the field `title`.
    fn get_field_entry(&self, title: &str) -> Option<String>;

    /// get the widget to render into an entry. You *can* technically return both this and the
    /// string field entry, but if this won't get called if there's a string. So don't do that.
    /// Use this to render album art or something.
    #[allow(unused_variables)]
    fn get_field_entry_widget(&self, title: &str) -> Option<Box<dyn Widget>> {
        None
    }

    /// writes back an edited field. The returned value is whether or not the operation succeeded.
    /// This technically can be called on fields which are marked not editable, but only through
    /// code; the UI for editing that calls this won't come up if the field isn't editable.
    fn write_field(&mut self, title: &str, input_value: String) -> bool;

    /// gets the sort key for a particular field. For unsortable fields just return zero.
    fn sort_key(self, title: &str) -> Arc<Option<impl Ord>>;
}

pub struct DetailListState {
    fields: Vec<FieldConfig>,
    sorted_by: Option<String>,
    reverse_sort: bool,
    selection: HashSet<usize>,
}

impl DetailListState {
    pub fn new(fields: Vec<FieldConfig>) -> Self {
        Self {
            fields,
            sorted_by: None,
            reverse_sort: false,
            selection: HashSet::new(),
        }
    }
}

pub struct DetailList<'a, T: DetailListEntry> {
    entries: &'a mut [T],
    size: Option<Vec2>,
    state: &'a mut DetailListState,
    row_height: f32,
}

impl<'a, T: DetailListEntry> DetailList<'a, T> {
    pub fn new(entries: &'a mut [T], state: &'a mut DetailListState) -> Self {
        Self {
            entries,
            size: None,
            state,
            row_height: 20.,
        }
    }

    pub fn with_size(self, size: Vec2) -> Self {
        Self {
            size: Some(size),
            ..self
        }
    }

    pub fn with_row_height(self, row_height: f32) -> Self {
        Self { row_height, ..self }
    }
}

fn draw_header_element(
    ui: &mut Ui,
    field: FieldConfig,
    draw_arrow: bool,
    reverse: bool,
) -> Response {
    let painter = ui.painter();
    let text = painter.text(
        ui.max_rect().left_center(),
        Align2::LEFT_CENTER,
        field.title(),
        TextStyle::Heading.resolve(ui.style()),
        Color32::WHITE,
    );

    if draw_arrow {
        let tri_base = text.right_center() + vec2(10., -1. * 3f32.sqrt());
        let mut mesh: egui::Mesh = Mesh::default();
        mesh.add_triangle_simple(
            tri_base - vec2(2., 0.),
            tri_base + vec2(2., 0.),
            tri_base + vec2(0., 2. * 3f32.sqrt()),
            Color32::WHITE,
        );
        mesh.scale(2.);
        if reverse {
            mesh.mirror_y(text.right_center().y);
        }
        painter.add(mesh);
    }

    let (rect, rect_painter) = ui.allocate_painter(ui.max_rect().size(), Sense::click());

    let rect_rect = rect.rect.clone();

    rect_painter.add(RectShape::stroke(
        rect_rect,
        Rounding::ZERO,
        Stroke::new(1., Color32::BLUE),
    ));

    if rect.hovered() {
        rect_painter.add(RectShape::stroke(
            rect_rect,
            Rounding::ZERO,
            Stroke::new(1., Color32::WHITE),
        ));
    }

    rect
}

// TODO(quinntyx): figure out why I need so many 'statics
impl<'a, T: DetailListEntry> Widget for DetailList<'a, T> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let size = self.size.unwrap_or_else(|| ui.available_size());

        let mut table_builder = egui_extras::TableBuilder::new(ui).sense(Sense::click());

        for field in self.state.fields.iter() {
            if let Some(w) = field.fixed_width() {
                table_builder = table_builder.column(egui_extras::Column::exact(w))
            } else {
                table_builder =
                    table_builder.column(egui_extras::Column::remainder().resizable(true));
            }
        }

        let table = table_builder.header(self.row_height, |mut header| {
            for field in self.state.fields.iter() {
                header.col(|ui| {
                    if field.sort_enabled && !field.sortable {
                        panic!("invalid field configuration");
                    }
                    if draw_header_element(
                        ui,
                        field.clone(),
                        self.state.sorted_by == Some(field.title()),
                        self.state.reverse_sort,
                    )
                    .clicked()
                        && field.sort_enabled
                    {
                        if self.state.sorted_by == Some(field.title.clone()) {
                            self.state.reverse_sort = !self.state.reverse_sort;
                            if self.state.reverse_sort {
                                self.entries.sort_by_key(|e| {
                                    std::cmp::Reverse(e.clone().sort_key(&field.title).clone())
                                });
                            } else {
                                self.entries
                                    .sort_by_key(|e| e.clone().sort_key(&field.title).clone());
                            }
                        } else {
                            self.state.sorted_by = Some(field.title.clone());
                            self.entries
                                .sort_by_key(|e| e.clone().sort_key(&field.title).clone());
                            self.state.reverse_sort = false;
                        }
                    }
                });
            }
        });

        table.body(|mut body| {
            body.rows(25., self.entries.len(), |mut row| {
                let idx = row.index();

                if self.state.selection.contains(&idx) {
                    row.set_selected(true);
                }
                for field in self.state.fields.iter() {
                    row.col(|ui| {
                        let (response, painter) =
                            ui.allocate_painter(ui.max_rect().size(), Sense::click());

                        self.entries
                            .get(idx)
                            .unwrap()
                            .get_field_entry(&field.title)
                            .map(|e| {
                                painter.text(
                                    response.rect.left_center()
                                        + vec2(ui.style().spacing.button_padding.x, 0.),
                                    Align2::LEFT_CENTER,
                                    e,
                                    TextStyle::Body.resolve(ui.style()),
                                    Color32::WHITE,
                                )
                            });
                    });
                }

                if row.response().clicked() {
                    if !row.response().ctx.input(|i| i.modifiers.shift_only()) {
                        self.state.selection.clear()
                    }
                    self.state.selection.insert(idx);
                }
            })
        });

        ui.allocate_rect(
            Rect::from_min_size(ui.next_widget_position(), size),
            Sense::click(),
        )
    }
}
