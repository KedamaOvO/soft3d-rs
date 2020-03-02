use crate::vector::Vector;
use std::ops::{Mul, Index, IndexMut};
use std::fmt;
use std::fmt::{Formatter, Error};

#[derive(PartialEq,Clone)]
pub struct Matrix{
    m:[Vector;4],
}
impl fmt::Display for Matrix{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,"[{},{},{},{}]",self.m[0],self.m[1],self.m[2],self.m[3])
    }
}

impl fmt::Debug for Matrix{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f,"[{},{},{},{}]",self.m[0],self.m[1],self.m[2],self.m[3])
    }
}


impl Index<usize> for Matrix{
    type Output = Vector;

    fn index(&self, index: usize) -> &Self::Output {
        &self.m[index]
    }
}

impl IndexMut<usize> for Matrix{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.m[index]
    }
}

impl<'a,'b> Mul<&'a Matrix> for & 'b Matrix{
    type Output = Matrix;

    fn mul(self,rhs:&'a Matrix)->Matrix{
        let rx = Vector::new(rhs[0].x,rhs[1].x,rhs[2].x,rhs[3].x);
        let ry = Vector::new(rhs[0].y,rhs[1].y,rhs[2].y,rhs[3].y);
        let rz = Vector::new(rhs[0].z,rhs[1].z,rhs[2].z,rhs[3].z);
        let rw = Vector::new(rhs[0].w,rhs[1].w,rhs[2].w,rhs[3].w);
        Matrix{
            m:[
                Vector::new(self[0].dot(&rx),self[0].dot(&ry),self[0].dot(&rz),self[0].dot(&rw)),
                Vector::new(self[1].dot(&rx),self[1].dot(&ry),self[1].dot(&rz),self[1].dot(&rw)),
                Vector::new(self[2].dot(&rx),self[2].dot(&ry),self[2].dot(&rz),self[2].dot(&rw)),
                Vector::new(self[3].dot(&rx),self[3].dot(&ry),self[3].dot(&rz),self[3].dot(&rw)),
            ]
        }
    }
}

impl Matrix{
    pub fn apply(&self,v:&Vector)->Vector{
        Vector::new(
            self[0].dot(v),
            self[1].dot(v),
            self[2].dot(v),
            self[3].dot(v),
        )
    }
}

impl Matrix{
    pub fn perspective(fov:f32,aspect:f32,near:f32,far:f32)->Matrix{
        let tan_inv = 1f32 / f32::tan(fov*0.5f32);
        let nsf = near - far;

        Matrix{
            m:[
                Vector::new(tan_inv / aspect,0.0,0.0,0.0),
                Vector::new(0.0,tan_inv,0.0,0.0),
                Vector::new(0.0,0.0,(near + far)/nsf,(2f32*near*far)/nsf),
                Vector::new(0.0,0.0,-1.0,0.0),
            ]
        }
    }

    pub fn look_at(eye:&Vector,target:&Vector,up:&Vector)->Matrix{
        let zaxis = (target-eye).normalize();
        let xaxis = zaxis.cross(&up).normalize();
        let yaxis = xaxis.cross(&zaxis).normalize();
        let px = xaxis.dot(eye);
        let py = yaxis.dot(eye);
        let pz = zaxis.dot(eye);

        Matrix{
            m:[
                Vector::new(xaxis.x,xaxis.y,xaxis.z,-px),
                Vector::new(yaxis.x,yaxis.y,yaxis.z,-py),
                Vector::new(-zaxis.x,-zaxis.y,-zaxis.z,pz),
                Vector::new(0.0,0.0,0.0,1.0),
            ]
        }
    }
}

#[cfg(test)]
mod test{
    use crate::matrix::Matrix;
    use crate::vector::Vector;

    #[test]
    fn test_apply(){
        let a = Matrix{
            m:[
                Vector::new(1.0,2.0,3.0,4.0),
                Vector::new(5.0,6.0,7.0,8.0),
                Vector::new(1.0,2.0,3.0,4.0),
                Vector::new(5.0,6.0,7.0,8.0),
            ]
        };
        let v = Vector::new(1.0,2.0,3.0,4.0);
        let c = Vector::new(30.0,70.0,30.0,70.0);
        assert_eq!(c,a.apply(&v));
    }

    #[test]
    fn test_mul(){
        let a = Matrix{
            m:[
                Vector::new(1.0,2.0,3.0,4.0),
                Vector::new(5.0,6.0,7.0,8.0),
                Vector::new(9.0,10.0,11.0,12.0),
                Vector::new(13.0,14.0,15.0,16.0),
            ]
        };

        let b = Matrix{
            m:[
                Vector::new(17.0,18.0,19.0,20.0),
                Vector::new(21.0,22.0,23.0,24.0),
                Vector::new(25.0,26.0,27.0,28.0),
                Vector::new(29.0,30.0,31.0,32.0),
            ]
        };

        let c = Matrix{
            m:[
                Vector::new(250.0,260.0,270.0,280.0),
                Vector::new(618.0,644.0,670.0,696.0),
                Vector::new(986.0,1028.0,1070.0,1112.0),
                Vector::new(1354.0,1412.0,1470.0,1528.0),
            ]
        };

        assert_eq!(c,&a*&b);
    }
}