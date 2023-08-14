use learnrenderer::*;
use learnrenderer::math::Vec3;
use learnrenderer::model::*;
use sdl2::pixels::Color;

pub fn main() {
    let width = 600;
    let height = 600;
    let m = Model::new(".\\african_head.obj");
    let mut sr = SoftRenderer::new("learn",width,height);
    loop {
        sr.clear();
        for triangle in &m.triangles{
            let v0 = m.vertexes[triangle[0] as usize].transform(width, height);
            let v1 = m.vertexes[triangle[1] as usize].transform(width, height);
            let v2 = m.vertexes[triangle[2] as usize].transform(width, height);
            //draw triangle with v0 v1 v2 with random color
            sr.draw_triangle(v0,v1,v2,Color::RGB(rand::random::<u8>(),rand::random::<u8>(),rand::random::<u8>()));
        }
        sr.render();
        if sr.if_close_window(){
            break;
        }
        std::thread::sleep(std::time::Duration::from_secs(1)/60);
    }
}