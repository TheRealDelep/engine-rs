use sdl2::{pixels, rect::Rect, render::WindowCanvas};

use crate::{Point, ToPoint, Vec2};

pub type Color = pixels::Color;

pub struct GraphicsPipeline {
    pub options: GraphicsOptions,
    pub camera: Camera,
    canvas: WindowCanvas,
}

pub struct GraphicsOptions {
    pub pixel_per_unit: u32,
    pub window_size: (u32, u32),
}

#[derive(Default)]
pub struct Camera {
    pub position: Vec2,
}

impl GraphicsPipeline {
    pub fn new(options: GraphicsOptions, canvas: WindowCanvas) -> Self {
        GraphicsPipeline {
            options,
            canvas,
            camera: Camera::default(),
        }
    }

    pub fn draw_rect(&mut self, position: &Vec2, size: &Vec2, color: &Color, filled: bool) {
        self.canvas.set_draw_color(*color);

        let center = Vec2::new(position.x - (size.x / 2.), position.y - (size.y / 2.));

        let pos = self.camera.get_screen_coordinate(self, &center);
        let rect = Rect::new(
            pos.x,
            pos.y,
            (size.x * self.options.pixel_per_unit as f64) as u32,
            (size.y * self.options.pixel_per_unit as f64) as u32,
        );

        if filled {
            self.canvas.fill_rect(rect).unwrap();
        } else {
            self.canvas.draw_rect(rect).unwrap();
        }
    }

    pub fn world_to_screen_position(&self, position: &Vec2) -> Point {
        Point::new(
            (position.x * self.options.pixel_per_unit as f64).round() as i32
                + self.options.window_size.0 as i32 / 2,
            (position.y * self.options.pixel_per_unit as f64).round() as i32
                + self.options.window_size.1 as i32 / 2,
        )
    }

    pub fn run(&mut self) {
        self.canvas.present();
        self.canvas.clear();
    }
}

impl Camera {
    pub fn get_screen_coordinate(
        &self,
        graphics_ppl: &GraphicsPipeline,
        world_coordinate: &Vec2,
    ) -> Point {
        let relative_pos = world_coordinate - self.position;
        graphics_ppl.world_to_screen_position(&relative_pos)
    }
}
