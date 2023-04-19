use chrono::{DateTime, Local, format, Duration};
use macroquad::{
    prelude::Color,
    shapes::draw_rectangle,
    window::{clear_background, next_frame},
};
const MY_FONT: &[u8] = include_bytes!("./font/SpaceMono-Bold.ttf");
use macroquad::prelude::*;
use macroquad_text::Fonts;
struct Person {
    score: usize,
    name: String,
    time_started: DateTime<Local>,
    time_ended: DateTime<Local>,
}

impl Person {
    fn new(score: usize, name: &String) -> Person {
        Person {
            score,
            name: name.to_string(),
            time_started: Local::now(),
            time_ended: Local::now(),
        }
    }
}

struct Bar {
    x: f32,
    y: f32,
    l: f32,
    w: f32,
    speedx: f32,
    speedy: f32,
    color: Color,
}

struct Ball {
    x: f32,
    y: f32,
    r: f32,
    speedx: f32,
    speedy: f32,
    color: Color,
}

impl Bar {
    fn new(x: f32, y: f32, l: f32, w: f32, speedx: f32, speedy: f32, color: Color) -> Bar {
        Bar {
            x,
            y,
            l,
            w,
            speedx,
            speedy,
            color,
        }
    }
}

impl Ball {
    fn new(x: f32, y: f32, r: f32, speedx: f32, speedy: f32, color: Color) -> Ball {
        Ball {
            x,
            y,
            r,
            speedx,
            speedy,
            color,
        }
    }
}

#[macroquad::main("pong_single")]
async fn main() {
    let mut bar = Bar::new(
        (screen_width() - 200.0) / 2.0,
        1.6 * screen_height(),
        200.0,
        10.0,
        3.0,
        2.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    let mut ball = Ball::new(
        (screen_width() - 200.0) / 2.0,
        80.0,
        15.0,
        2.0,
        3.0,
        Color::new(1.0, 1.0, 1.0, 1.0),
    );
    let mut player = Person::new(0, &"Pradyumna".to_string());
    let mut background_color = Color::new(0.1, 0.2, 0.3, 0.4);
    let mut fonts = Fonts::default();
    fonts.load_font_from_bytes(MY_FONT).unwrap();
    let mut ended: bool = false;
    loop {
        clear_background(background_color);
        let dur = player.time_ended - player.time_started;
        let (sec, min, hrs) = (dur.num_seconds(), dur.num_minutes(), dur.num_hours());
        let time_string = format!("{}:{}:{}", hrs, min, sec);
        // .format("%H:%M:%S").to_string();
        let text = "Player: ".to_owned()
            + &player.name.to_owned()
            + " | Score: "
            + &player.score.to_string()
            + " | Time Elapsed: "
            + &time_string;

        draw_rectangle(0.0, 0.0, screen_width(), 50.0, BLACK);
        fonts.draw_text(&text, 20.0, 0.0, 30, Color::from([1.0; 4]));
        
        if !ended {
            draw_rectangle(bar.x, bar.y, bar.l, bar.w, bar.color);
            draw_circle(ball.x, ball.y, ball.r, ball.color);
            println!("{:?}", text);
            // Move bar right
            if is_key_down(KeyCode::Right) {
                if bar.x + 200.0 + bar.speedy < screen_width() {
                    bar.x += bar.speedx
                }
            }
            // Move bar left
            if is_key_down(KeyCode::Left) {
                if bar.x > bar.speedy {
                    bar.x -= bar.speedx
                }
            }
            // Move ball down. If it touches the bar, reverse speed. Same if it goes above the screen.
            if ball.y + ball.r > bar.y && (ball.x > bar.x && ball.x < bar.x + bar.l) {
                ball.speedy *= -1.0;
                player.score += 1;
            } else if ball.y - ball.r < 50.0 {
                ball.speedy *= -1.0;
            }
            // If ball falls below make background red
            else if ball.y + ball.r > bar.y {
                background_color = RED;
                ball.color = RED;
                ended = true;
            }

            // Move ball right. If it touches the wall, reverse speed.
            if ball.x + ball.r > screen_width() || ball.x - ball.r < 0.0 {
                ball.speedx *= -1.0;
            }
            // Move the ball horizontally
            ball.y += ball.speedy;
            ball.x += ball.speedx;
            player.time_ended = Local::now();
        }
        next_frame().await;
    }
}
