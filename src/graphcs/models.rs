use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::sys::KeyCode;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub struct SpaceObject{
    pub head: Rect,
    pub v_x: f64,
    pub v_y: f64,
    pub react_x: f64,
    pub react_y: f64,
    pub m: f64,
    pub frozen: bool,
    pub head_color: Color
}


impl SpaceObject{    
    pub fn new(x:i32,y:i32,v_x:f64,v_y:f64,m:f64,col: Color)->Self{
        Self{
            head: Rect::new(x,y,20,20),
            v_x: v_x,
            v_y: v_y,
            react_x: x as f64,
            react_y: y as f64,
            m: m,
            frozen: true,
            head_color: col//Color::RGB(5,5,5)
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.head_color);
        canvas.fill_rect(self.head).ok().unwrap_or_default();
    }
    pub fn change_position(&mut self,screen_width: u32,screen_high:u32){
        if self.react_x+self.v_x < 0.0{
            self.v_x *= -1.0
        }
        else if self.react_x+self.v_x > screen_width as f64 {
            self.v_x *= -1.0
        }
        else if self.react_y+self.v_y < 0.0{
            self.v_y *= -1.0
        }
        else if self.react_y+self.v_y > screen_high as f64 {
            self.v_y *= -1.0
        }
        self.react_x+=self.v_x;
        self.react_y+=self.v_y;
        self.head.set_x((self.react_x) as i32);
        self.head.set_y((self.react_y) as i32);
    }
    pub fn change_speed(&mut self, keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::Right {
            if self.v_x < 10.0{
                self.v_x += 2.0;
            }
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Left {
            if self.v_x > -10.0 {
                self.v_x -= 2.0;
            }
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Up {
            if self.v_y > -10.0 {
                self.v_y -= 2.0;
            }
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Down {
            if self.v_y < 10.0{
                self.v_y += 2.0;
            }
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Space{
            self.v_x = 0.0;
            self.v_y = 0.0;
            self.frozen = !self.frozen;
        }

    }
    pub fn do_restart(&mut self,keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::R{
            self.react_x = 100.0;
            self.head.set_x(100);
            self.react_y = 100.0;
            self.head.set_y(100);
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::T{
            self.react_x = 100.0;
            self.head.set_x(100);
            self.v_x = 0.0;
            self.react_y = 100.0;
            self.head.set_y(100);
            self.v_y = 0.0;
        }
    }
    pub fn print_coordnates(&self){
        println!("Head of the snake is currently in: x: {} y: {} {} {}",self.head.x(),self.head.y(),self.v_x,self.v_y);
    }

    pub fn copy(&self) -> Self{
        Self{head: self.head, 
            v_x: self.v_x, 
            v_y: self.v_y, 
            react_x: self.react_x, 
            react_y: self.react_y, 
            m:self.m,
            frozen: self.frozen,
            head_color: self.head_color}
    }
}

