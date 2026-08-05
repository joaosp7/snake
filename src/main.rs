use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
async fn main() {
    let delta_height = 150.0;
    let delta_width = 133.3333;
    let mut counter = 0;
    loop {
        //initial screen height -> 600
        // initial screen width -> 800
        let fps = get_fps();
        println!("Current fps: {}",fps);
        clear_background(RED);
        draw_rectangle(0.0, 0.0, 800.0, 600.0, GREEN);

        draw_rectangle(400.0, 300.0, 20.0, 20.0, BLUE);
        println!("Screen width: {}", screen_width());
        println!("Screen height : {}", screen_height());
        
        // vertical grid
        draw_line(delta_width, 0.0, delta_width, 600.00, 2.00, BLUE);

        draw_line(2.0 * delta_width, 0.0, 2.0 * delta_width, 600.00, 2.00, BLUE);

        draw_line(3.0 * delta_width, 0.0, 3.0 * delta_width, 600.00, 2.00, BLUE);

        draw_line(4.0 * delta_width, 0.0, 4.0 * delta_width, 600.00, 2.00, BLUE);

        draw_line(5.0 * delta_width, 0.0, 5.0 * delta_width, 600.00, 2.00, BLUE);
        
        //horizontal grid

        draw_line(0.0 , delta_height, 800.0, delta_height, 2.00, BLUE);
        draw_line(0.0 , 2.0 * delta_height, 800.0, 2.0 * delta_height, 2.00, BLUE);
        draw_line(0.0 , 3.0 * delta_height, 800.0, 3.0 * delta_height, 2.00, BLUE);
        draw_line(0.0 , 4.0 * delta_height, 800.0, 4.0 * delta_height, 2.00, BLUE); 
        //draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        //draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        //draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        //draw_ellipse(50.0, 50.0, 15.0, 15.0, 0.0, DARKBLUE);
        //draw_text("HELLO", 20.0, 50.0, 20.0, DARKGRAY);
        println!("Current Counter is: {}", counter);
        counter+= 1;
        next_frame().await
    }
}