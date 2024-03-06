use macroquad::ui::{hash, root_ui, widgets};

use macroquad::prelude::*;

#[macroquad::main("Events")]
async fn main() {
    loop {
        clear_background(BLACK);
        draw_rectangle(0.0, 0.0, screen_width() / 3.0, screen_height(), DARKGRAY);
        draw_rectangle(
            screen_width() / 3.0,
            0.0,
            screen_width() / 3.0,
            screen_height(),
            YELLOW,
        );
        draw_rectangle(
            screen_width() * 2.0 / 3.0,
            0.0,
            screen_width() / 3.0,
            screen_height(),
            DARKGRAY,
        );
        let (mouse_x, _) = mouse_position();
        if is_mouse_button_down(MouseButton::Left)
            && mouse_x > screen_width() / 3.0
            && mouse_x < screen_width() * 2.0 / 3.0
        {
            draw_text(
                "SUCCESS!!!",
                screen_width() / 2.0,
                screen_height() / 2.0,
                30.0,
                BLACK,
            );
        }
        next_frame().await;
    }
}
