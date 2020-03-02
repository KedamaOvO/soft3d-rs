extern crate core;

mod vector;
mod vertex;
mod matrix;
mod renderer;


use crate::vertex::Vertex;
use crate::vector::Vector;
use crate::renderer::Renderer;
use crate::matrix::Matrix;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f32;

use std::time::{Duration, SystemTime};

fn main() -> Result<(), String> {
    let (w,h) = (800,600);

    let fs = |f:&Vertex|->Vector{
        f.color.clone()
    };

    let data = vec![
        Vertex{
            pos: Vector::new(-8.0,-1.0,-2.0,1.0),
            color: Vector::new(1.0,0.0,0.0,1.0),
            normal: Vector::zero(),
            uv: Vector::zero(),
        },
        Vertex{
            pos: Vector::new(1.0,-1.0,-2.0,1.0),
            color: Vector::new(0.0,1.0,0.0,1.0),
            normal: Vector::zero(),
            uv: Vector::zero(),
        },
        Vertex{
            pos: Vector::new(0.0,1.0,-2.0,1.0),
            color: Vector::new(0.0,0.0,1.0,1.0),
            normal: Vector::zero(),
            uv: Vector::zero(),
        },
    ];

    let mut ren = Renderer::new(w,h);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let mut window = video_subsystem.window("Soft3d With Clip", w as u32, h as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, w as u32, h as u32)
        .map_err(|e| e.to_string())?;


    let mut event_pump = sdl_context.event_pump()?;
    let mut x =0f32;

    ren.set_fs(fs);

    'running: loop{
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        x+=0.1f32;
        let p = Matrix::perspective(f32::consts::PI * 0.5f32,w as f32/h as f32,0.1,1000.0);
        let view = Matrix::look_at(&Vector::point(3f32*f32::sin(x),0f32,3f32*f32::cos(x)),&Vector::zero(),&Vector::vec(0f32,1f32,0f32));
        ren.set_vs(move|v:&Vertex|->(Vector,Vertex){
            (
                (&view*&p).apply(&v.pos),
                Vertex{
                    pos: v.pos.clone(),
                    color: v.color.clone(),
                    normal: Vector::zero(),
                    uv: Vector::zero(),
                })
        });

        let sy_time = SystemTime::now();
        ren.clear();
        ren.render(data.as_slice());
        let d = SystemTime::now().duration_since(sy_time).unwrap().as_millis();
        canvas.window_mut().set_title(format!("Soft3d With Clip {} ms", d).as_ref());

        ren.get_color_buffer(|buf|{
            texture.update(None,buf,3 * w);
        });
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}