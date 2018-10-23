//! The simplest possible example that does something.

extern crate ggez;
extern crate rand;

use ggez::conf;
use ggez::event;
use ggez::graphics::{self, DrawMode, Point2};
use ggez::{Context, GameResult};
use ggez::timer;
use rand::{thread_rng, Rng};

const GRAVITY: f32 = 0.1;

#[derive(Debug)]
struct Cricle {
    radius: f32,
    point:Point2,
    velocity:Point2,
    is_active:bool,
    color:graphics::Color,
}

impl Cricle {
    fn new(radius:f32,x:f32,y:f32,vel:(f32,f32),color:graphics::Color) -> Cricle{
        Cricle{
            radius:radius,
            is_active:true,
            velocity:Point2::new(vel.0,vel.1),
            point: Point2::new(x,y),
            color:color,
            }
    }

    fn next(&mut self){
        self.velocity.y += GRAVITY;
        self.point.x += self.velocity.x;
        self.point.y += self.velocity.y;
    }

    fn reset(&mut self,radius:f32,x :f32,y:f32,vel:(f32,f32),color:graphics::Color){
        self.radius = radius;
        self.is_active = true;
        self.velocity.x = vel.0;
        self.velocity.y = vel.1;
        self.point.x = x;
        self.point.y = y;
        self.color = color;
    }
    fn disable(&mut self){
        self.is_active = false;
    }

    fn draw(&self,_ctx:&mut Context) ->GameResult<()>{
        graphics::circle(
            _ctx,
            DrawMode::Fill,
            self.point,
            self.radius,
            0.2,
        )?;
        graphics::set_color(_ctx, self.color)?;
        Ok(())
    }
}

struct MainState {
    rng : rand::ThreadRng,
    circle_list : Vec<Cricle>,
}

impl MainState {
    fn new(_ctx: &mut Context) -> GameResult<MainState> {
        let rng = thread_rng();
        let s = MainState {rng:rng, circle_list:vec![] };
        Ok(s)
    }
    fn random_color(&mut self) -> graphics::Color{
        graphics::Color::new(self.rng.gen_range(0f32,1f32),self.rng.gen_range(0f32,1f32),self.rng.gen_range(0f32,1f32),1.0)
    }

    fn random_vel(&mut self) ->(f32,f32){
        (self.rng.gen_range(-5f32,5f32),self.rng.gen_range(-5f32,2f32))
    }
    fn random_radius(&mut self) -> f32{
        self.rng.gen_range(5f32,15f32)
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        self.circle_list.iter_mut().filter(|c| c.is_active).for_each(|c| c.next());
        let window_size = graphics::get_size(_ctx);
        self.circle_list.iter_mut().for_each(| c| if c.point.x > window_size.0 as f32|| c.point.x < 0.0 || c.point.y > window_size.1 as f32 { c.disable();});
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::Color::new(1.0,1.0,1.0,1.0));
        for c in self.circle_list.iter().filter(|c| c.is_active){
            c.draw(ctx)?;
        }
        graphics::present(ctx);
        timer::yield_now();
        Ok(())
    }

    fn mouse_button_up_event(&mut self, _ctx: &mut Context, button: event::MouseButton, x: i32, y: i32) {
        if button == event::MouseButton::Left{
            for _i in 0..5{
            let color = self.random_color();
            let vel = self.random_vel();
            let radius = self.random_radius();
            if let Some(c) = self.circle_list.iter_mut().find(|item| !item.is_active){
                println!("reuse circle");
                c.reset(radius,x as f32,y as f32,vel,color);
                continue;
            }
            println!("create new circle :{}",self.circle_list.len());
            self.circle_list.push(Cricle::new(radius,x as f32,y as f32,vel,color));
            }
            
        }
    }
    fn resize_event(&mut self, ctx: &mut Context, width: u32, height: u32) {
        let new_rect = graphics::Rect::new(0.0, 0.0, width as f32, height as f32);
        graphics::set_screen_coordinates(ctx, new_rect).unwrap();
    }
}

pub fn main() {
    let c = conf::Conf::new();
    let ctx = &mut Context::load_from_conf("falling ball", "ggez", c).unwrap();
    let state = &mut MainState::new(ctx).unwrap();
    event::run(ctx, state).unwrap();
}
