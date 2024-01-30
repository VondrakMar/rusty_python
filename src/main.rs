use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;

mod graphcs;
use graphcs::models::Python;

mod physics;
use physics::physics::force;
fn main() -> Result<(),String> {
    let screen_width = 800;
    let screen_high = 800;
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem.window("rusty_python",screen_width,screen_high)
        .position_centered()
        .build()
        .unwrap();
    let mut canvas = window.into_canvas()
        .build()
        .unwrap();    
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut snake = Rect::new(100,100,10,10);
    let mut snake = Python::new(20,20,4,4,Color::RGB(5,5,5));
    let mut planet = Python::new(400,400,0,0,Color::RGB(10,50,10));
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit {..} => {
                    break 'running;
                }
                Event::KeyDown {keycode,..}=>{
                    snake.change_speed(keycode);
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 120, 160));
        canvas.fill_rect(Rect::new(0,0,screen_width,screen_high)).ok().unwrap_or_default();
        // snake.set_x(snake.x() + move_horizont);
        // snake.set_y(snake.y() + move_vertical);
        // canvas.set_draw_color(Color::RGB(0,0,10));
        // canvas.fill_rect(snake).ok().unwrap_or_default();
        snake.print_coordnates();
        planet.change_position(screen_width,screen_high); 
        snake.change_position(screen_width,screen_high);
        force(&mut snake,&mut planet);
        planet.render(&mut canvas);
        snake.render(&mut canvas);
        canvas.present();
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


