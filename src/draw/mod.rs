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
            let line = LineIter::new(x1,y1,x2,y2);
            for i in line{
                let (x,y) = i;
                self.canvas.draw_point(Point::new(x,y)).unwrap();
            }
        }
    }

    pub fn draw_triangle(&mut self,v1:Vec3,v2:Vec3,v3:Vec3,color:Color){
        self.canvas.set_draw_color(color);
        let mut v1 = v1;
        let mut v2 = v2;
        let mut v3 = v3;
        if v2.y > v1.y{
            swap(&mut v1, &mut v2);
        }
        if v3.y > v1.y{
            swap(&mut v1, &mut v3);
        }
        if v3.y > v2.y{
            swap(&mut v3, &mut v2);
        }
        let mut line1 = LineIter::new(v1.x,v1.y,v3.x,v3.y);
        let mut line2 = LineIter::new(v2.x,v2.y,v1.x,v1.y);
        let mut line3 = LineIter::new(v3.x,v3.y,v2.x,v2.y);

        for i in to_i32(v3.y)..to_i32(v1.y){
            if i > to_i32(v2.y){
                let (mut x1,y1) = line1.next().unwrap();
                let (mut x2,y2) = line2.next().unwrap();
                if x1 > x2{
                    swap(&mut x1, &mut x2);
                }
                if y1 != y2{
                    println!("{} {} {} {}",x1,y1,x2,y2);
                }
                for j in x1..x2{
                    self.canvas.draw_point(Point::new(j,y1)).unwrap();
                }
            }else{
                let (mut x1,y1) = line1.next().unwrap();
                let (mut x2,_) = line3.next().unwrap();
                if x1 > x2{
                    swap(&mut x1, &mut x2);
                }
                for j in x1..x2{
                    self.canvas.draw_point(Point::new(j,y1)).unwrap();
                }
            }
        }
    }
}
