use std::{fs::File, io::Read};

use crate::math::*;
pub struct Model{
    pub vertexes : Vec<Vec3>,
    pub triangles : Vec<[i32;3]>
}
impl Model{
    pub fn new(filename:&str)->Self{
        let mut vertexes = Vec::new();
        let mut triangles = Vec::new();
        let mut file = File::open(filename).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        for line in contents.lines(){
            let mut iter = line.split_whitespace();
            //println!("{:?}",iter);
            match iter.next(){
                Some("v") => {
                    let x = iter.next().unwrap().parse::<f64>().unwrap();
                    let y = iter.next().unwrap().parse::<f64>().unwrap();
                    let z = iter.next().unwrap().parse::<f64>().unwrap();
                    //println!("{:?}",[x,y,z]);
                    vertexes.push(Vec3::new(x,y,z));
                },
                Some("f") => {
                    let x = iter.next().unwrap().split("/").next().unwrap().parse::<i32>().unwrap() - 1;
                    let y = iter.next().unwrap().split("/").next().unwrap().parse::<i32>().unwrap() - 1;
                    let z = iter.next().unwrap().split("/").next().unwrap().parse::<i32>().unwrap() - 1;
                    //println!("{:?}",[x,y,z]);
                    triangles.push([x,y,z]);
                },
                _ => (),
            }
        }
        Model{
            vertexes,
            triangles,
        }
    }
}
#[cfg(test)]
mod tests{
    use super::*;
    #[test]
    fn test_model(){
        let m = Model::new("D:\\workspace\\rust\\learnrenderer\\african_head.obj");
    }
}