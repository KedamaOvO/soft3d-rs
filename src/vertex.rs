use super::Vector;

pub trait VertexAttribute {
    fn lerp(a: &Self, b: &Self, t: f32) -> Self;
    fn scale(&self, s: f32) -> Self;
}

pub struct Vertex{
    pub pos:Vector,
    pub color:Vector,
    pub normal:Vector,
    pub uv:Vector,
}

impl VertexAttribute for Vertex{
    fn lerp(a:&Vertex,b:&Vertex,t:f32)->Vertex{
        Vertex{
            pos: Vector::lerp(&a.pos,&b.pos,t),
            color: Vector::lerp(&a.color,&b.color,t),
            normal: Vector::lerp(&a.normal,&b.normal,t),
            uv: Vector::lerp(&a.uv,&b.uv,t),
        }
    }

    fn scale(&self, s: f32) -> Self {
        Vertex{
            pos: self.pos.scale(s),
            color: self.color.scale(s),
            normal: self.normal.scale(s),
            uv: self.uv.scale(s),
        }
    }
}