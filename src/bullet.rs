use tetra::math::Vec2;
use tetra::graphics::{DrawParams,Texture};
use tetra::{Context};

#[derive(Debug)]
pub struct Bullet{
	position: Vec2<f32>,
	scale: Vec2<f32>,
	texture: Texture,
	speed: f32,
    dead: bool,
    area: (f32,f32)
}

impl Bullet{
	pub fn new(texture: Texture,position: Vec2<f32>) -> Self{
		let scale = Vec2::new(0.5, 0.5);
		let speed = 5.0;
        let dead = false;
        let area = (texture.width() as f32 * 0.5, texture.height() as f32 * 0.5);
		Bullet{
			texture,
			position,
			scale,
			speed,
            dead,
            area
		}
	}

    pub fn get_area(&self) -> (f32,f32){
		self.area
	}

    pub fn is_dead(&self) -> bool{
		self.dead
	}

    pub fn hurt(&mut self){
        self.dead = true;
	}

    pub fn get_position(&self) -> Vec2<f32>{
		self.position
	}

	pub fn update(&mut self){
		self.position.y -= self.speed;
        if self.position.y <= 0.0{
            self.dead = true;
        }
	}

    pub fn draw(&mut self,ctx: &mut Context){
		self.texture.draw(ctx, 
            DrawParams::new()
                .position(self.position)
                .scale(self.scale),
        );
	}
}