use egui::{vec2, Ui, Color32, pos2, Rect};

pub fn draw_chip8_screen(
    ui: &mut Ui,
    screen: &chip8::screen::Chip8Screen,
    pixel_scale: u32,
    foreground_color: Color32,
    background_color: Color32,
) -> egui::Response {
    let pixel_scale = pixel_scale as f32;
    let desired_size = pixel_scale * vec2(64.0, 32.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    let pixel_vec = pixel_scale * vec2(1., 1.);

    if ui.is_rect_visible(rect) {
        for y in 0..chip8::screen::SCREEN_HEIGHT_PIXELS as u8 {
            for x in 0..chip8::screen::SCREEN_WIDTH_PIXELS as u8 {
                let min = pixel_scale * vec2(x as f32, y as f32) + rect.min.to_vec2();
                let max = min + pixel_vec;
                let color = {
                    if screen.get_pixel(x, y) {
                        foreground_color
                    } else {
                        background_color
                    }
                };
                ui.painter().rect_filled(
                    Rect {
                        min: pos2(min.x, min.y),
                        max: pos2(max.x, max.y),
                    },
                    egui::Rounding::none(),
                    color,
                );
            }
        }
    }

    response
}
