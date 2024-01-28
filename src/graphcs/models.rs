use sdl2::render::Canvas;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::sys::KeyCode;
use sdl2::video::Window;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
pub struct Python{
    pub head: Rect,
    pub v_x: i32,
    pub v_y: i32,
    pub head_color: Color
}

impl Python{    
    pub fn new(x:i32,y:i32,v_x:i32,v_y:i32)->Self{
        Self{
            head: Rect::new(x,y,20,20),
            v_x: v_x,
            v_y: v_y,
            head_color: Color::RGB(5,5,5)
        }
    }
    pub fn render(&self, canvas: &mut Canvas<Window>){
        canvas.set_draw_color(self.head_color);
        canvas.fill_rect(self.head).ok().unwrap_or_default();
    }
    pub fn change_position(&mut self){
        // let temp = self.copy();
        // self.head.set_x(temp.head.x()+temp.v_x);
        self.head.set_x(self.head.x()+self.v_x);
        self.head.set_y(self.head.y()+self.v_y);
    }
    pub fn change_speed(&mut self, keycode: Option<Keycode>){
        if keycode.unwrap() == sdl2::keyboard::Keycode::Right {
            self.v_x += 2;
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Left {
            self.v_x -= 2;
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Up {
            self.v_y -= 2;
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::Down {
            self.v_y += 2;
        }
        else if keycode.unwrap() == sdl2::keyboard::Keycode::R{
            self.head.set_x(100);
            self.head.set_y(100);
        }
    }
    pub fn print_coordnates(&self){
        println!("Head of the snake is currently in: x: {} y: {}",self.head.x(),self.head.y());
    }

    pub fn copy(&self) -> Self{
        Self{head: self.head, v_x: self.v_x, v_y: self.v_y, head_color: self.head_color}
    }
}

