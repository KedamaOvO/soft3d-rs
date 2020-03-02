use super::Vector;

pub trait Lerp{
     fn lerp(a:&Self,b:&Self,t:f32)->Self;
}

pub struct Vertex{
    pub pos:Vector,
    pub color:Vector,
    pub normal:Vector,
    pub uv:Vector,
}

impl Lerp for Vertex{
    fn lerp(a:&Vertex,b:&Vertex,t:f32)->Vertex{
        Vertex{
            pos: Vector::lerp(&a.pos,&b.pos,t),
            color: Vector::lerp(&a.color,&b.color,t),
            normal: Vector::lerp(&a.normal,&b.normal,t),
            uv: Vector::lerp(&a.uv,&b.uv,t),
        }
    }
}