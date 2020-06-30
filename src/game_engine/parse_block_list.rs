use crate::game_engine::shapes::{Block, BlockBuilder, ShapeKind};
use std::fs;
use crate::RED;

pub fn parse_block_list(contents: String, point_list: &mut Vec<Vec<f64>>) -> Vec<Block> {
    let mut blocks: Vec<Block> = Vec::new();
    let mut b: BlockBuilder = BlockBuilder::empty();
    contents.lines().for_each(|line| {
        if line.starts_with("-") {
            let f_values: Vec<String> = line.split("-").map(|s| s.trim().to_string()).collect();
            let s = f_values[1].to_owned();
            b = BlockBuilder::from_str(s);
        } else if line == "#" {
            let b_built = b.build();
            blocks.push(b_built);
            b = BlockBuilder::empty();
        } else {
            let f_values: Vec<String> = line.split(",").map(|s| s.trim().to_string()).collect();
            let d_values = f_values.iter().skip(1);
            if f_values[0].trim() == "P".to_string() {
                b.points(d_values.map(|v| v.parse::<f64>().unwrap()).collect(), point_list);
            } else if f_values[0].trim() == "C".to_string() {
                b.color(d_values.map(|v| v.parse::<f32>().unwrap()).collect());
            } else if f_values[0].trim() == "V".to_string() {
                b.velocity(d_values.map(|v| v.parse::<f32>().unwrap()).collect());
            }
        }
    });
    blocks
}

#[test]
fn test_parse_block_list() {
    let content = "- Rect
    P, 32.0, 32.0
    C, 0.0, 1.0, 1.0, 1.0
    V, 3.0, 0.0

";
    let mut point_list: Vec<Vec<f64>> = Vec::new();
    let blocks = parse_block_list(format!("{}", content), &mut point_list);
    assert_eq!(blocks[0].shape, ShapeKind::Rect);
    assert_eq!(point_list[0], vec![32.0, 32.0]);
}
