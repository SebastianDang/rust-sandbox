use bevy::prelude::*;
use bevy_prototype_debug_lines::*;

use super::foothold::*;

const DEFAULT_COLOR: Color = Color::WHITE;
const DEFAULT_PALETTE: [Color; 37] = [
    Color::ALICE_BLUE,
    Color::AQUAMARINE,
    Color::AZURE,
    Color::BEIGE,
    Color::BISQUE,
    Color::BLACK,
    Color::BLUE,
    Color::CRIMSON,
    Color::CYAN,
    Color::DARK_GRAY,
    Color::DARK_GREEN,
    Color::FUCHSIA,
    Color::GOLD,
    Color::GRAY,
    Color::GREEN,
    Color::INDIGO,
    Color::LIME_GREEN,
    Color::MAROON,
    Color::MIDNIGHT_BLUE,
    Color::NAVY,
    Color::NONE,
    Color::OLIVE,
    Color::ORANGE,
    Color::ORANGE_RED,
    Color::PINK,
    Color::PURPLE,
    Color::RED,
    Color::SALMON,
    Color::SEA_GREEN,
    Color::SILVER,
    Color::TEAL,
    Color::TOMATO,
    Color::TURQUOISE,
    Color::VIOLET,
    Color::WHITE,
    Color::YELLOW,
    Color::YELLOW_GREEN,
];

#[derive(Copy, Clone, Component)]
pub struct RenderColor {
    pub color: Color,
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
    pub fn with_id(i: usize) -> Self {
        DEFAULT_PALETTE[i % DEFAULT_PALETTE.len()].into()
    }
}
pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(DebugLinesPlugin::default())
            .add_system(render_sprites_system)
            .add_system(render_footholds_system);
    }
}

fn render_sprites_system(
    mut debug_lines: ResMut<DebugLines>,
    images: Res<Assets<Image>>,
    sprites: Query<
        (&Transform, &Handle<Image>, &RenderColor),
        (With<Transform>, With<Sprite>, With<RenderColor>),
    >,
) {
    for (transform, texture, render_color) in sprites.iter() {
        let position = transform.translation;
        let color = render_color.color;

        if let Some(image) = images.get(texture) {
            let width = image.texture_descriptor.size.width as f32;
            let height = image.texture_descriptor.size.height as f32;

            let top_left = Vec3::new(position.x - width / 2.0, position.y + height / 2.0, 0.0);
            let top_right = Vec3::new(position.x + width / 2.0, position.y + height / 2.0, 0.0);
            let bottom_left = Vec3::new(position.x - width / 2.0, position.y - height / 2.0, 0.0);
            let bottom_right = Vec3::new(position.x + width / 2.0, position.y - height / 2.0, 0.0);

            debug_lines.line_colored(top_left, top_right, 0., color);
            debug_lines.line_colored(top_left, bottom_left, 0., color);
            debug_lines.line_colored(top_right, bottom_right, 0., color);
            debug_lines.line_colored(bottom_left, bottom_right, 0., color);
        }
    }
}

fn render_footholds_system(
    mut debug_lines: ResMut<DebugLines>,
    footholds: Query<(&Foothold, &RenderColor), (With<Foothold>, With<RenderColor>)>,
) {
    for (foothold, render_color) in footholds.iter() {
        let color = render_color.color;
        let p1 = Vec3::new(foothold.x1, foothold.y1, 0.0);
        let p2 = Vec3::new(foothold.x2, foothold.y2, 0.0);
        debug_lines.line_colored(p1, p2, 0., color);
    }
}
