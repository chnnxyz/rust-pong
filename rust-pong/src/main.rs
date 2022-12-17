use macroquad::prelude::*;
use std::{thread, time};

const PLAYER_SIZE: Vec2 = Vec2::from_array([25f32, 130f32]);
const BALL_SIZE: f32 = 25f32;
const PLAYER_SPEED:f32 = 700f32;

// properly define player and implement functions
struct Player {
    rect: Rect,
}

impl Player {
    pub fn new() -> Self {
        Self{
            rect: Rect::new(
                20f32,
                screen_height() * 0.5f32 - PLAYER_SIZE.y *0.5f32,
                PLAYER_SIZE.x,
                PLAYER_SIZE.y,
            )
        }
    }

    // literally draws on screen
    pub fn draw(&self) {        
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }

    // update state
    // dt is a delta time to avoid framerate issues
    pub fn update(&mut self, dt: f32) {
        let mut y_move = 0f32;
        if is_key_down(KeyCode::Up){
            y_move -= 1f32
        }
        if is_key_down(KeyCode::Down){
            y_move += 1f32
        }
        self.rect.y += y_move * dt * PLAYER_SPEED;   
        
        // collision
        if self.rect.y <= 0f32{
            self.rect.y = 0f32
        } 
        if self.rect.y >= screen_height() - self.rect.h {
            self.rect.y = screen_height() - self.rect.h
        }
    }
}

// Rival
struct Rival {
    rect: Rect,
}

impl Rival {
    pub fn new() -> Self {
        Self{
            rect: Rect::new(
                screen_width() - PLAYER_SIZE.x -20f32,
                0f32,
                PLAYER_SIZE.x,
                screen_height(),
            )
        }
    }

    // literally draws on screen
    pub fn draw(&self) {        
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }

    // update state
    // dt is a delta time to avoid framerate issues
    pub fn update(&mut self, dt: f32) {


    }
}

// BALLS
struct Ball {
    rect: Rect,
    vel: Vec2
}

impl Ball {
    pub fn new() -> Self {
        Self{
            rect: Rect::new(
                screen_width() / 2f32 - BALL_SIZE * 0.5f32,
                screen_height() * 0.5f32 - BALL_SIZE *0.5f32,
                BALL_SIZE,
                BALL_SIZE,
            ),
            vel: vec2(
                // 0f32,
                // 1f32
                rand::gen_range(-1f32,1f32),
                rand::gen_range(-1f32,1f32),
            ).normalize()
        }
    }

    // literally draws on screen
    pub fn draw(&self) {        
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }

    // update state
    // dt is a delta time to avoid framerate issues
    pub fn update(&mut self, dt: f32) {
        self.rect.x += self.vel.x * dt * PLAYER_SPEED;
        self.rect.y += self.vel.y * dt * PLAYER_SPEED;

        //collisions
        if self.rect.y <= 0f32 {
            self.vel.y *= -1f32;
        }
        if self.rect.y >= screen_height() - BALL_SIZE{
            self.vel.y *= -1f32;
        }
    }
}

// a dumb net
struct Net {
    rect: Rect
}
impl Net {
    pub fn new() -> Self { 
        Self{           
            rect: Rect::new(
                screen_width() / 2f32 - 1f32,
                0f32,
                2f32,
                screen_height(),
            )  
        }     
    }

    pub fn draw(&self) {
        draw_rectangle(
            self.rect.x, 
            self.rect.y, 
            self.rect.w, 
            self.rect.h, 
            WHITE
        );
    }
}

fn resolve_collision(a: &mut Rect, vel: &mut Vec2, b: &mut Rect) -> bool{
    if let Some(intersection) = a.intersect(*b){
        let a_center = a.center();
        let b_center = b.center();
        let to = b_center - a_center;
        let to_signum = to.signum();
        match intersection.w > intersection.h {
            true =>{
                a.y -= to_signum.y * intersection.h;
                vel.y  = -to_signum.y * vel.y.abs();
            }
            false =>{
                a.x -= to_signum.x * intersection.w;
                vel.x  = -to_signum.x * vel.x.abs();
            }
        }
        return true
    }
    return false
}

fn reset_ball(ball: &mut Ball){
    ball.rect.x = screen_width() * 0.5f32 - BALL_SIZE * 0.5f32;
    ball.rect.y = screen_height() * 0.5f32 - BALL_SIZE * 0.5f32;
    ball.vel.x = rand::gen_range(-1f32,1f32);
    if ball.vel.x <= 0.33f32 {
        // guarantees we dont get shitty x velocities
        ball.vel.x = (ball.vel.x.signum()) *0.33f32;
    }
    ball.vel.y = rand::gen_range(-1f32,1f32);
}

// macro to add async functionality
#[macroquad::main("pong")]
async fn main() {
    let mut score = 0;
    let mut player = Player::new();
    let mut rival = Rival::new();
    let mut ball = Ball::new();
    let net = Net::new();
    // macroquad colors range from 0 to 1
    let bg_color = Color::new(0.37, 0.37, 0.37, 0.0);
    loop {
        //get frametime at any time to update for framedrops
        player.update(get_frame_time());
        ball.update(get_frame_time());
        resolve_collision(&mut ball.rect, &mut ball.vel, &mut player.rect);
        let add_score = resolve_collision(
            &mut ball.rect, 
            &mut ball.vel, 
            &mut rival.rect
        );
        if add_score {
            score +=1
        }
        if ball.rect.x <= -55f32 || ball.rect.x >=screen_width() + 30f32 {
            reset_ball(&mut ball);
            score = 0;
        }
        
        // clear_background take macroquad::Color struct
        clear_background(bg_color);
        player.draw();
        net.draw();
        rival.draw();
        ball.draw();
        draw_text_ex(
            &format!("successful bounces {}", score),
            screen_width() * 0.1f32,
            40f32,
            TextParams{color: WHITE, font_size:30u16, ..Default::default()}
        );
        next_frame().await
    }
}
