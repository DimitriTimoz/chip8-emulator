use std::time::Duration;

use sdl2::{video::Window, render::Canvas, pixels::{Color, self}};

extern crate sdl2;

pub struct DisplayDriver {
    canvas: Canvas<Window>,
}

const WIDTH: u32 = 64;
const HEIGHT: u32 = 32;
const PIXEL_SIZE: u32 = 10;

impl DisplayDriver {
    pub fn new(sdl_context: &sdl2::Sdl) -> Result<Self, String>  {
        let video_subsys = sdl_context.video().unwrap();
        let window = video_subsys
            .window(
                "Chip8 Emulator",
                WIDTH * PIXEL_SIZE,
                HEIGHT * PIXEL_SIZE,
            )
            .position_centered()
            .opengl()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Ok(Self {
            canvas,
        })
    }

    pub fn init(&mut self) -> Result<(), String> {
        self.clear_screen()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) -> Result<(), String> {
        self.canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        self.canvas.clear();

        self.canvas.present();
        println!("Screen cleared");
        Ok(())
    }
}