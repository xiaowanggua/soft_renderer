use std::mem::swap;

use sdl2::{pixels::Color, rect::Point};

use crate::SoftRenderer;
use crate::math::*;

impl SoftRenderer{

    pub fn draw_line(&mut self,x1:f64,y1:f64,x2:f64,y2:f64,color:Color){
        self.canvas.set_draw_color(color);
        let mut x1 = to_i32(x1);
        let mut y1 = to_i32(y1);
        let mut x2 = to_i32(x2);
        let mut y2 = to_i32(y2);

        let dx = (x1-x2).abs();
        let dy =(y1-y2).abs();

        if dx > dy{
            if x1>x2{
                swap(&mut x1, &mut x2);
                swap(&mut y1, &mut y2);
            }
            for i in x1..x2{
                let t = (i-x1).abs() as f64/dx as f64;
                let y = y1 as f64 - (y1- y2) as f64 * t;
                self.canvas.draw_point(Point::new(i,to_i32(y))).unwrap();
            }
        }else{
            if y1>y2{
                swap(&mut x1, &mut x2);
                swap(&mut y1, &mut y2);
            }
            for i in y1..y2{
                let t = (i-y1).abs() as f64/dy as f64;
                let x = x1 as f64 - (x1- x2) as f64 * t;
                self.canvas.draw_point(Point::new(to_i32(x),i)).unwrap();
            }
        }
    }

    pub fn draw_triangle(&mut self,v1:Vec3,v2:Vec3,v3:Vec3,color:Color){
        let to_i32 = |temp_x: f64|{
            return format!("{:.1$}", temp_x, 0).parse::<i32>().unwrap();
        };
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
        let max_dy = to_i32(v1.y - v3.y).abs();
        for i in to_i32(v3.y)..to_i32(v1.y){
            
        }
    }
}
