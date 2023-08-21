use graphics::GraphicsOptions;
use inputs::InputScheme;

pub mod graphics;
pub mod inputs;
pub mod physics;

pub type Vec2 = parry2d_f64::math::Vector<f64>;
pub type Point = parry2d_f64::math::Vector<i32>;

pub trait ToPoint {
    fn to_point(&self) -> Point;
}

pub trait ToVec2 {
    fn to_vec2(&self) -> Vec2;
}

pub struct Engine<T>
where
    T: InputScheme,
{
    pub graphics_ppl: graphics::GraphicsPipeline,
    pub inputs_ppl: inputs::InputsPipeline<T>,
}

impl ToPoint for Vec2 {
    fn to_point(&self) -> Point {
        Point::new(self.x.round() as i32, self.y.round() as i32)
    }
}

impl<T> Engine<T>
where
    T: InputScheme,
{
    pub fn new(game_title: &str, graphics_options: GraphicsOptions) -> Self {
        let ctx = sdl2::init().unwrap();

        // Setup GrahicsPipeline
        let video_subsystem = ctx.video().unwrap();
        let window = video_subsystem
            .window(
                game_title,
                graphics_options.window_size.0,
                graphics_options.window_size.1,
            )
            .position_centered()
            .build()
            .unwrap();

        let canvas = window.into_canvas().build().unwrap();
        let graphics_ppl = graphics::GraphicsPipeline::new(graphics_options, canvas);

        // Setup InputsPipeline
        let event_pump = ctx.event_pump().unwrap();
        let inputs_ppl = inputs::InputsPipeline::new(event_pump);

        Engine {
            graphics_ppl,
            inputs_ppl,
        }
    }
}
