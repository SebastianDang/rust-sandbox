use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use super::geometry::*;

const DEFAULT_COLOR: Color = Color::BEIGE;
const DEFAULT_PALETTE: [Color; 10] = [
    Color::ALICE_BLUE,
    Color::ANTIQUE_WHITE,
    Color::AQUAMARINE,
    Color::AZURE,
    Color::BEIGE,
    Color::BISQUE,
    Color::BLACK,
    Color::BLUE,
    Color::CRIMSON,
    Color::CYAN,
];

#[derive(Copy, Clone, Component)]
struct RenderColor {
    color: Color,
}

impl Default for RenderColor {
    fn default() -> Self {
        DEFAULT_COLOR.into()
    }
}

impl From<Color> for RenderColor {
    fn from(color: Color) -> Self {
        RenderColor { color }
    }
}

impl RenderColor {
    fn with_id(i: usize) -> Self {
        DEFAULT_PALETTE[i % DEFAULT_PALETTE.len()].into()
    }
}
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(DebugLinesPlugin::default());

        app.add_system(render_lines_system)
            .add_system(render_quads_system);
    }
}

fn render_lines_system(mut debug_lines: ResMut<DebugLines>, lines: Query<(Entity, &Line2d)>) {
    for (entity, line) in lines.iter() {
        let color = RenderColor::with_id(entity.id() as usize).color;
        debug_lines.line_colored(line.p0.as_vec3(), line.p1.as_vec3(), 0., color);
    }
}

fn render_quads_system(mut debug_lines: ResMut<DebugLines>, quads: Query<(Entity, &Quad2d)>) {
    for (entity, quad) in quads.iter() {
        let color = RenderColor::with_id(entity.id() as usize).color;

        let top_left = quad.top_left().as_vec3();
        let top_right = quad.top_right().as_vec3();
        let bottom_left = quad.bottom_left().as_vec3();
        let bottom_right = quad.bottom_right().as_vec3();

        debug_lines.line_colored(top_left, top_right, 0., color);
        debug_lines.line_colored(top_left, bottom_left, 0., color);
        debug_lines.line_colored(bottom_right, top_right, 0., color);
        debug_lines.line_colored(bottom_right, bottom_left, 0., color);
    }
}
