use std::cell::RefCell;
use std::mem::swap;
use std::f32::INFINITY;
use crate::vector::Vector;
use crate::vertex::Lerp;
use std::marker::PhantomData;

pub struct VSOutput<VA:Lerp>{
    pub pos:Vector,
    pub va:VA,
}

impl<VA:Lerp> VSOutput<VA>{
    pub fn new(pos:Vector,va:VA)->Self{
        VSOutput{
            pos,va,
        }
    }
}

struct Segment<'a, V: Lerp> {
    pub s: (&'a Vector, &'a V),
    pub e: (&'a Vector, &'a V),
}

impl<'a, V: Lerp> Segment<'a, V> {
    pub fn new(a: (&'a Vector, &'a V), b: (&'a Vector, &'a V)) -> Self {
        if a.0.y < b.0.y {
            Self {
                s: a,
                e: b,
            }
        } else {
            Self {
                s: b,
                e: a,
            }
        }
    }

    pub fn length(&self) -> f32 {
        (self.e.0 - self.s.0).length()
    }

    pub fn length_y(&self) -> f32 {
        (self.e.0.y - self.s.0.y).abs()
    }

    pub fn length_x(&self) -> f32 {
        (self.e.0.x - self.s.0.x).abs()
    }
}

#[derive(Clone, Copy)]
enum Plane {
    NX = 0,
    X = 1,
    NY = 2,
    Y = 3,
    NZ = 4,
    Z = 5,
}

impl From<u8> for Plane {
    fn from(a: u8) -> Self {
        match a {
            0 => Plane::NX,
            1 => Plane::X,
            2 => Plane::NY,
            3 => Plane::Y,
            4 => Plane::NZ,
            5 => Plane::Z,
            _ => panic!("Unknown value: {}", a),
        }
    }
}

impl From<Plane> for u8 {
    fn from(a: Plane) -> Self {
        a as u8
    }
}

impl Plane{
    fn next(self)->Option<Plane>{
        let u = u8::from(self) + 1;
        if u >= 6{
            None
        }else{
            Some(Plane::from(u))
        }
    }
}

// V is Vertex attributes
pub struct Renderer<VS, FS, V: Lerp> where
    VS: Fn(&V) -> VSOutput<V>,
    FS: Fn(&V) -> Vector
{
    width: usize,
    height: usize,
    clear_color:[u8;3],

    vertex_shader: Option<VS>,
    fragment_shader: Option<FS>,

    color_buffer: RefCell<Vec<u8>>,
    depth_buffer: RefCell<Vec<f32>>,
    _phantom: PhantomData<V>,
}


impl<VS, FS, V> Renderer<VS, FS, V> where
    VS: Fn(&V) -> VSOutput<V>,
    FS: Fn(&V) -> Vector,
    V: Lerp
{
    pub fn new(w: usize, h: usize) -> Self {
        Renderer {
            width: w,
            height: h,
            clear_color: [0u8;3],

            vertex_shader: None,
            fragment_shader: None,

            color_buffer: RefCell::new(vec![0u8; w * h * 3]),
            depth_buffer: RefCell::new(vec![-INFINITY; w * h]),

            _phantom: PhantomData {},
        }
    }

    pub fn set_vs(&mut self,vs:VS){
        self.vertex_shader = Some(vs)
    }

    pub fn set_fs(&mut self,fs:FS){
        self.fragment_shader = Some(fs)
    }

    pub fn clear_color(&mut self,r:f32,g:f32,b:f32){
        self.clear_color = [(r * 255f32) as u8,(g * 255f32) as u8,(b * 255f32) as u8];
    }

    pub fn get_color_buffer<F>(&self, mut cb: F)
        where F: FnMut(&[u8])
    {
        cb(self.color_buffer.borrow_mut().as_slice())
    }

    pub fn clear(&self) {
        let mut i = 0;
        for c in self.color_buffer.borrow_mut().iter_mut() {
            *c = self.clear_color[i];
            i = (i + 1) % self.clear_color.len()
        }

        for d in self.depth_buffer.borrow_mut().iter_mut() {
            *d = INFINITY;
        }
    }

    pub fn render(&self, vertices: &[V]) {
        let mut data:Vec<VSOutput<V>> = vertices.iter().map(|x| {
            (self.vertex_shader.as_ref().unwrap())(&x)
        }).collect();

        for i in (0..data.len() / 3).map(|x| x * 3) {
            let p0 = data.get(i).unwrap();
            let p1 = data.get(i + 1).unwrap();
            let p2 = data.get(i + 2).unwrap();
            self.draw_triangle(p0, p1, p2,true);
        }
    }

    pub fn render_with_index(&self, vertices: &[V], indices: &[usize]) {
        let mut data: Vec<VSOutput<V>> = vertices.iter().map(|x| {
            (self.vertex_shader.as_ref().unwrap())(&x)
        }).collect();

        for i in (0..indices.len() / 3).map(|x| x * 3) {
            let p0 =data.get(indices[i]).unwrap();
            let p1 = data.get(indices[i+1]).unwrap();
            let p2 = data.get(indices[i+2]).unwrap();
            self.draw_triangle(p0, p1, p2,true);
        }
    }

    fn draw_triangle(&self, p0: &VSOutput<V>, p1: &VSOutput<V>, p2: &VSOutput<V>,clip:bool) {
        if clip {
            let cc0 = Self::check_cvv(&p0.pos);
            let cc1 = Self::check_cvv(&p1.pos);
            let cc2 = Self::check_cvv(&p2.pos);

            let cc_and = cc0 & cc1 & cc2;

            //三个点全在某个平面之外
            if cc_and != 0 {
                return;
            }

            let cc_or = cc0 | cc1 | cc2;

            //有顶点在裁剪空间外
            if cc_or != 0 {
                self.clip_triangle(p0, p1, p2,Self::find_next_clip_plane(0, cc_or));
                return;
            }
        }

        //透视除法
        let pos0 = Self::perspective_div(&p0.pos);
        let pos1 = Self::perspective_div(&p1.pos);
        let pos2 = Self::perspective_div(&p2.pos);

        //到NDC
        let pos0 = self.to_ndc(&pos0);
        let pos1 = self.to_ndc(&pos1);
        let pos2 = self.to_ndc(&pos2);

        let v0 = (&pos0, &p0.va);
        let v1 = (&pos1, &p1.va);
        let v2 = (&pos2, &p2.va);

        let s1 = Segment::<V>::new(v0, v1);
        let s2 = Segment::<V>::new(v0, v2);
        let s3 = Segment::<V>::new(v1, v2);

        let mut tss: Vec<Segment<V>> = vec![s1, s2, s3];
        //tss[0]长度最长
        tss.sort_by(|a, b| b.length_y().partial_cmp(&a.length_y()).or(Some(std::cmp::Ordering::Equal)).unwrap());

        self.rasterize(&tss[0], &tss[1]);
        self.rasterize(&tss[0], &tss[2]);
    }

    fn clip_triangle(&self, p0: &VSOutput<V>, p1: &VSOutput<V>, p2: &VSOutput<V>, plane: Option<Plane>) {
        let cc0 = Self::check_cvv(&p0.pos);
        let cc1 = Self::check_cvv(&p1.pos);
        let cc2 = Self::check_cvv(&p2.pos);

        let cc_or = cc0 | cc1 | cc2;

        if plane.is_none() || cc_or == 0{
            self.draw_triangle(p0,p1,p2,false);
            return;
        }

        let plane = plane.unwrap();
        let plane = Self::find_next_clip_plane(plane as u8,cc_or);
        let plane = match plane{
            None=>{
                self.draw_triangle(p0,p1,p2,false);
                return;
            },
            Some(plane)=>plane
        };

        let plane_mask = 1 << u8::from(plane);
        let cc_xor = (cc0 ^ cc1 ^ cc2) & plane_mask;
        let mut tvs: Vec<&VSOutput<V>> = Vec::with_capacity(3);

        if cc_xor == 0 {
            //有两个顶点在当前裁剪平面外
            //tvs[0]在平面内
            if (cc0 & plane_mask) == 0 {
                tvs.push(p0);
                tvs.push(p1);
                tvs.push(p2);
            } else if (cc1 & plane_mask) == 0 {
                tvs.push(p1);
                tvs.push(p2);
                tvs.push(p0);
            } else {
                tvs.push(p2);
                tvs.push(p0);
                tvs.push(p1);
            }

            let t1 = Self::compute_t_on_clip_plane(&tvs[0].pos, &tvs[1].pos, plane);
            let t2 = Self::compute_t_on_clip_plane(&tvs[0].pos, &tvs[2].pos, plane);

            let pos01 = Vector::lerp(&tvs[0].pos, &tvs[1].pos, t1);
            let pos02 = Vector::lerp(&tvs[0].pos, &tvs[2].pos, t2);
            let v01 = V::lerp(&tvs[0].va, &tvs[1].va, t1);
            let v02 = V::lerp(&tvs[0].va, &tvs[2].va, t2);
            let p01 = VSOutput::new(pos01, v01);
            let p02 = VSOutput::new(pos02, v02);

            self.clip_triangle(tvs[0], &p01, &p02,plane.next());
        } else {
            //有一个顶点在当前裁剪平面外
            //tvs[0]在平面外
            if (cc0 & plane_mask) > 0 {
                tvs.push(p0);
                tvs.push(p1);
                tvs.push(p2);
            } else if (cc1 & plane_mask) > 0 {
                tvs.push(p1);
                tvs.push(p2);
                tvs.push(p0);
            } else {
                tvs.push(p2);
                tvs.push(p0);
                tvs.push(p1);
            }

            let t1 = Self::compute_t_on_clip_plane(&tvs[1].pos, &tvs[0].pos, plane);
            let t2 = Self::compute_t_on_clip_plane(&tvs[2].pos, &tvs[0].pos, plane);

            let pos10 = Vector::lerp(&tvs[1].pos, &tvs[0].pos, t1);
            let pos20 = Vector::lerp(&tvs[2].pos, &tvs[0].pos, t2);
            let v10 = V::lerp(&tvs[1].va, &tvs[0].va, t1);
            let v20 = V::lerp(&tvs[2].va, &tvs[0].va, t2);
            let p10 = VSOutput::new(pos10, v10);
            let p20 = VSOutput::new(pos20, v20);

            let np = plane.next();
            self.clip_triangle(tvs[2], &p20, tvs[1], np);
            self.clip_triangle(tvs[1], &p20, &p10, np);
        }
    }

    fn compute_t_on_clip_plane(s: &Vector, e: &Vector, plane: Plane) -> f32 {
        match plane {
            Plane::NX => (s.x + s.w) / (s.x - e.x + s.w - e.w),
            Plane::X => (s.x - s.w) / (s.x - e.x - s.w + e.w),
            Plane::NY => (s.y + s.w) / (s.y - e.y + s.w - e.w),
            Plane::Y => (s.y - s.w) / (s.y - e.y - s.w + e.w),
            Plane::NZ => (s.z + s.w) / (s.z - e.z + s.w - e.w),
            Plane::Z => (s.z - s.w) / (s.z - e.z - s.w + e.w),
        }
    }

    fn rasterize(&self, s1: &Segment<V>, s2: &Segment<V>) {
        let y_start = s2.s.0.y as usize;
        let y_end = s2.e.0.y as usize;

        for y in y_start..y_end {
            let fy = y as f32;
            let s1ey = s1.e.0.y.floor();
            let s1sy = s1.s.0.y.floor();
            let s2ey = s2.e.0.y.floor();
            let s2sy = s2.s.0.y.floor();

            let t1 = (fy - s2sy) / (s2ey - s2sy);
            let t2 = (fy - s1sy) / (s1ey - s1sy);

            let mut xp_start = (
                Vector::lerp(&s2.s.0, &s2.e.0, t1),
                V::lerp(&s2.s.1, &s2.e.1, t1)
            );
            let mut xp_end = (
                Vector::lerp(&s1.s.0, &s1.e.0, t2),
                V::lerp(&s1.s.1, &s1.e.1, t2)
            );

            if xp_start.0.x > xp_end.0.x {
                swap(&mut xp_start, &mut xp_end);
            }

            let x_start = xp_start.0.x as usize;
            let x_end = xp_end.0.x as usize;
            let x_len = (xp_end.0.x - xp_start.0.x).floor();

            for x in x_start..x_end {
                let fx = x as f32;
                let t = (fx - xp_start.0.x.floor()) / x_len;
                let p = (
                    Vector::lerp(&xp_start.0, &xp_end.0, t),
                    V::lerp(&xp_start.1, &xp_end.1, t)
                );

                let color = (self.fragment_shader.as_ref().unwrap())(&p.1);
                //let pos = self.to_ndc(&p.0);
                if self.set_depth(x as usize, y as usize, p.0.z) {
                    self.set_color(x as usize, y as usize, &color);
                }
            }
        }
    }

    #[inline]
    fn set_depth(&self, x: usize, y: usize, depth: f32) -> bool {
        let pos = self.width * y + x;
        let mut db = self.depth_buffer.borrow_mut();

        if depth < db[pos] {
            db[pos] = depth;
            return true;
        }
        false
    }

    #[inline]
    fn set_color(&self, x: usize, y: usize, color: &Vector) {
        let pos = (self.width * y + x) * 3;
        let mut cb = self.color_buffer.borrow_mut();
        let (r, g, b) = ((color.x * 255f32) as u8, (color.y * 255f32) as u8, (color.z * 255f32) as u8);
        cb[pos + 0] = r;
        cb[pos + 1] = g;
        cb[pos + 2] = b;
    }

    //透视除法
    fn perspective_div(v: &Vector) -> Vector {
        v.scale(1f32 / v.w)
    }

    fn to_ndc(&self, v: &Vector) -> Vector {
        let nx = (v.x + 1f32) * 0.5f32 * self.width as f32;
        let ny = (-v.y + 1f32) * 0.5f32 * self.height as f32;
        Vector::point(nx, ny, v.z)
    }

    fn find_next_clip_plane(s: u8, code: u8) -> Option<Plane> {
        for b in s..7 {
            if (code & (1 << b)) > 0 {
                return Some(Plane::from(b));
            }
        }
        return None;
    }

    fn check_cvv(p: &Vector) -> u8 {
        let mut c = 0u8;
        if p.x < -p.w {
            c = c | (1 << u8::from(Plane::NX));
        }

        if p.x > p.w {
            c = c | (1 << u8::from(Plane::X));
        }

        if p.y < -p.w {
            c = c | (1 << u8::from(Plane::NY));
        }

        if p.y > p.w {
            c = c | (1 << u8::from(Plane::Y));
        }

        if p.z < -p.w {
            c = c | (1 << u8::from(Plane::NZ));
        }

        if p.z > p.w {
            c = c | (1 << u8::from(Plane::Z));
        }

        c
    }
}