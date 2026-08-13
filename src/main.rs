use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]

async fn main() {
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

        
        next_frame().await
    }
}