use macroquad::{input::{TouchPhase, *}, math::Vec2};

pub fn get_input_pos() -> Option<Vec2> {
    let mut input_pos: Option<Vec2> = None;
    for touch in touches() {
        if touch.phase == TouchPhase::Started {
            input_pos = Some(touch.position);
        }
    }

    let (mouse_x, mouse_y) = mouse_position();
    if is_mouse_button_pressed(MouseButton::Left) {
        input_pos = Some(Vec2::new(mouse_x, mouse_y));
    }

    return input_pos;
}