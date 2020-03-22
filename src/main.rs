#[macro_use]
extern crate nuuro;

nuuro_header!();

use std::time::Duration;

use nuuro::renderer::{Affine, Renderer};
use nuuro::{App, AppContext, AppInfo, KeyCode};

mod asset_id {
    include!(concat!(env!("OUT_DIR"), "/asset_id.rs"));
}

const ANGULAR_VELOCITY: f64 = 20.;
const COLOR_AMPLITUDE: f64 = 0.5;
const SIZE_AMPLITUDE: f64 = 0.25;

use crate::asset_id::{AssetId, SpriteId};

fn main() {
    let info = AppInfo::with_max_dims(80., 60.)
        .min_dims(64., 48.)
        .title("Nuuro Template")
        .fixed()
        .print_workload_info()
        .print_gl_info();
    nuuro::run(info, GameApp::new());
}

struct Timer(f64);

impl Timer {
    pub fn new() -> Timer {
        Timer(0.)
    }
    pub fn step(&mut self, seconds: f64) {
        self.0 += 1000. * seconds;
    }

    pub fn elapsed(&self) -> Duration {
        Duration::from_millis(self.0 as u64)
    }
}

struct GameApp {
    timer: Timer,
    flash_ratio: f64,
    size: f64,
    angle: f64,
}

impl GameApp {
    pub fn new() -> GameApp {
        GameApp {
            timer: Timer::new(),
            flash_ratio: 0.,
            size: 0.,
            angle: 0.,
        }
    }
}

impl App<AssetId> for GameApp {
    fn start(&mut self, _ctx: &mut AppContext<AssetId>) {
        // Function called on game init
    }

    fn advance(&mut self, seconds: f64, _ctx: &mut AppContext<AssetId>) {
        // Function called on every tick of the game
        self.timer.step(seconds);

        let now = self.timer.elapsed().as_millis() as f64 * 0.01;
        self.flash_ratio = wave(COLOR_AMPLITUDE, ANGULAR_VELOCITY, now) + COLOR_AMPLITUDE;
        self.size = wave(SIZE_AMPLITUDE, ANGULAR_VELOCITY, now) + 1.;
        self.angle += 2. * seconds;
    }

    fn key_down(&mut self, _key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        // Function called when key down input is triggered
    }

    fn key_up(&mut self, _key: KeyCode, _ctx: &mut AppContext<AssetId>) {
        // Function called when key released input is triggered
    }

    fn render(&mut self, renderer: &mut Renderer<AssetId>, ctx: &AppContext<AssetId>) {
        // Function called on render a frame
        renderer.clear((121, 85, 72));

        let affine = &Affine::translate(0.5 * ctx.dims().0, 0.5 * ctx.dims().1)
            .pre_scale(self.size)
            .pre_rotate(self.angle);
        renderer
            .sprite_mode()
            .draw_flash(affine, SpriteId::Nuuro, self.flash_ratio);
    }
}

fn sin(degrees: f64) -> f64 {
    (degrees * std::f64::consts::PI / 180.).sin()
}

fn wave(amplitude: f64, angular_velocity: f64, angle: f64) -> f64 {
    amplitude * sin(angular_velocity * angle)
}
