use soft_renderer::*;
use soft_renderer::math::Vec3;
use soft_renderer::model::*;
use sdl2::pixels::Color;
use sdl2::rect::Point;

pub fn main() {
    let width = 600;
    let height = 600;
    let m = Model::new(".\\african_head.obj");
    let mut sr = Renderer::new("learn",width,height);
    loop {
        sr.clear();
        for triangle in &m.triangles{
            //let triangle = mm.next().unwrap();
            let v0 = m.vertexes[triangle[0] as usize].transform(width, height);
            let v1 = m.vertexes[triangle[1] as usize].transform(width, height);
            let v2 = m.vertexes[triangle[2] as usize].transform(width, height);
            sr.draw_triangle(v0,v1,v2,Color::WHITE);
            //sr.draw_line(v0.x,v0.y,v1.x,v1.y,Color::WHITE);
            //sr.draw_line(v1.x,v1.y,v2.x,v2.y,Color::WHITE);
            //sr.draw_line(v2.x,v2.y,v0.x,v0.y,Color::WHITE);
        }
        //sr.draw_triangle(Vec3::new(0.,100.,0.),Vec3::new(0.,0.,0.),Vec3::new(200.,0.,0.),Color::WHITE);
        sr.render();
        if sr.if_close_window(){
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1));
    }
}