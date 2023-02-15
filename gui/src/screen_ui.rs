pub fn draw_chip8_screen(
    ui: &mut egui::Ui,
    pixel_scale: u32,
    screen: &chip8::screen::Chip8Screen
) -> egui::Response {
    let pixel_scale = pixel_scale as f32;
    let desired_size = pixel_scale * egui::vec2(64.0, 32.0);
    let (rect, response) = ui.allocate_exact_size(desired_size, egui::Sense::hover());

    let on_color = egui::Rgba::WHITE;
    let off_color = egui::Rgba::BLACK;
    let pixel_vec = pixel_scale * egui::vec2(1., 1.);

    if ui.is_rect_visible(rect) {
        for y in 0..chip8::screen::SCREEN_HEIGHT_PIXELS as u8 {
            for x in 0..chip8::screen::SCREEN_WIDTH_PIXELS as u8{
                let min = pixel_scale * egui::vec2(x as f32, y as f32) + rect.min.to_vec2();
                let max = min + pixel_vec;
                let color = {
                    if screen.get_pixel(x, y) {
                        on_color
                    } else {
                        off_color
                    }
                };
                ui.painter().rect_filled(
                    egui::Rect {
                        min: egui::pos2(min.x, min.y),
                        max: egui::pos2(max.x, max.y),
                    },
                    egui::Rounding::none(),
                    color,
                );
            }
        }
    }

    response
}
