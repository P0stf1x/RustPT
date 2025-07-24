use std::sync::{Arc, RwLock};

use crate::{aabb::AABB, helper::polygon::Triangle,};

#[derive(Debug, Clone)]
pub enum BVHNode {
    Leaf(Arc<RwLock<Triangle>>),
    Node(Box<BVH>),
}

#[derive(Debug, Clone)]
pub struct BVH {
    pub aabb: AABB,
    pub left: BVHNode,
    pub right: BVHNode,
}

impl BVH {
    pub fn generate_bottom(tris: &Vec<Arc<RwLock<Triangle>>>) -> Self {
        println!("Triangles got into BVH generator: {}", tris.len());
        if tris.is_empty() {
            panic!("Cannot build BVH with zero triangles");
        }

        let mut nodes: Vec<BVHNode> = tris.iter().map(|tri| BVHNode::Leaf(tri.clone())).collect();
        println!("Converted {} triangles into {} leaves", tris.len(), nodes.len());
        let mut leaves_processed = 0;

        while nodes.len() > 2 {
            let mut next_level = Vec::new();
            let mut iter = nodes.into_iter();

            while let Some(left) = iter.next() {
                match iter.next() {
                    Some(right) => {
                        let left_aabb = match &left {
                            BVHNode::Leaf(tri) => {leaves_processed += 1; AABB::new_from_tri(&vec![(tri.read().unwrap()).clone()])},
                            BVHNode::Node(node) => node.aabb.clone(),
                        };
                        let right_aabb = match &right {
                            BVHNode::Leaf(tri) => {leaves_processed += 1; AABB::new_from_tri(&vec![(tri.read().unwrap()).clone()])},
                            BVHNode::Node(node) => node.aabb.clone(),
                        };
                        let parent = BVHNode::Node(
                            Box::new(BVH {
                                aabb: left_aabb.expand(&right_aabb),
                                left,
                                right,
                            })
                        );
                        next_level.push(parent);
                    }
                    None => {
                        next_level.push(left);
                    }
                }
            }
            nodes = next_level;
        };

        println!("Processed leaves while building BVH: {}", leaves_processed);
        println!("final nodes length: {}", nodes.len());

        if tris.len() <= 2 { // FIXME: quick hack that doesn't work for a case with 2 tris
            return BVH {
                aabb: AABB::new_from_tri(&vec![tris[0].read().unwrap().clone()]),
                left: BVHNode::Leaf(tris[0].clone()),
                right: BVHNode::Leaf(tris[0].clone()),
            }
        }

        let final_left = nodes.pop().unwrap();
        let final_right = nodes.pop().unwrap();
        if let BVHNode::Node(bvh_left) = &final_left {
            if let BVHNode::Node(bvh_right) = &final_right {
                let aabb = bvh_left.aabb.expand(&bvh_right.aabb);
                return BVH {
                    aabb,
                    left: final_left,
                    right: final_right,
                };
            }
        };
        panic!("Error building BVH")
    }
}
