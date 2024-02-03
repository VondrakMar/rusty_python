use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;

mod graphcs;
use graphcs::models::SpaceObject;
use std::collections::HashSet;

mod physics;
use physics::physics::force;
fn main() -> Result<(),String> {
    let screen_width = 1200;
    let screen_high = 1200;
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
    let mut space_ship =  SpaceObject::new(20,20,2.0,5.0,1.0,Color::RGB(5,5,5));
    let mut planets: Vec<SpaceObject> = vec![];
    // let mut planet = SpaceObject::new(screen_width as i32/2,screen_high as i32/2,0.0,0.0,100.0,Color::RGB(10,50,10));

    let mut prev_keys = HashSet::new();
    'running: loop {
        canvas.clear();
        for event in event_pump.poll_iter(){
            match event{
                Event::Quit {..} => {
                    break 'running;
                }
                Event::KeyDown {keycode: Some(keycode),..} 
                if keycode == sdl2::keyboard::Keycode::Down || 
                keycode == sdl2::keyboard::Keycode::Up || 
                keycode == sdl2::keyboard::Keycode::Left ||
                keycode == sdl2::keyboard::Keycode::Right ||
                keycode == sdl2::keyboard::Keycode::Space => {  
                    space_ship.change_speed(Some(keycode));
                }
                Event::KeyDown {keycode: Some(keycode),..} 
                if keycode == sdl2::keyboard::Keycode::R || 
                keycode == sdl2::keyboard::Keycode::T => {
                    // Handle A, C, or D keydown event
                    println!("{:?} key pressed", keycode);
                    // Modify objectA here=>{
                    space_ship.do_restart(Some(keycode));
                }
                Event::MouseButtonDown {  mouse_btn, x, y,.. }=>{
                    if mouse_btn == sdl2::mouse::MouseButton::Left{
                        planets.push(SpaceObject::new(x,y ,0.0,0.0,100.0,Color::RGB(10,50,10)));
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 120, 160));
        canvas.fill_rect(Rect::new(0,0,screen_width,screen_high)).ok().unwrap_or_default();
        // space_ship.print_coordnates();
        
        // planet.change_position(screen_width,screen_high); 
        
        space_ship.change_position(screen_width,screen_high);
        // force(&mut space_ship,&mut planet);
        // planet.render(&mut canvas);
        for planet in planets.iter(){
            planet.render(&mut canvas);
        }
        space_ship.render(&mut canvas);
        canvas.present();
        ///////////////////////////////
        let keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .filter_map(sdl2::keyboard::Keycode::from_scancode)
            .collect();

        // Get the difference between the new and old sets.
        let new_keys = &keys - &prev_keys;
        let old_keys = &prev_keys - &keys;

        if !new_keys.is_empty() || !old_keys.is_empty() {
            println!("new_keys: {:?}\told_keys:{:?}", new_keys, old_keys);
        }

        prev_keys = keys;
        ///ssssssssssssssssssssssssss 
        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


