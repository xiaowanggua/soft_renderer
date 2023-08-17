use std::mem::swap;

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
#[derive(Debug,Clone,Copy)]
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
#[derive(Debug,Clone)]
pub struct LineIter{
    pub start: i32,
    y: f64,
    d: f64,
    current: i32,
    counter: i32,
    len: i32,
    adder: i32,
    xoy: bool,
}

impl LineIter{
    pub fn new(x1:f64,y1:f64,x2:f64,y2:f64,force:bool)->Self{
        let mut x1 = x1;
        let mut y1 = y1;
        let mut x2 = x2;
        let mut y2 = y2;

        if y1 > y2{
            swap(&mut x1, &mut x2);
            swap(&mut y1, &mut y2);
        }

        let ix1 = to_i32(x1);
        let iy1 = to_i32(y1);
        let ix2 = to_i32(x2);
        let iy2 = to_i32(y2);
        let dx = (x1-x2).abs();
        let dy = (y1-y2).abs();

        if dx >= dy && !force{
            if x1 > x2{
                LineIter{
                    start: ix1,
                    y: y1,
                    d: (y2-y1)/(x2-x1),
                    current: ix1 + 1,
                    counter: 0,
                    len: (ix1-ix2).abs(),
                    adder: -1,
                    xoy: true,
                }
            }else{
                LineIter{
                    start: ix1,
                    y: y1,
                    d: (y2-y1)/(x2-x1),
                    current: to_i32(x1) - 1,
                    counter: 0,
                    len: (ix1 - ix2).abs(),
                    adder: 1,
                    xoy: true,
                }
            }

        }else{
            LineIter{
                start: iy1,
                y: x1,
                d: (x2-x1)/(y2-y1),
                current: iy1 - 1,
                counter: 0,
                len: (iy1 - iy2).abs(),
                adder: 1,
                xoy: false,
            }
        }
    }
}

impl Iterator for LineIter{
    type Item = (i32,i32);
    fn next(&mut self) -> Option<Self::Item> {
        if self.counter <= self.len{
            self.counter += 1;
            self.current += self.adder;
            let y = self.d * (self.current - self.start) as f64 + self.y;
            if self.xoy{
                Some((self.current,to_i32(y)))
            }else{
                Some((to_i32(y),self.current))
            }
        }else{
            None
        }
    }
}