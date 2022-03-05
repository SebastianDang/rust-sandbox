use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use super::geometry::*;

const DEFAULT_COLOR: Color = Color::BEIGE;
// const DEFAULT_PALETTE: [Color; 10] = [
//     Color::ALICE_BLUE,
//     Color::ANTIQUE_WHITE,
//     Color::AQUAMARINE,
//     Color::AZURE,
//     Color::BEIGE,
//     Color::BISQUE,
//     Color::BLACK,
//     Color::BLUE,
//     Color::CRIMSON,
//     Color::CYAN,
// ];

#[derive(Copy, Clone, Component)]
pub struct RenderColor {
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

// impl RenderColor {
//     pub fn with_id(i: usize) -> Self {
//         DEFAULT_PALETTE[i % DEFAULT_PALETTE.len()].into()
//     }
// }
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_system(render_lines_system)
            .add_system(render_quads_system);
    }
}

fn render_lines_system(mut debug_lines: ResMut<DebugLines>, lines: Query<(&Line2d, &RenderColor)>) {
    for (line, render_color) in lines.iter() {
        let color = render_color.color;
        debug_lines.line_colored(line.p0.extend(0.0), line.p1.extend(0.0), 0., color);
    }
}

fn render_quads_system(mut debug_lines: ResMut<DebugLines>, quads: Query<(&Quad2d, &RenderColor)>) {
    for (quad, render_color) in quads.iter() {
        let color = render_color.color;

        let top_left = quad.top_left().extend(0.0);
        let top_right = quad.top_right().extend(0.0);
        let bottom_left = quad.bottom_left().extend(0.0);
        let bottom_right = quad.bottom_right().extend(0.0);

        debug_lines.line_colored(top_left, top_right, 0., color);
        debug_lines.line_colored(top_left, bottom_left, 0., color);
        debug_lines.line_colored(bottom_right, top_right, 0., color);
        debug_lines.line_colored(bottom_right, bottom_left, 0., color);
    }
}
