#[allow(unused_imports)]
use egui::{pos2, vec2, Color32, Mesh, Painter, Pos2, Rect, Stroke, Vec2};
// use ecolor::HexColor;
use egui::epaint::Vertex;

fn pos2_to_vertex(a: Pos2, c: Color32) -> Vertex {
    let mut v = Vertex::default();
    v.pos = a;
    v.color = c;
    v
}

pub fn triangle(a: Pos2, b: Pos2, c: Pos2, color: Color32) -> Mesh {
    let mut vertices = vec![a, b, c]
        .iter_mut()
        .map(|v| pos2_to_vertex(*v, color))
        .collect::<Vec<_>>();

    let mut mesh = Mesh::default();
    mesh.vertices.append(&mut vertices);

    mesh.add_triangle(0, 1, 2);

    mesh
}

pub fn quad(a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32) -> Mesh {
    let mut m = Mesh::default();

    m.append(triangle(a, b, c, color));
    m.append(triangle(b, c, d, color));

    m
}

pub trait SimpleMeshBuilder {
    fn add_triangle_simple(&mut self, a: Pos2, b: Pos2, c: Pos2, color: Color32);
    fn add_quad_simple(&mut self, a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32);
    fn mirror_y(&mut self, horiz_line_height: f32);
    fn scale(&mut self, factor: f32);
    fn scale_at(&mut self, factor: f32, center: Pos2);
}

impl SimpleMeshBuilder for Mesh {
    fn add_triangle_simple(&mut self, a: Pos2, b: Pos2, c: Pos2, color: Color32) {
        self.append(triangle(a, b, c, color));
    }

    fn add_quad_simple(&mut self, a: Pos2, b: Pos2, c: Pos2, d: Pos2, color: Color32) {
        self.append(quad(a, b, c, d, color));
    }

    fn mirror_y(&mut self, hlh: f32) {
        self.vertices.iter_mut().for_each(|v| {
            let p = v.pos;
            v.pos = Pos2 {
                x: p.x,
                y: -p.y + 2. * hlh,
            }
        })
    }

    fn scale(&mut self, factor: f32) {
        self.scale_at(factor, self.calc_bounds().center());
    }

    fn scale_at(&mut self, factor: f32, center: Pos2) {
        self.vertices.iter_mut().for_each(|v| {
            let mut p = v.pos;
            p -= center.to_vec2();

            p.x *= factor;
            p.y *= factor;

            p += center.to_vec2();
            v.pos = p;
        });
    }
}
