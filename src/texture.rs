use image::{ImageError, DynamicImage, GenericImageView};
use crate::vector::Vector;

pub struct Texture{
    pub image:DynamicImage
}

impl Texture{
    pub fn new(path:&str)->Result<Self,ImageError>{
        image::open(path).map(|image|{
            Texture{
                image
            }
        })
    }

    #[inline]
    fn get_color(&self,x:u32,y:u32)->Vector{
        let x = x % self.image.width();
        let y = y % self.image.height();
        let c = self.image.get_pixel(x,y);
        Vector::new(
            c.0[0] as f32/ 255f32,
            c.0[1] as f32/ 255f32,
            c.0[2] as f32/ 255f32,
            c.0[3] as f32/ 255f32,
        )
    }

    #[inline]
    pub fn get_color_nearest(&self,x:f32,y:f32)->Vector{
        let ix = (x*self.image.width() as f32) as u32;
        let iy = ((1f32 - y)*self.image.height() as f32) as u32;
        self.get_color(ix,iy)
    }

    pub fn get_color_linear(&self, x:f32, y:f32) ->Vector {
        let fx = x * self.image.width() as f32;
        let fy = (1f32 - y) * self.image.height() as f32;
        let ffx = fx.floor();
        let ffy = fy.floor();
        let dx = fx - ffx;// dx => (0 - 1)
        let dy = fy - ffy;

        let ix0 = ffx as u32;
        let iy0 = ffy as u32;
        let ix1 = ix0 + 1;
        let iy1 = iy0+ 1;

        let c00 = self.get_color(ix0,iy0);
        let c10 = self.get_color(ix1,iy0);
        let c01 = self.get_color(ix0,iy1);
        let c11 = self.get_color(ix1,iy1);

        let cx0 = &c00 + &((&c10-&c00).scale(dx));
        let cx1 = &c01 + &((&c11-&c01).scale(dx));
        &cx0 + &((&cx1-&cx0).scale(dy))
    }

}