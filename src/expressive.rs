#![allow(dead_code)]

use eframe::egui::epaint::{PathShape, PathStroke, Vertex};
use eframe::egui::{self, Color32, Id, Pos2, Rect, Shape, Stroke, TextureId, Vec2, vec2};
use std::f32::consts::TAU;

pub const RADIUS_NONE: f32 = 0.0;
pub const RADIUS_XS: f32 = 4.0;
pub const RADIUS_SM: f32 = 8.0;
pub const RADIUS_MD: f32 = 12.0;
pub const RADIUS_LG: f32 = 16.0;
pub const RADIUS_LG_INCREASED: f32 = 20.0;
pub const RADIUS_XL: f32 = 28.0;
pub const RADIUS_XL_INCREASED: f32 = 32.0;
pub const RADIUS_XXL: f32 = 48.0;

pub const SPRING_FAST_SPATIAL_STIFFNESS: f32 = 380.0;
pub const SPRING_FAST_SPATIAL_DAMPING: f32 = 0.73;
pub const SPRING_FAST_EFFECTS_DAMPING: f32 = 1.0;

pub const SAMPLES: usize = 96;

pub const COVER_SHAPES: [Glyph; 10] = [
    Glyph::Circle,
    Glyph::Square,
    Glyph::Pentagon,
    Glyph::Diamond,
    Glyph::Flower,
    Glyph::Sunny,
    Glyph::Cookie9,
    Glyph::Burst,
    Glyph::Puffy,
    Glyph::Zigzag,
];

pub fn cover_shape(index: usize) -> Glyph {
    COVER_SHAPES[index % COVER_SHAPES.len()]
}

pub fn cover_name(index: usize) -> &'static str {
    match cover_shape(index) {
        Glyph::Circle => "Circle",
        Glyph::Square => "Rounded square",
        Glyph::Pentagon => "Pentagon",
        Glyph::Diamond => "Diamond",
        Glyph::Flower => "Flower",
        Glyph::Sunny => "Sunny",
        Glyph::Cookie9 => "Cookie",
        Glyph::Burst => "Burst",
        Glyph::Puffy => "Puffy",
        Glyph::Zigzag => "Zigzag",
        _ => "Circle",
    }
}

pub mod typography {
    pub const DISPLAY_LARGE: f32 = 36.0;
    pub const DISPLAY_MEDIUM: f32 = 28.0;
    pub const HEADLINE_LARGE: f32 = 26.0;
    pub const HEADLINE_MEDIUM: f32 = 22.0;
    pub const TITLE_LARGE: f32 = 20.0;
    pub const TITLE_MEDIUM: f32 = 16.0;
    pub const BODY_LARGE: f32 = 16.0;
    pub const BODY_MEDIUM: f32 = 14.0;
    pub const LABEL_LARGE: f32 = 14.0;
    pub const LABEL_MEDIUM: f32 = 12.0;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Glyph {
    Circle,
    Square,
    Oval,
    Pill,
    Slanted,
    Arch,
    SemiCircle,
    Triangle,
    Diamond,
    Pentagon,
    Gem,
    Heart,
    Arrow,
    Fan,
    Flower,
    Clover4,
    Clover8,
    Bun,
    Sunny,
    VerySunny,
    Cookie4,
    Cookie6,
    Cookie7,
    Cookie9,
    Cookie12,
    Burst,
    SoftBurst,
    Boom,
    SoftBoom,
    Puffy,
    PuffyDiamond,
    PixelCircle,
    PixelTriangle,
    Ghostish,
    ClamShell,
    Zigzag,
}

enum Geometry {
    Ellipse(f32, f32),
    Superellipse(f32, f32, f32),
    Polygon(Vec<Vec2>, f32),
    Radial(usize, f32, f32, f32),
    Heart,
    Arch,
    Fan,
    Arrow,
    Zigzag,
}

fn geometry(glyph: Glyph) -> Geometry {
    match glyph {
        Glyph::Circle => Geometry::Ellipse(1.0, 1.0),
        Glyph::Oval => Geometry::Ellipse(0.86, 1.0),
        Glyph::Pill => Geometry::Superellipse(1.0, 0.62, 3.4),
        Glyph::Square => Geometry::Superellipse(1.0, 1.0, 7.0),
        Glyph::PuffyDiamond => Geometry::Superellipse(1.0, 1.0, 4.0),
        Glyph::PixelCircle => Geometry::Superellipse(1.0, 1.0, 12.0),
        Glyph::Triangle => Geometry::Polygon(regular(3, -TAU / 4.0), 0.17),
        Glyph::PixelTriangle => Geometry::Polygon(regular(3, -TAU / 4.0), 0.04),
        Glyph::Diamond => Geometry::Polygon(regular(4, 0.0), 0.13),
        Glyph::Pentagon => Geometry::Polygon(regular(5, -TAU / 4.0), 0.19),
        Glyph::Gem => Geometry::Polygon(regular(6, 0.0), 0.09),
        Glyph::Slanted => Geometry::Polygon(vec![vec2(-0.85, -1.0), vec2(1.0, -1.0), vec2(0.85, 1.0), vec2(-1.0, 1.0)], 0.07),
        Glyph::SemiCircle => Geometry::Polygon(semicircle(), 0.03),
        Glyph::Burst => Geometry::Polygon(star(12, 0.56), 0.03),
        Glyph::SoftBurst => Geometry::Polygon(star(10, 0.62), 0.2),
        Glyph::Boom => Geometry::Polygon(star(7, 0.44), 0.08),
        Glyph::SoftBoom => Geometry::Polygon(star(6, 0.58), 0.26),
        Glyph::Flower => Geometry::Radial(8, 0.46, 1.6, 0.0),
        Glyph::Clover4 => Geometry::Radial(4, 0.62, 0.9, 0.0),
        Glyph::Clover8 => Geometry::Radial(8, 0.52, 0.9, 0.0),
        Glyph::Bun => Geometry::Radial(2, 0.4, 1.0, 0.0),
        Glyph::Sunny => Geometry::Radial(8, 0.36, 3.0, 0.0),
        Glyph::VerySunny => Geometry::Radial(16, 0.28, 2.6, 0.0),
        Glyph::Cookie4 => Geometry::Radial(4, 0.16, 0.55, 0.0),
        Glyph::Cookie6 => Geometry::Radial(6, 0.15, 0.55, 0.0),
        Glyph::Cookie7 => Geometry::Radial(7, 0.14, 0.55, 0.0),
        Glyph::Cookie9 => Geometry::Radial(9, 0.12, 0.55, 0.0),
        Glyph::Cookie12 => Geometry::Radial(12, 0.1, 0.55, 0.0),
        Glyph::Puffy => Geometry::Radial(9, 0.11, 0.5, 0.0),
        Glyph::Ghostish => Geometry::Radial(5, 0.24, 0.7, TAU / 4.0),
        Glyph::ClamShell => Geometry::Radial(6, 0.34, 0.8, TAU / 4.0),
        Glyph::Heart => Geometry::Heart,
        Glyph::Arch => Geometry::Arch,
        Glyph::Fan => Geometry::Fan,
        Glyph::Arrow => Geometry::Arrow,
        Glyph::Zigzag => Geometry::Zigzag,
    }
}

fn regular(sides: usize, rotation: f32) -> Vec<Vec2> {
    (0..sides)
        .map(|index| {
            let angle = rotation + index as f32 * TAU / sides as f32;
            vec2(angle.cos(), angle.sin())
        })
        .collect()
}

fn star(points: usize, inner: f32) -> Vec<Vec2> {
    (0..points * 2)
        .map(|index| {
            let angle = -TAU / 4.0 + index as f32 * TAU / (points * 2) as f32;
            let radius = if index % 2 == 0 { 1.0 } else { inner };
            vec2(angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

fn semicircle() -> Vec<Vec2> {
    let mut points = vec![vec2(-1.0, 1.0)];
    points.extend((0..=12).map(|index| {
        let angle = std::f32::consts::PI - index as f32 * std::f32::consts::PI / 12.0;
        vec2(angle.cos(), -angle.sin())
    }));
    points.push(vec2(-1.0, 1.0));
    points
}

fn resample(points: Vec<Vec2>, target: usize) -> Vec<Vec2> {
    if points.len() == target {
        return points;
    }
    let mut lengths = Vec::with_capacity(points.len() + 1);
    lengths.push(0.0);
    for index in 0..points.len() {
        lengths.push(lengths[index] + (points[(index + 1) % points.len()] - points[index]).length());
    }
    let total = lengths[lengths.len() - 1];
    if total <= f32::EPSILON {
        return vec![Vec2::ZERO; target];
    }
    let mut segment = 0;
    (0..target)
        .map(|index| {
            let wanted = total * index as f32 / target as f32;
            while segment + 1 < lengths.len() && lengths[segment + 1] < wanted {
                segment += 1;
            }
            let span = (lengths[segment + 1] - lengths[segment]).max(f32::EPSILON);
            let fraction = (wanted - lengths[segment]) / span;
            points[segment] + (points[(segment + 1) % points.len()] - points[segment]) * fraction
        })
        .collect()
}

fn rounded(vertices: &[Vec2], amount: f32, target: usize) -> Vec<Vec2> {
    let sides = vertices.len();
    let per_corner = (target / sides).max(2);
    let mut points = Vec::with_capacity(sides * per_corner);
    for index in 0..sides {
        let previous = vertices[(index + sides - 1) % sides];
        let current = vertices[index];
        let next = vertices[(index + 1) % sides];
        let start = current + (previous - current) * amount;
        let end = current + (next - current) * amount;
        for step in 0..per_corner {
            let t = step as f32 / per_corner as f32;
            let inverse = 1.0 - t;
            points.push(start * (inverse * inverse) + current * (2.0 * inverse * t) + end * (t * t));
        }
    }
    resample(points, target)
}

fn ellipse(rx: f32, ry: f32, target: usize) -> Vec<Vec2> {
    (0..target)
        .map(|index| {
            let angle = TAU * index as f32 / target as f32;
            vec2(angle.cos() * rx, angle.sin() * ry)
        })
        .collect()
}

fn superellipse(rx: f32, ry: f32, exponent: f32, target: usize) -> Vec<Vec2> {
    let power = 2.0 / exponent;
    (0..target)
        .map(|index| {
            let angle = TAU * index as f32 / target as f32;
            let cosine = angle.cos();
            let sine = angle.sin();
            vec2(
                cosine.signum() * cosine.abs().powf(power) * rx,
                sine.signum() * sine.abs().powf(power) * ry,
            )
        })
        .collect()
}

fn radial(lobes: usize, depth: f32, exponent: f32, phase: f32, target: usize) -> Vec<Vec2> {
    (0..target)
        .map(|index| {
            let angle = TAU * index as f32 / target as f32;
            let wave = (lobes as f32 * angle + phase).cos();
            let shaped = wave.abs().powf(exponent) * wave.signum();
            let radius = 1.0 - depth * 0.5 * (1.0 - shaped);
            vec2(angle.cos() * radius, angle.sin() * radius)
        })
        .collect()
}

fn heart(target: usize) -> Vec<Vec2> {
    resample(
        (0..target)
            .map(|index| {
                let angle = TAU * index as f32 / target as f32;
                let x = 16.0 * angle.sin().powi(3) / 17.0;
                let y = -(13.0 * angle.cos() - 5.0 * (2.0 * angle).cos() - 2.0 * (3.0 * angle).cos() - (4.0 * angle).cos()) / 17.0;
                vec2(x, -y)
            })
            .collect(),
        target,
    )
}

fn arch(target: usize) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(target);
    for index in 0..target {
        let t = index as f32 / target as f32;
        let point = if t < 0.72 {
            let angle = std::f32::consts::PI - t / 0.72 * std::f32::consts::PI;
            vec2(angle.cos(), -angle.sin().max(0.0))
        } else if t < 0.86 {
            vec2(1.0 - (t - 0.72) / 0.14 * 0.1, 0.0)
        } else {
            vec2(0.9 - (t - 0.86) / 0.14 * 1.9, 0.0)
        };
        points.push(point);
    }
    points
}

fn fan(target: usize) -> Vec<Vec2> {
    let mut points = vec![Vec2::ZERO];
    points.extend((0..target - 1).map(|index| {
        let angle = -std::f32::consts::PI / 3.0 + index as f32 / (target - 2) as f32 * (std::f32::consts::PI * 2.0 / 3.0);
        vec2(angle.cos(), angle.sin())
    }));
    points
}

fn arrow(target: usize) -> Vec<Vec2> {
    let corners = vec![
        vec2(-0.35, -1.0),
        vec2(0.55, -1.0),
        vec2(0.55, -0.42),
        vec2(1.0, -0.42),
        vec2(0.0, 0.45),
        vec2(-1.0, -0.42),
        vec2(-0.35, -0.42),
    ];
    resample(rounded(&corners, 0.06, target), target)
}

fn zigzag(target: usize) -> Vec<Vec2> {
    let waves = 4.0;
    let amplitude = 0.34;
    let thickness = 0.16;
    let steps = (target / 2).max(2);
    let mut top = Vec::with_capacity(steps + 1);
    let mut bottom = Vec::with_capacity(steps + 1);
    for index in 0..=steps {
        let t = index as f32 / steps as f32;
        let x = t * 2.0 - 1.0;
        let saw = (t * waves).fract() * 2.0 - 1.0;
        top.push(vec2(x, saw * amplitude - thickness));
        bottom.push(vec2(x, saw * amplitude + thickness));
    }
    top.extend(bottom.into_iter().rev());
    resample(top, target.max(8))
}

fn align_outline(points: Vec<Vec2>) -> Vec<Vec2> {
    let mut points = points;
    let area: f32 = points
        .iter()
        .enumerate()
        .map(|(index, point)| {
            let next = points[(index + 1) % points.len()];
            point.x * next.y - next.x * point.y
        })
        .sum();
    if area < 0.0 {
        points.reverse();
    }
    let top = vec2(0.0, -1.0);
    let first = points
        .iter()
        .enumerate()
        .min_by(|(_, a), (_, b)| (**a - top).length_sq().total_cmp(&(**b - top).length_sq()))
        .map_or(0, |(index, _)| index);
    points.rotate_left(first);
    points
}

fn outline(glyph: Glyph, target: usize) -> Vec<Vec2> {
    let target = target.max(8);
    let points = match geometry(glyph) {
        Geometry::Ellipse(rx, ry) => ellipse(rx, ry, target),
        Geometry::Superellipse(rx, ry, exponent) => superellipse(rx, ry, exponent, target),
        Geometry::Polygon(vertices, round) => rounded(&vertices, round, target),
        Geometry::Radial(lobes, depth, exponent, phase) => radial(lobes, depth, exponent, phase, target),
        Geometry::Heart => heart(target),
        Geometry::Arch => resample(arch(target), target),
        Geometry::Fan => resample(fan(target), target),
        Geometry::Arrow => arrow(target),
        Geometry::Zigzag => zigzag(target),
    };
    align_outline(points)
}

fn map_points(points: &[Vec2], rect: Rect) -> Vec<Pos2> {
    let half = rect.size() * 0.5;
    let center = rect.center();
    points
        .iter()
        .map(|point| center + vec2(point.x * half.x, point.y * half.y))
        .collect()
}

fn path_shape(points: Vec<Pos2>, fill: Color32, stroke: Stroke) -> Shape {
    let stroke = if stroke.width > 0.0 {
        PathStroke::new(stroke.width, stroke.color).inside()
    } else {
        PathStroke::NONE
    };
    Shape::Path(PathShape {
        points,
        closed: true,
        fill,
        stroke,
    })
}

impl Glyph {
    pub fn sample(self, samples: usize) -> Vec<Vec2> {
        outline(self, samples)
    }

    pub fn shape(self, rect: Rect, fill: Color32, stroke: Stroke) -> Shape {
        path_shape(map_points(&outline(self, SAMPLES), rect), fill, stroke)
    }

    pub fn textured(self, rect: Rect, texture: TextureId, tint: Color32) -> Shape {
        masked(rect, outline(self, SAMPLES), texture, tint)
    }
}

pub fn morph_outline(from: Glyph, to: Glyph, amount: f32) -> Vec<Vec2> {
    let first = outline(from, SAMPLES);
    if amount <= 0.0 {
        return first;
    }
    let second = outline(to, SAMPLES);
    if amount >= 1.0 {
        return second;
    }
    first.iter().zip(&second).map(|(a, b)| *a + (*b - *a) * amount).collect()
}

pub fn morph(from: Glyph, to: Glyph, amount: f32, rect: Rect, fill: Color32, stroke: Stroke) -> Shape {
    path_shape(map_points(&morph_outline(from, to, amount), rect), fill, stroke)
}

pub fn outline_shape(points: Vec<Vec2>, rect: Rect, fill: Color32, stroke: Stroke) -> Shape {
    path_shape(map_points(&points, rect), fill, stroke)
}

pub fn textured_outline(rect: Rect, points: Vec<Vec2>, texture: TextureId, tint: Color32) -> Shape {
    masked(rect, points, texture, tint)
}

pub fn masked(rect: Rect, points: Vec<Vec2>, texture: TextureId, tint: Color32) -> Shape {
    let center = Vertex {
        pos: rect.center(),
        uv: pos2(0.5, 0.5),
        color: tint,
    };
    let mut mesh = egui::Mesh {
        texture_id: texture,
        ..Default::default()
    };
    mesh.vertices.push(center);
    for point in &points {
        mesh.vertices.push(Vertex {
            pos: rect.center() + vec2(point.x * rect.width() * 0.5, point.y * rect.height() * 0.5),
            uv: pos2((point.x + 1.0) * 0.5, (point.y + 1.0) * 0.5),
            color: tint,
        });
    }
    for index in 0..points.len() {
        mesh.add_triangle(0, index as u32 + 1, (index + 1) as u32 % points.len() as u32 + 1);
    }
    Shape::mesh(mesh)
}

fn pos2(x: f32, y: f32) -> Pos2 {
    Pos2::new(x, y)
}

#[derive(Clone, Copy)]
pub struct Spring {
    pub stiffness: f32,
    pub damping: f32,
}

pub const FAST_SPATIAL: Spring = Spring {
    stiffness: SPRING_FAST_SPATIAL_STIFFNESS,
    damping: SPRING_FAST_SPATIAL_DAMPING,
};

pub const FAST_EFFECTS: Spring = Spring {
    stiffness: SPRING_FAST_SPATIAL_STIFFNESS,
    damping: SPRING_FAST_EFFECTS_DAMPING,
};

#[derive(Clone, Copy)]
struct State {
    value: f32,
    velocity: f32,
    time: f64,
}

pub fn spring_keyed(ctx: &egui::Context, id: Id, key: &str, target: f32, settings: Spring) -> f32 {
    let now = ctx.input(|input| input.time);
    let key_id = id.with("key");
    let changed = ctx.data_mut(|data| {
        let stored = data.get_temp_mut_or_insert_with::<String>(key_id, String::new);
        if *stored != key {
            *stored = key.to_owned();
            true
        } else {
            false
        }
    });
    if changed {
        ctx.data_mut(|data| {
            data.insert_temp(
                id,
                State {
                    value: 0.0,
                    velocity: 0.0,
                    time: now,
                },
            );
        });
    }
    spring(ctx, id, target, settings)
}

pub fn spring(ctx: &egui::Context, id: Id, target: f32, settings: Spring) -> f32 {
    let now = ctx.input(|input| input.time);
    let (state, moving) = ctx.data_mut(|data| {
        let entry = data.get_temp_mut_or_insert_with(id, || State {
            value: target,
            velocity: 0.0,
            time: now,
        });
        let elapsed = (now - entry.time).clamp(0.0, 0.05) as f32;
        entry.time = now;
        let delta = entry.value - target;
        let moving = delta.abs() >= 0.0005 || entry.velocity.abs() >= 0.0005;
        if moving {
            let friction = 2.0 * settings.stiffness.sqrt() * settings.damping;
            entry.velocity += (-settings.stiffness * delta - friction * entry.velocity) * elapsed;
            entry.value += entry.velocity * elapsed;
        } else {
            entry.value = target;
            entry.velocity = 0.0;
        }
        (*entry, moving)
    });
    if moving {
        ctx.request_repaint();
    }
    state.value
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_shape_samples_the_same_count() {
        let all = [
            Glyph::Circle,
            Glyph::Square,
            Glyph::Oval,
            Glyph::Pill,
            Glyph::Slanted,
            Glyph::Arch,
            Glyph::SemiCircle,
            Glyph::Triangle,
            Glyph::Diamond,
            Glyph::Pentagon,
            Glyph::Gem,
            Glyph::Heart,
            Glyph::Arrow,
            Glyph::Fan,
            Glyph::Flower,
            Glyph::Clover4,
            Glyph::Clover8,
            Glyph::Bun,
            Glyph::Sunny,
            Glyph::VerySunny,
            Glyph::Cookie4,
            Glyph::Cookie6,
            Glyph::Cookie7,
            Glyph::Cookie9,
            Glyph::Cookie12,
            Glyph::Burst,
            Glyph::SoftBurst,
            Glyph::Boom,
            Glyph::SoftBoom,
            Glyph::Puffy,
            Glyph::PuffyDiamond,
            Glyph::PixelCircle,
            Glyph::PixelTriangle,
            Glyph::Ghostish,
            Glyph::ClamShell,
            Glyph::Zigzag,
        ];
        assert_eq!(all.len(), 36);
        for glyph in all {
            let points = glyph.sample(SAMPLES);
            assert_eq!(points.len(), SAMPLES, "{glyph:?}");
            assert!(points.iter().all(|point| point.x.is_finite() && point.y.is_finite()), "{glyph:?}");
            assert!(points.iter().all(|point| point.x.abs() <= 1.1 && point.y.abs() <= 1.1), "{glyph:?}");
        }
    }

    #[test]
    fn morph_ends_on_its_target_outline() {
        let start = Glyph::Circle;
        let end = Glyph::Pentagon;
        assert_eq!(morph_outline(start, end, 0.0), start.sample(SAMPLES));
        assert_eq!(morph_outline(start, end, 1.0), end.sample(SAMPLES));
    }

    #[test]
    fn spring_repaints_without_holding_context_state_lock() {
        let ctx = egui::Context::default();
        let id = Id::new("spring-lock");
        assert_eq!(spring(&ctx, id, 0.0, FAST_SPATIAL), 0.0);
        let changed = spring(&ctx, id, 1.0, FAST_SPATIAL);
        assert!((0.0..=1.0).contains(&changed));
    }

    #[test]
    fn masked_geometry_has_a_triangle_per_edge() {
        let rect = Rect::from_min_size(Pos2::ZERO, vec2(80.0, 80.0));
        let points = Glyph::Pentagon.sample(12);
        let shape = masked(rect, points, TextureId::default(), Color32::WHITE);
        let Shape::Mesh(mesh) = shape else { panic!("expected a mesh") };
        assert_eq!(mesh.vertices.len(), 13);
        assert_eq!(mesh.indices.len(), 36);
        assert_eq!(mesh.vertices[0].uv, pos2(0.5, 0.5));
    }
}
