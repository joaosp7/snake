use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
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
        for i in 0..cols+1{
            let x = origin_x + cell_size * i as f32;
            draw_line(x, origin_y, x, origin_y + grid_px_h, 2.0, BLUE);
        }
        //horizontal lines
        for i in 1..rows+1{
            let y = origin_y + cell_size * i as f32;
            draw_line(origin_x, y, origin_x + grid_px_w, y, 2.0, BLUE);
        }
        let fps = get_fps();
        println!("Current fps: {}",fps);
       ;
        println!("Screen width: {}", screen_width());
        println!("Screen height : {}", screen_height());
        
        // vertical grid
        //draw_line(delta_width, 0.0, delta_width, 600.00, 2.00, BLUE);

       
        //horizontal grid

        //draw_line(0.0 , delta_height, 800.0, delta_height, 2.00, BLUE);
              //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //draw_ellipse(50.0, 50.0, 15.0, 15.0, 0.0, DARKBLUE);
        //draw_text("HELLO", 20.0, 50.0, 20.0, DARKGRAY)
        
        next_frame().await
    }
}