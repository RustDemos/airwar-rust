use tetra::math::Vec2;
use tetra::graphics::{DrawParams,Texture};
use tetra::{Context};

#[derive(Debug)]
pub struct Hero{
	position: Vec2<f32>,
	scale: Vec2<f32>,
	texture: Texture,
	speed: f32,
	dead: bool,
	area: (f32,f32)
}

impl Hero{
	pub fn new(texture: Texture,position: Vec2<f32>) -> Self{
		let scale = Vec2::new(0.5, 0.5);
		let speed = 2.0;
		let area = (texture.width() as f32 * 0.5, texture.height() as f32 * 0.5);
		let dead = false;
		Hero{
			texture,
			position,
			scale,
			speed,
			area,
			dead
		}
	}

	pub fn get_area(&self) -> (f32,f32){
		self.area
	}

	pub fn is_dead(&self) -> bool{
		self.dead
	}

	pub fn live(&mut self) {
		self.dead = false;
	}

	pub fn hurt(&mut self){
        self.dead = true;
	}

	pub fn up(&mut self){
		self.position.y -= self.speed;
	}
	pub fn down(&mut self){
		self.position.y += self.speed;
	}
	pub fn left(&mut self){
		self.position.x -= self.speed;
	}
	pub fn right(&mut self){
		self.position.x += self.speed;
	}

	pub fn get_position(&self) -> Vec2<f32>{
		self.position
	}

	pub fn get_position_center(&self) -> Vec2<f32>{
		Vec2::new(self.position.x + self.texture.width() as f32 /4.0 , self.position.y + self.texture.height() as f32 /4.0)
	}
	

	pub fn draw(&mut self,ctx: &mut Context){
		self.texture.draw(ctx, 
            DrawParams::new()
                .position(self.position)
                .scale(self.scale),
        );
	}
}