use std::time::{SystemTime, UNIX_EPOCH};
use macroquad::{prelude::*, rand::{gen_range, srand}};

const MOVE_INTERVAL: f64 = 0.2;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut last_move = get_time();
    let mut snake = vec![
        (15, 15),
        (14, 15),
        (13, 15),
    ];
    let cols = 30;
    let rows = 30;
    let cell_size = screen_width().min(screen_height()) / cols as f32 ;
    let grid_px_w = cell_size * cols as f32;
    let grid_px_h = cell_size * rows as f32;

    let origin_x = (screen_width() - grid_px_w) / 2.0 ;
    let origin_y = (screen_height() - grid_px_h) / 2.0;

    let mut direction = (0,-1);

    let seed = match SystemTime::now().duration_since(UNIX_EPOCH){
        Ok(time) => time.as_secs(),
        Err(_) => panic!("SystemTime before UNIX Epoch!")
    };

    srand(seed);
    let apple_position = (
        gen_range(0, rows),
        gen_range(0, cols)
    );
    loop {
    
        //initial screen height -> 600
        // initial screen width -> 800
        clear_background(RED);
        //vertical lines
        for i in 1..=cols{
            let x = origin_x + cell_size * i as f32;
            draw_line(x, origin_y, x, origin_y + grid_px_h, 2.0, BLUE);
        }
        //horizontal lines
        for i in 1..=rows{
            let y = origin_y + cell_size * i as f32;
            draw_line(origin_x, y, origin_x + grid_px_w, y, 2.0, BLUE);
        }
        draw_rectangle_lines(origin_x, origin_y, grid_px_w, grid_px_h, 2.0, BLUE);
        let fps = get_fps();
        println!("Current fps: {}",fps);


        if snake.len() == 3 {
            draw_rectangle(
                origin_x + apple_position.0 as f32 * cell_size,
                origin_y + apple_position.1 as f32 * cell_size,
                cell_size - 2.0 ,
                cell_size - 2.0 ,
                BLACK);
        }
        
        
        //iterator - idiomatic rust
        for &(col, row) in &snake {
            draw_rectangle(
                origin_x + col as f32 * cell_size,
                origin_y + row as f32 * cell_size,
                cell_size - 2.0 ,
                cell_size - 2.0 ,
                GREEN);
        }

        if is_key_pressed(KeyCode::Up) {direction = (0,-1)}
        if is_key_pressed(KeyCode::Down) {direction = (0,1)}
        if is_key_pressed(KeyCode::Left) {direction = (-1,0)}
        if is_key_pressed(KeyCode::Right) {direction = (1,0)}

        if get_time() - last_move >= MOVE_INTERVAL {
            let (current_head_x, current_head_y) = snake[0];
            let next_head = (
                (current_head_x + direction.0 as i32).rem_euclid(cols),
                (current_head_y + direction.1 as i32).rem_euclid(rows)
            );
            snake.insert(0, next_head);
            snake.pop();    

            last_move = get_time(); //updates time
        }

        println!("APPLE X {}", apple_position.0);
        println!("APPLE Y {}", apple_position.1);

        
        
        next_frame().await
    }
}