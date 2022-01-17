use ggez::{Context, ContextBuilder, GameResult};
use ggez::graphics::{self, Color, Text};
use ggez::event::{self, EventHandler, KeyCode};
use ggez::input::keyboard;

use ggez::graphics::set_window_title;
use ggez::mint::{Point2, Vector2};
use rand::{thread_rng, Rng};


//   Constants   //
//Settings
const WINDOW_TITLE: &str = "Pong!";
//Racket
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_PADDING: f32 = 40.0;
//const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH*0.5;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT*0.5;
const RACKET_SPEED: f32 = 680.0;
//Ball
const BALL_SIDE: f32 = 30.0;
const BALL_SIDE_HALF: f32 = BALL_SIDE*0.5;
const BALL_SPEED: f32 = 220.0;
//Decorations
const LINE_WIDTH: f32 = 4.0;


fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("pong", "ZiZi")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);
    
    //Basic window setup
    set_window_title(&ctx, WINDOW_TITLE);

    // Run!
    event::run(ctx, event_loop, my_game);
}

// Global functions //
//Prevent things going past the screen
fn clamp(number: &mut f32, low: f32, high: f32) {
   if *number < low {
       *number = low;
   } else if *number > high {
       *number = high;
   }
}

fn move_rocket(pos: &mut Point2<f32>, keycode: KeyCode, y_direct: f32, _ctx: &mut Context) {
    let (_, screen_h) = graphics::drawable_size(_ctx);
    let dt = ggez::timer::delta(_ctx).as_secs_f32();
    if keyboard::is_key_pressed(_ctx, keycode) {
        pos.y += y_direct * RACKET_SPEED * dt;
        clamp(&mut pos.y, 0.0, screen_h - RACKET_HEIGHT);
    }
}

fn random_vector(vec: &mut Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    //Return a bool with a probability p of being true.
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

fn racket_hit_ball(ball_pos: &mut Point2<f32>, ball_vel: &mut Vector2<f32>, 
                   racket_pos: &Point2<f32>, screen_w: &f32) {
    if ball_pos.y >= racket_pos.y && ball_pos.y <= racket_pos.y + RACKET_HEIGHT {
        if ball_pos.x + BALL_SIDE >= screen_w - RACKET_WIDTH - RACKET_PADDING {
            ball_vel.x = -ball_vel.x.abs(); //bounce to the left
        } else if ball_pos.x <= 0.0 + RACKET_WIDTH + RACKET_PADDING {
            ball_vel.x = ball_vel.x.abs(); //bounce to the right
        }
    }
/*
 *    let racket_intersects_y = ball_pos.y + BALL_SIDE >= racket_pos.y //ball btm >= racket top
 *        && ball_pos.y <= racket_pos.y + RACKET_HEIGHT; //and ball top <= racket btm
 *
 *    let racket_intersects_x = ball_pos.x + BALL_SIDE >= racket_pos.x //ball right >= racket left
 *        || ball_pos.x < racket_pos.x + RACKET_WIDTH;// and ball left < racket right
 *
 *    if racket_intersects_x && racket_intersects_y {
 *        ball_vel.x = -ball_vel.x;
 *    }
 */
    /*
     *let racket_intersects = ball_pos.x < racket_pos.x + RACKET_WIDTH
     *    && ball_pos.x + BALL_SIDE > racket_pos.x
     *    && ball_pos.y <= racket_pos.y + RACKET_HEIGHT
     *    && ball_pos.y + BALL_SIDE >= racket_pos.y;
     *if racket_intersects {
     *    ball_vel.x = -ball_vel.x;
     *}
     */
}

fn restart_ball(ball_pos: &mut Point2<f32>, ball_vel: &mut Vector2<f32>, _ctx: &mut Context) {
    let (screen_w, screen_h) = graphics::drawable_size(_ctx);
    ball_pos.x = screen_w*0.5;
    ball_pos.y = screen_h*0.5;
    random_vector(ball_vel, BALL_SPEED, BALL_SPEED);
}


struct GameRect {
    position: Point2<f32>,
    thickness: Point2<f32>, //height, width
}

impl GameRect{
    pub fn new(position: Point2<f32>, thickness: Point2<f32>) -> Self {
        GameRect {
            position,
            thickness,
        }
    }
    fn draw(&self, ctx: &mut Context) -> GameResult<()>{
        //TODO optimise mesh generation
        let racket = graphics::Mesh::new_rectangle(
           ctx,
           graphics::DrawMode::fill(),
           graphics::Rect::new(0.0, 0.0, self.thickness.x, self.thickness.y),
           graphics::Color::WHITE,
         )?;
        graphics::draw(ctx, &racket, (self.position,))?;
        Ok(())
    }
}


struct Score {
    player_1: u32,
    player_2: u32,
    text: Text,
    text_pos: Point2 <f32>,
}

impl Score {
    pub fn new(mut text_pos: Point2<f32>, ctx: &mut Context) -> Self {
        let (player_1, player_2) = (0, 0);
        let text = Text::new(format!("{}      {}", player_1, player_2));
        //offset text x position, to center it on the screen
        let text_w = text.width(ctx);
        text_pos.x -= text_w*0.5;
        Score {
            player_1,
            player_2,
            text,
            text_pos,
        }
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.text = Text::new(format!("{}      {}", self.player_1, self.player_2));
        graphics::draw(ctx, &self.text, (self.text_pos,))?;
        Ok(())
    }
}



struct MyGame {
    //  Game state here  //
    //Rackets for players
    player_1_racket: GameRect,
    player_2_racket: GameRect,
    //Ball
    ball: GameRect,
    ball_velocity: Vector2<f32>,
    //Decorations
    score: Score,
    line: GameRect,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        // Load/create resources such as images here.
        //get screen size
        let (screen_w, screen_h) = graphics::drawable_size(_ctx);
        let (screen_w_half, screen_h_half) = (screen_w*0.5, screen_h*0.5);
        //   Initial position, size   //
        let racket_1_pos = Point2 {x: 0.0 + RACKET_PADDING, y: screen_h_half - RACKET_HEIGHT_HALF};
        let racket_2_pos = Point2 {x: screen_w - RACKET_WIDTH - RACKET_PADDING, y: screen_h_half - RACKET_HEIGHT_HALF};
        let racket_thick = Point2 {x: RACKET_WIDTH, y: RACKET_HEIGHT};
        //ball
        let ball_pos = Point2 {x: screen_w_half - BALL_SIDE_HALF, y: screen_h_half - BALL_SIDE_HALF};
        let ball_thick = Point2 {x: BALL_SIDE, y: BALL_SIDE};

        //random starting ball velocity
        let mut ball_velocity = Vector2 {x: 0.0, y: 0.0};
        random_vector(&mut ball_velocity, BALL_SPEED, BALL_SPEED);

        //score position, will be offseted to half of the text width to the left
        let text_pos = Point2 {x: screen_w_half, y: 0.0};

        //center line
        let line_pos = Point2 {x: screen_w_half - LINE_WIDTH*0.5, y: 0.0};
        let line_thick = Point2 {x: LINE_WIDTH , y: screen_w};


        MyGame {
            //Starting game state values
            player_1_racket: GameRect::new(racket_1_pos, racket_thick),
            player_2_racket: GameRect::new(racket_2_pos, racket_thick),
            ball: GameRect::new(ball_pos, ball_thick),
            ball_velocity: ball_velocity,
            score: Score::new(text_pos, _ctx),
            line: GameRect::new(line_pos, line_thick),
        }
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
        let dt = ggez::timer::delta(_ctx).as_secs_f32();
        //Player racket movement
        move_rocket(&mut self.player_1_racket.position, KeyCode::W, -1.0, _ctx);
        move_rocket(&mut self.player_1_racket.position, KeyCode::S, 1.0,  _ctx);
        move_rocket(&mut self.player_2_racket.position, KeyCode::Up, -1.0, _ctx);
        move_rocket(&mut self.player_2_racket.position, KeyCode::Down, 1.0, _ctx);
        //Ball movement
        self.ball.position.x += self.ball_velocity.x * dt;
        self.ball.position.y += self.ball_velocity.y * dt;

        let (screen_w, screen_h) = graphics::drawable_size(_ctx);
        //Score goals, rethrow the ball, if it touches left or right screen border
        if self.ball.position.x < 0.0 {
            restart_ball(&mut self.ball.position, &mut self.ball_velocity, _ctx);
            self.score.player_1 += 1;
        }
        if self.ball.position.x + BALL_SIDE > screen_w {
            restart_ball(&mut self.ball.position, &mut self.ball_velocity, _ctx);
            self.score.player_2 += 1;
        }
        
        //Bounce ball from the top/bottom
        if self.ball.position.y + BALL_SIDE > screen_h  {
            //Prevent clipping
            self.ball.position.y = screen_h - BALL_SIDE;
            self.ball_velocity.y = -self.ball_velocity.y;
        }
        if self.ball.position.y < 0.0 {
            //Prevent clipping
            self.ball.position.y = 0.0;
            self.ball_velocity.y = -self.ball_velocity.y;
        }


        racket_hit_ball(&mut self.ball.position, &mut self.ball_velocity, &self.player_2_racket.position, &screen_w);
        racket_hit_ball(&mut self.ball.position, &mut self.ball_velocity, &self.player_1_racket.position, &screen_w);
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        //Beginning, draw after this
        graphics::clear(ctx, Color::BLACK);
        
        //NOTE now mesh is generated each time, it can be optimised to run only once
        //TODO optimise mesh generation
        self.player_1_racket.draw(ctx)?;
        self.player_2_racket.draw(ctx)?;
        self.line.draw(ctx)?;
        self.ball.draw(ctx)?;
        self.score.draw(ctx)?;

        //End, draw before this
        graphics::present(ctx)?;
        Ok(())
    }
}


