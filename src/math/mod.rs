use std::{mem::swap};

pub struct Vec2{
    pub x: f64,
    pub y: f64,
}
impl Vec2{
    pub fn new(x: f64, y: f64) -> Self{
        Vec2{
            x,
            y,
        }
    }
}
impl std::ops::Add for Vec2{
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output{
        Vec2{
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}
impl std::ops::Sub for Vec2{
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output{
        Vec2{
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}
impl std::ops::Mul<f64> for Vec2{
    type Output = Self;
    fn mul(self, rhs: f64) -> Self::Output{
        Vec2{
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}
impl std::ops::Mul<Vec2> for Vec2{
    type Output = f64;
    fn mul(self, rhs: Vec2) -> Self::Output{
        self.x * rhs.x + self.y * rhs.y
    }
}
#[derive(Debug,Clone)]
pub struct Vec3{
    pub x: f64,
    pub y: f64,
    pub z: f64,
}
impl Vec3{
    pub fn new(x: f64, y: f64, z: f64) -> Self{
        Vec3{
            x,
            y,
            z,
        }
    }
    pub fn transform(&self,width:u32,height:u32)->Self{
        Vec3{
            x: (self.x+1.)*width as f64 /2.0,
            y: (-self.y+1.)*height as f64 /2.0,
            z: self.z
        }
    }
}
pub fn to_i32(x:f64)->i32{
    return format!("{:.1$}", x, 0).parse::<i32>().unwrap();
}
#[derive(Debug)]
pub struct LineIter{
    d:f64,
    start:f64,
    end:f64,
    x1:f64,
    x2:f64,
    current:f64,
    xoy:i32
}

impl LineIter{
    pub fn new(x1:f64,y1:f64,x2:f64,y2:f64)->Self{
        let mut x1 = x1;
        let mut y1 = y1;
        let mut x2 = x2;
        let mut y2 = y2;

        let dx = (x1-x2).abs();
        let dy =(y1-y2).abs();


        if dx > dy{
            if x1>x2{
                swap(&mut x1, &mut x2);
                swap(&mut y1, &mut y2);
            }
            LineIter { d:dx, start:x1, end:x2, x1:y1, x2:y2, current:x1-1.0,xoy:0}
        }else{
            if y1>y2{
                swap(&mut x1, &mut x2);
                swap(&mut y1, &mut y2);
            }
            LineIter { d:dy, start:y1, end:y2, x1:x1, x2:x2, current:y1-1.0,xoy:1}
        }

    }
}
impl Iterator for LineIter{
    type Item = (i32,i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.end{
            self.current+=1.0;
            let t = (self.current-self.start).abs()/self.d;
            let x = self.x1 - (self.x1- self.x2) * t;
            if self.xoy==0{
                return Some((to_i32(self.current),to_i32(x)));
            }else {
                return Some((to_i32(x),to_i32(self.current)));
            }
        }else{
            None
        }
    }
}
