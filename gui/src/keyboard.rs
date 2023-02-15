// see here for layout information:
// http://devernay.free.fr/hacks/chip8/C8TECH10.HTM#2.3
const KEY_MAP: [egui::Key; 16] = [
    egui::Key::X, // 0
    egui::Key::Num1, // 1
    egui::Key::Num2, // 2
    egui::Key::Num3, // 3
    egui::Key::Q, // 4
    egui::Key::W, // 5
    egui::Key::E, // 6
    egui::Key::A, // 7
    egui::Key::S, // 8
    egui::Key::D, // 9
    egui::Key::Z, // A
    egui::Key::C, // B
    egui::Key::Num4, // C
    egui::Key::R, // D
    egui::Key::F, // E
    egui::Key::V, // F
];

pub fn get_key_state(input_state: &egui::InputState) -> [bool; 16] {
    let mut state = [false; 16];
    for (i, key) in KEY_MAP.iter().enumerate() {
        state[i] = input_state.key_down(*key);
    }
    state
}