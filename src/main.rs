use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::rect::Rect;
use std::time::Duration;
use std::path::Path;
mod graphcs;
use graphcs::models::SpaceObject;
use std::collections::HashMap;
use sdl2::keyboard::Keycode;
mod physics;
use physics::physics::force;

fn scale_to_255(orig: f64) -> u8{
    // let temp = orig.parse::<f64>().unwrap();
    let new_val = (orig / 999.0) * 255.0;
    // Convert to u8
    new_val as u8
}
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
    let texture_creator = canvas.texture_creator();
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;
    let font_path: &Path = Path::new(&"/home/martin/development/rusty_space/src/fonts/OpenSans-Bold.ttf");
    let mut font = ttf_context.load_font(font_path, 128)?; 
    font.set_style(sdl2::ttf::FontStyle::BOLD);
    let mut event_pump = sdl_context.event_pump().unwrap();
    // let mut snake = Rect::new(100,100,10,10);
    let mut space_ship =  SpaceObject::new(20,20,2.0,5.0,1.0,Color::RGB(5,5,5));
    space_ship.frozen = false;
    let mut planets: Vec<SpaceObject> = vec![];
    let mut m_of_new_planet: String = "100".to_string();
    // let mut planet = SpaceObject::new(screen_width as i32/2,screen_high as i32/2,0.0,0.0,100.0,Color::RGB(10,50,10));
    let number_keys: HashMap<Keycode, u8> = [
        (Keycode::Num0, 0),
        (Keycode::Num1, 1),
        (Keycode::Num2, 2),
        (Keycode::Num3, 3),
        (Keycode::Num4, 4),
        (Keycode::Num5, 5),
        (Keycode::Num6, 6),
        (Keycode::Num7, 7),
        (Keycode::Num8, 8),
        (Keycode::Num9, 9),
    ].iter().cloned().collect();
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
                if number_keys.contains_key(&keycode)=>{
                    let mut temp_string =number_keys[&keycode].to_string();
                    m_of_new_planet.pop();
                    temp_string.push_str(&m_of_new_planet);
                    m_of_new_planet = temp_string;
                }
                Event::KeyDown {keycode: Some(keycode),..} 
                if keycode == sdl2::keyboard::Keycode::R || 
                keycode == sdl2::keyboard::Keycode::T => {
                    space_ship.do_restart(Some(keycode));
                    planets = vec![];
                }
                Event::MouseButtonDown {  mouse_btn, x, y,.. }=>{
                    if mouse_btn == sdl2::mouse::MouseButton::Left{
                        let m_f64 = m_of_new_planet.parse::<f64>().unwrap();
                        planets.push(SpaceObject::new(x,y ,0.0,0.0,m_f64,Color::RGB(0,scale_to_255(m_f64),0)));
                    }
                }
                _ => {}
            }
        }
        canvas.set_draw_color(Color::RGB(0, 120, 160));
        canvas.fill_rect(Rect::new(0,0,screen_width,screen_high)).ok().unwrap_or_default();
        // space_ship.print_coordnates();
        
        ////////////// TEXT 
        let surface = font
            .render(&m_of_new_planet)
            .blended(Color::RGBA(255,0,0,128))
            .map_err(|e| e.to_string())?;
        let texture = texture_creator
            .create_texture_from_surface(&surface)
            .map_err(|e| e.to_string())?;
        let target_rect = Rect::new(10 as i32, 0 as i32, 200 as u32, 100 as u32);
        canvas.copy(&texture,None,Some(target_rect))?;
        ////////////// TEXT

        space_ship.change_position(screen_width,screen_high);
        // planet.render(&mut canvas);
        if space_ship.frozen == false{
            for planet in planets.iter_mut(){
                let mut temp_planet = planet.copy();
                force(&mut space_ship,&mut temp_planet);
                planet.v_x = temp_planet.v_x;
                planet.v_y = temp_planet.v_y;
                planet.render(&mut canvas);
            }
        }
        else {
            for planet in planets.iter_mut(){
                planet.render(&mut canvas);
            }
        }
        space_ship.render(&mut canvas);
        canvas.present();

        
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }
    Ok(())
}


