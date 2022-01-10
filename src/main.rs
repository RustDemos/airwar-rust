use tetra::graphics::{self, Color,Texture};
use tetra::graphics::text::{Font, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};
use rand::prelude::*;

mod hero;
mod bullet;
mod enemy;

use crate::hero::Hero;
use crate::bullet::Bullet;
use crate::enemy::Enemy;

const WINDOW_WIDTH: f32 = 480.0;
const WINDOW_HEIGHT: f32 = 320.0;

const SEED:[u8;32] = [12; 32];

#[derive(PartialEq)]
pub enum Status {
	Normal,
	Pressed,
	Dead,
	Pause,
}

fn main() -> tetra::Result {
    ContextBuilder::new("AirWar", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
        .quit_on_escape(true)
        .fullscreen(false)
        .build()?
        .run(GameState::new)
}

struct GameState {
    hero: Hero,
    bullets: Vec<Bullet>,
	enemies: Vec<Enemy>,
    rnd: StdRng,
    status: Status,
    score: i32,
    text: Text,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {
        let hero_texture = Texture::new(ctx, "./assets/hero.png")?;
        let hero_position = Vec2::new(
            WINDOW_WIDTH/2.0-hero_texture.width() as f32 /4.0,
            WINDOW_HEIGHT - hero_texture.height() as f32 / 2.0 ,
        );

        let hero = Hero::new(hero_texture, hero_position);
        Ok(GameState {
            status: Status::Normal,
            hero: hero,
            bullets: vec![],
            enemies: vec![],
            rnd: SeedableRng::from_seed(SEED),
            score:0,
            text: Text::new(
                "",
                Font::vector(ctx, "./assets/DejaVuSansMono.ttf", 20.0)?,
            )
        })
    }

    fn reset(&mut self) {
        self.enemies.clear();
        self.bullets.clear();
        self.status = Status::Normal;
        self.rnd = SeedableRng::from_seed(SEED);
        self.hero.live();
        self.score = 0;
    }
}

impl State for GameState {
    fn update(&mut self, ctx: &mut Context) -> tetra::Result {
        if self.status != Status::Dead && self.status != Status::Pause {

            if input::is_key_down(ctx, Key::W) {
                self.hero.up();
            }
    
            if input::is_key_down(ctx, Key::S) {
                self.hero.down();
            }
    
            if input::is_key_down(ctx, Key::A) {
                self.hero.left();
            }
    
            if input::is_key_down(ctx, Key::D) {
                self.hero.right();
            }
    
            if input::is_key_down(ctx, Key::Space) {
                //make a bullet
                if self.status == Status::Normal{
                    let hero_position = self.hero.get_position_center();
                    let bullet_texture = Texture::new(ctx, "./assets/bullet.png")?;
                    let b = Bullet::new(bullet_texture, hero_position);
                    self.bullets.push(b);
                    self.status = Status::Pressed;
                }
            }
    
            if input::is_key_released(ctx, Key::Space){
                if self.status == Status::Pressed{
                    self.status = Status::Normal;
                }
            }

            self.bullets.retain(|e| !e.is_dead());
            self.enemies.retain(|e| !e.is_dead());
    
            let positions = vec![0.0,60.0,120.0,180.0,240.0,300.0];
            let max_len = positions.len();
            let x = positions[self.rnd.gen_range(0,max_len)];
            if self.enemies.len() < 3 as usize{
                let texture = Texture::new(ctx, "./assets/enemy.png")?;
                let enemy = Enemy::new(texture,Vec2::new(x,-20.0));
                self.enemies.push(enemy);
            }
    
            for enemy in self.enemies.iter_mut(){
                enemy.update();
            }
    
            for bullet in self.bullets.iter_mut(){
                bullet.update();
            }
    
            //check_collision
            for enemy in self.enemies.iter_mut(){
                if !enemy.is_dead() {
                    let x = enemy.get_position().x as f32;
                    let y = enemy.get_position().y as f32;
                    let w = enemy.get_area().0;
                    let h = enemy.get_area().1;
                    for bullet in self.bullets.iter_mut() {
                        if !bullet.is_dead() {
                            let bx = bullet.get_position().x as f32;
                            let by = bullet.get_position().y as f32;
                            let bw = bullet.get_area().0;
                            let bh = bullet.get_area().1;
                            if w > 0.0 && h > 0.0 && bw > 0.0 && bh > 0.0 && x < bx + bw && x + w > bx && y < by + bh && y + h > by {
                                bullet.hurt();
                                enemy.hurt();
                                self.score += 1;
                            }
                        }
                    }
                }
            }

            //check_collision
            for enemy in self.enemies.iter_mut(){
                if !enemy.is_dead() {
                    let x = enemy.get_position().x as f32;
                    let y = enemy.get_position().y as f32;
                    let w = enemy.get_area().0;
                    let h = enemy.get_area().1;
                    let bx = self.hero.get_position().x as f32;
                    let by = self.hero.get_position().y as f32;
                    let bw = self.hero.get_area().0;
                    let bh = self.hero.get_area().1;
                    if w > 0.0 && h > 0.0 && bw > 0.0 && bh > 0.0 && x < bx + bw && x + w > bx && y < by + bh && y + h > by {
                        self.hero.hurt();
                        enemy.hurt();
                    }
                }
            }

            if self.hero.is_dead() {
                self.status = Status::Dead;
            }

            self.text.set_content(format!(
                "SCORE: {:.2}\n",
                self.score
            ));
            
        }

        if self.status == Status::Dead {
            if input::is_key_down(ctx, Key::R) {
                self.reset();
            }
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        graphics::clear(ctx, Color::rgb(0.0, 0.0, 0.0));

        for enemy in self.enemies.iter_mut(){
			enemy.draw(ctx);
		}

        for bullet in self.bullets.iter_mut(){
			bullet.draw(ctx);
		}

        self.hero.draw(ctx);
        self.text.draw(ctx, Vec2::new(10.0, 10.0));

        Ok(())
    }
}