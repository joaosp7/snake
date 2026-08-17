use macroquad::{
    prelude::*,
    rand::{gen_range, srand},
};
const MOVE_INTERVAL: f64 = 0.2;
const SWIPE_THRESHOLD: f32 = 30.0;

fn apply_swipe_direction(direction: &mut (i32, i32), delta: Vec2) {
    if delta.length() < SWIPE_THRESHOLD {
        return;
    }

    if delta.x.abs() > delta.y.abs() {
        if delta.x > 0.0 && *direction != (-1, 0) {
            *direction = (1, 0);
        } else if delta.x < 0.0 && *direction != (1, 0) {
            *direction = (-1, 0);
        }
    } else if delta.y > 0.0 && *direction != (0, -1) {
        *direction = (0, 1);
    } else if delta.y < 0.0 && *direction != (0, 1) {
        *direction = (0, -1);
    }
}

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut last_move = get_time();
    let mut snake = vec![(15, 15), (14, 15), (13, 15)];
    let cols = 30;
    let rows = 30;
    let cell_size = screen_width().min(screen_height()) / cols as f32;
    let grid_px_w = cell_size * cols as f32;
    let grid_px_h = cell_size * rows as f32;

    let origin_x = (screen_width() - grid_px_w) / 2.0;
    let origin_y = (screen_height() - grid_px_h) / 2.0;

    let mut direction = (0, -1);
    let mut touch_start: Option<(u64, Vec2)> = None;

    srand((get_time() * 1_000.0) as u64);
    let mut apple_position = (gen_range(0, cols), gen_range(0, rows));
    loop {
        //initial screen height -> 600
        // initial screen width -> 800
        clear_background(Color::from_hex(0xf1aaaa));
        //vertical lines
        for i in 1..=cols {
            let x = origin_x + cell_size * i as f32;
            draw_line(
                x,
                origin_y,
                x,
                origin_y + grid_px_h,
                2.0,
                Color::from_hex(0xcedada),
            );
        }
        //horizontal lines
        for i in 1..=rows {
            let y = origin_y + cell_size * i as f32;
            draw_line(
                origin_x,
                y,
                origin_x + grid_px_w,
                y,
                2.0,
                Color::from_hex(0xcedada),
            );
        }
        draw_rectangle_lines(
            origin_x,
            origin_y,
            grid_px_w,
            grid_px_h,
            2.0,
            Color::from_hex(0xcedada),
        );
        let fps = get_fps();
        println!("Current fps: {}", fps);

        draw_rectangle(
            origin_x + apple_position.0 as f32 * cell_size,
            origin_y + apple_position.1 as f32 * cell_size,
            cell_size - 2.0,
            cell_size - 2.0,
            BLACK,
        );

        //iterator - idiomatic rust
        for &(col, row) in &snake {
            draw_rectangle(
                origin_x + col as f32 * cell_size,
                origin_y + row as f32 * cell_size,
                cell_size - 2.0,
                cell_size - 2.0,
                GREEN,
            );
        }

        if is_key_pressed(KeyCode::Up) && direction != (0, 1) {
            direction = (0, -1)
        }
        if is_key_pressed(KeyCode::Down) && direction != (0, -1) {
            direction = (0, 1)
        }
        if is_key_pressed(KeyCode::Left) && direction != (1, 0) {
            direction = (-1, 0)
        }
        if is_key_pressed(KeyCode::Right) && direction != (-1, 0) {
            direction = (1, 0)
        }

        for touch in touches() {
            match touch.phase {
                TouchPhase::Started => touch_start = Some((touch.id, touch.position)),
                TouchPhase::Moved => {
                    if let Some((id, start)) = touch_start {
                        if id == touch.id {
                            let delta = touch.position - start;
                            if delta.length() >= SWIPE_THRESHOLD {
                                apply_swipe_direction(&mut direction, delta);
                                touch_start = None;
                            }
                        }
                    }
                }
                TouchPhase::Ended => {
                    if let Some((id, start)) = touch_start.take() {
                        if id == touch.id {
                            apply_swipe_direction(&mut direction, touch.position - start);
                        }
                    }
                }
                TouchPhase::Cancelled => touch_start = None,
                TouchPhase::Stationary => {}
            }
        }

        if get_time() - last_move >= MOVE_INTERVAL {
            let (current_head_x, current_head_y) = snake[0];
            let next_head = (
                (current_head_x + direction.0 as i32).rem_euclid(cols),
                (current_head_y + direction.1 as i32).rem_euclid(rows),
            );

            if snake.iter().take(snake.len() - 1).any(|&x| x == next_head) {
                println!("Game is over!");
                panic!("You are dead.");
            }
            snake.insert(0, next_head);

            if next_head == apple_position {
                apple_position = (gen_range(0, cols), gen_range(0, rows));
            } else {
                snake.pop();
            }

            last_move = get_time(); //updates time
        }

        next_frame().await
    }
}
