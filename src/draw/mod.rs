use std::mem::swap;

use sdl2::{pixels::Color, rect::Point};

use crate::SoftRenderer;
use crate::math::*;

impl SoftRenderer{

    pub fn draw_line(&mut self,x1:f64,y1:f64,x2:f64,y2:f64,color:Color){
        self.canvas.set_draw_color(color);
        if x1==x2 && y1==y2{
            self.canvas.draw_point(Point::new(to_i32(x1),to_i32(y1))).unwrap();
            return;
        }else{
            let line = LineIter::new(x1,y1,x2,y2,false);
            for i in line{
                let (x,y) = i;
                self.canvas.draw_point(Point::new(x,y)).unwrap();
            }
        }
    }

    pub fn draw_triangle(&mut self,v1:Vec3,v2:Vec3,v3:Vec3,color:Color){
        if v1.x == v2.x && v2.x == v3.x {return;}
        self.canvas.set_draw_color(color);
        let mut v1 = v1;
        let mut v2 = v2;
        let mut v3 = v3;
        //println!("{:?},{:?},{:?}",v1,v2,v3);
        if v2.y > v1.y{
            swap(&mut v1, &mut v2);
        }
        if v3.y > v1.y{
            swap(&mut v1, &mut v3);
        }
        if v3.y > v2.y{
            swap(&mut v3, &mut v2);
        }

        let height = v1.y - v3.y;
        let mut current: f64 = 0.;
        while current < height{
            let k1 = current/height;
            if v3.y + current <= v2.y{
                let k2;
                if v2.y == v3.y{k2 = 1.;}else{k2 =current/(v2.y - v3.y);};
                let mut x1 = to_i32(v3.x + k1 * (v1.x - v3.x));
                let mut x2: i32 = to_i32(v3.x + k2 * (v2.x - v3.x));
                if x2 < x1{
                    swap(&mut x1,&mut x2);
                }
                for i in x1..x2{
                    self.canvas.draw_point(Point::new(i,to_i32(v3.y + current))).unwrap();
                }
            }else{
                let k2;
                if v1.y == v2.y{k2 = 1.;}else{k2 = (current - (v2.y - v3.y))/(v1.y - v2.y);};
                let mut x1 = to_i32(v3.x + k1 * (v1.x - v3.x));
                let mut x2 = to_i32(v2.x + k2 * (v1.x - v2.x));
                if x2 < x1{
                    swap(&mut x1,&mut x2);
                }
                for i in x1..x2{
                    self.canvas.draw_point(Point::new(i,to_i32(v3.y + current))).unwrap();
                }
            }
            current+=1.;
        }
    }

}
