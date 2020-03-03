extern crate core;

mod vector;
mod vertex;
mod matrix;
mod renderer;
mod texture;


use crate::vertex::Vertex;
use crate::vector::Vector;
use crate::renderer::{Renderer, VSOutput};
use crate::matrix::Matrix;
use sdl2::pixels::PixelFormatEnum;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::f32;

use std::time::{Duration, SystemTime};
use crate::texture::Texture;

fn main() -> Result<(), String> {
    let (w, h) = (800, 600);
    let tex = Texture::new("./img.png").expect("无法打开图片");

    let fs = move|f: &Vertex| -> Vector{
        tex.get_color_linear(f.uv.x, f.uv.y)
    };

    let data = vec![
        Vertex {
            pos: Vector::new(-1.0, 0.0, -1.0, 1.0),
            color: Vector::new(1.0, 1.0, 1.0, 1.0),
            normal: Vector::zero(),
            uv: Vector::point(0.0,0.0,0.0),
        },
        Vertex {
            pos: Vector::new(-1.0, 0.0, 1.0, 1.0),
            color: Vector::new(0.0, 1.0, 1.0, 1.0),
            normal: Vector::zero(),
            uv: Vector::point(0.0,0.0,0.0),
        },
        Vertex {
            pos: Vector::new(1.0, 0.0, 1.0, 1.0),
            color: Vector::new(1.0, 1.0, 0.0, 1.0),
            normal: Vector::zero(),
            uv: Vector::point(1.0,0.0,0.0),
        },
        Vertex {
            pos: Vector::new(1.0, 0.0, -1.0, 1.0),
            color: Vector::new(1.0, 0.0, 1.0, 1.0),
            normal: Vector::zero(),
            uv: Vector::point(1.0,0.0,0.0),
        },
        Vertex {
            pos: Vector::new(0.0, 1.0, 0.0, 1.0),
            color: Vector::new(0.0, 1.0, 0.0, 1.0),
            normal: Vector::zero(),
            uv: Vector::point(0.0,1.0,0.0),
        },
    ];

    let indices:&[usize] = &[
        0, 1, 3,
        1, 2, 3,
        1, 2, 4,
        1, 0, 4,
        0, 3, 4,
        3, 2, 4,
    ];

    let mut ren = Renderer::new(w, h);

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("Soft3d With Clip", w as u32, h as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGB24, w as u32, h as u32)
        .map_err(|e| e.to_string())?;


    let mut event_pump = sdl_context.event_pump()?;
    let mut x = 0f32;

    ren.set_fs(fs);
    ren.clear_color(0.5,0.8,1.0);

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running;
                }
                _ => {}
            }
        }
        x += 0.1f32;

        let p = Matrix::perspective(f32::consts::PI * 0.5f32, w as f32 / h as f32, 0.1, 1000.0);
        let view = Matrix::look_at(
           &Vector::point(2f32 * f32::sin(x), 1f32, 2f32 * f32::cos(x)),
            &Vector::point(0.0,1.0,0.0),
            &Vector::vec(0f32, 1f32, 0f32));
        ren.set_vs(move |v: &Vertex| -> VSOutput<Vertex>{
            //let pos = cgmath::Point3::new(v.pos.x,v.pos.y,v.pos.z)*2f32;
            //let pos = (cp * cview).transform_point(pos);
            //let pos = Vector::point(pos.x,pos.y,pos.z);
            VSOutput::new(
                (&p * &view).apply(&v.pos),
                //pos,
                Vertex {
                    pos: v.pos.clone(),
                    color: v.color.clone(),
                    normal: Vector::zero(),
                    uv: v.uv.clone(),
                })
        });

        let sy_time = SystemTime::now();
        ren.clear();
        ren.render_with_index(data.as_slice(),indices);
        let d = SystemTime::now().duration_since(sy_time).unwrap().as_millis();
        canvas.window_mut().set_title(format!("Soft3d With Clip {} ms", d).as_ref());

        ren.get_color_buffer(|buf| {
            texture.update(None, buf, 3 * w);
        });
        canvas.clear();
        canvas.copy(&texture, None, None)?;
        canvas.present();
    }

    Ok(())
}
