use glam::{Vec2, Vec3};

use crate::object::Object;
use crate::polygon::{ Vertex, Triangle };

pub struct Importer{}

impl Importer {
    pub fn obj(file_path: &str) -> Object {
        use std::io::{BufRead, BufReader};
        use std::fs::File;

        let reader = BufReader::new(File::open(file_path).expect(format!("Cannot open {}", file_path).as_str()));

        let mut verticies: Vec<Vertex> = Vec::new();
        let mut faces: Vec<Triangle> = Vec::new();

        'line: for line in reader.lines() {
            let unwrapped = line.unwrap();
            let mut splitted = unwrapped.split_whitespace();
            let operation = splitted.next().unwrap_or("#"); // Empty lines are considered comments
            let words: Vec<&str> = splitted.collect();
            match operation {
                "#" => { println!("Comment: '{:?}'", words); continue 'line; },
                "v" => {
                    assert_eq!(words.len(), 3, "Non 3D verticies are unsupported");
                    verticies.push(Vertex {
                        pos: Vec3::new(
                            words[0].parse::<f32>().unwrap(),
                            words[1].parse::<f32>().unwrap(),
                            words[2].parse::<f32>().unwrap(),
                        ),
                        uv: Vec2::new(1., 1.),
                    });
                },
                "f" => {
                    assert_eq!(words.len(), 3, "Non triangulated faces are unsupported");
                    faces.push(Triangle { verticies: [
                        verticies[words[0].parse::<usize>().unwrap() - 1].clone(),
                        verticies[words[1].parse::<usize>().unwrap() - 1].clone(),
                        verticies[words[2].parse::<usize>().unwrap() - 1].clone(),
                    ] });
                }
                _ => { println!("Unknown operation '{}'", operation); continue 'line; },
            }
        }

        let object = Object::new(faces);
        return object;
    }
}
