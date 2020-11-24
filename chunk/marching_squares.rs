use crate::chunk::{CellSize, ChunkData, ChunkSize};
use bevy::prelude::Vec2;

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
    TopLeft,
    TopRight,
    BotRight,
    BotLeft,
}

use Side::*;

const TRI_LUT: [&'static [Side]; 18] = [
    &[],
    &[Left, BotLeft, Bottom],
    &[Bottom, BotRight, Right],
    &[Left, BotLeft, BotRight, Left, BotRight, Right],
    &[Top, Right, TopRight],
    &[
        Left, BotLeft, Bottom, Left, Bottom, Right, Left, Right, Top, Top, Right, TopRight,
    ],
    &[Top, BotRight, TopRight, Bottom, BotRight, Top],
    &[
        BotLeft, BotRight, TopRight, TopRight, Top, Left, Left, BotLeft, TopRight,
    ],
    &[TopLeft, Left, Top],
    &[TopLeft, BotLeft, Bottom, Bottom, Top, TopLeft],
    &[
        TopLeft, Left, Top, Left, Bottom, Top, Top, Bottom, Right, Right, Bottom, BotRight,
    ],
    &[
        TopLeft, BotLeft, BotRight, TopLeft, BotRight, Right, Right, Top, TopLeft,
    ],
    &[TopLeft, Right, TopRight, TopLeft, Left, Right],
    &[
        TopRight, TopLeft, BotLeft, BotLeft, Bottom, Right, Right, TopRight, BotLeft,
    ],
    &[
        TopLeft, BotRight, TopRight, TopLeft, Left, Bottom, Bottom, BotRight, TopLeft,
    ],
    &[TopLeft, BotLeft, BotRight, BotRight, TopRight, TopLeft],
    &[Left, BotLeft, Bottom, Top, Right, TopRight], // 17 Disambiguity for case 5
    &[Bottom, BotRight, Right, TopLeft, Left, Top], // 17 Disambiguity for case 10
];

// fn norm(x: f32) -> f32 {
//     (x + 2.0) / (4.0)
// }

fn lerp(a: Vec2, b: Vec2, w1: f32, w2: f32, thresh: f32) -> Vec2 {
    let min = w1.min(w2);
    let max = w1.max(w2);

    // let t = min / (min - max);

    // if w1 > w2 {
    //     a.lerp(b, t)
    // } else {
    //     b.lerp(a, t)
    // }

    // let (p, p_w, n, n_w) = if w1 > w2 {
    //     (a, w1, b, w2)
    // } else {
    //     (b, w2, a, w1)
    // };

    // let t = n_w / (n_w - p_w);

    // let x = n.x() + t * (p.x() - n.x());
    // let y = n.y() + t * (p.x() - n.y());

    // Vec2::new(x, y)

    // let t = w1.min(w2) / (w1.min(w2) - w1.max(w2));

    // if w1 > w2 {
    //     a.lerp(b, t)
    // } else {
    //     b.lerp(a, t)
    // }

    // let (b, d, b_w, d_w) = if w1 > w2 {
    //     (b, a, w2, w1)
    // } else {
    //     (a, b, w1, w2)
    // };

    // let v = b + (b - d) * ((thresh - b_w) / (d_w - b_w));
    // v
    // d.lerp(b, v / 8.0)

    // let w1 = if w1 < thresh && thresh - w1 < 0.03 {
    //     w1 + 0.03
    // } else {
    //     w1
    // };

    // let w2 = if w2 < thresh && thresh - w2 < 0.03 {
    //     w2 + 0.03
    // } else {
    //     w2
    // };

    // let n = (1.0 / (w1)) + (1.0 / (w2));
    // let d = (w1).recip().abs() + (w2).recip().abs();

    // if w1 > w2 {
    // println!("b");
    // b + (n / d)
    // } else {
    // println!("a");
    // a + (n / d)
    // }

    // let v = (n / d).abs();

    // if w1 > w2 {
    //     a.lerp(b, v)
    // } else {
    //     b.lerp(a, v)
    // }

    (a + b) / 2.0
}

fn get_point(pts: &[f32; 4], side: Side, cz: CellSize, threshold: f32) -> Vec2 {
    let (cx, cy) = (cz.0 as f32, cz.1 as f32);

    match side {
        Top => {
            let a = Vec2::new(0.0, 0.0);
            let b = Vec2::new(cx, 0.0);

            lerp(a, b, pts[0], pts[1], threshold)
            // a.lerp(b, t)
            // let diff = norm(pts[1] - pts[0]);
            // a.lerp(b, diff)
        }
        Right => {
            let a = Vec2::new(cx, 0.0);
            let b = Vec2::new(cx, -cy);

            lerp(a, b, pts[1], pts[2], threshold)
            // a.lerp(b, t)
            // let diff = norm(pts[2] - pts[1]);
            // a.lerp(b, diff)
        }
        Bottom => {
            let a = Vec2::new(cx, -cy);
            let b = Vec2::new(0.0, -cy);

            lerp(a, b, pts[2], pts[3], threshold)
            // a.lerp(b, t)
            // let diff = norm(pts[3] - pts[2]);
            // a.lerp(b, diff)
        }
        Left => {
            let a = Vec2::new(0.0, -cy);
            let b = Vec2::new(0.0, 0.0);

            lerp(a, b, pts[3], pts[0], threshold)
            // a.lerp(b, t)
            // let diff = norm(pts[0] - pts[3]);
            // a.lerp(b, diff)
        }
        TopLeft => Vec2::new(0.0, 0.0),
        TopRight => Vec2::new(cx, 0.0),
        BotRight => Vec2::new(cx, -cy),
        BotLeft => Vec2::new(0.0, -cy),
    }
}

pub fn get_points(pts: [f32; 4], op: (&Side, &Side), cz: CellSize, threshold: f32) -> (Vec2, Vec2) {
    let pt1 = get_point(&pts, *op.0, cz, threshold);
    let pt2 = get_point(&pts, *op.1, cz, threshold);

    (pt1, pt2)
}

pub fn marching_squares(
    data: ChunkData,
    chunk_size: ChunkSize,
    cell_size: CellSize,
    threshold: f32,
) -> (Vec<[f32; 2]>, Vec<u32>) {
    let mut verts = Vec::new();
    let mut idxs = Vec::new();

    for x in 0..chunk_size.0 {
        for y in 0..chunk_size.1 {
            let offset = Vec2::new((x * cell_size.0) as f32, (y * cell_size.1) as f32 * -1.0);

            let top_left = data.get_at(chunk_size, x, y);
            let top_right = data.get_at(chunk_size, x + 1, y);
            let bot_right = data.get_at(chunk_size, x + 1, y + 1);
            let bot_left = data.get_at(chunk_size, x, y + 1);

            let mut idx: usize = 0;
            idx |= ((top_left > threshold) as usize) << 3;
            idx |= ((top_right > threshold) as usize) << 2;
            idx |= ((bot_right > threshold) as usize) << 1;
            idx |= (bot_left > threshold) as usize;

            // println!("X Y: ({}, {})", x, y);
            // println!("Idx: {}", idx);
            // dbg!(top_left, top_right, bot_right, bot_left);

            let idx = match idx {
                5 => {
                    if (top_left + top_right + bot_right + bot_left) / 4.0 > threshold {
                        5
                    } else {
                        16
                    }
                }
                10 => {
                    if (top_left + top_right + bot_right + bot_left) / 4.0 > threshold {
                        10
                    } else {
                        17
                    }
                }

                idx => idx,
            };

            let triangles = TRI_LUT[idx];

            let pts = [top_left, top_right, bot_right, bot_left];

            for c in triangles.chunks_exact(3) {
                // println!("{:?}", c);

                let (a, b, c) = (c[0], c[1], c[2]);
                let pt_a = get_point(&pts, a, cell_size, threshold) + offset;
                let pt_b = get_point(&pts, b, cell_size, threshold) + offset;
                let pt_c = get_point(&pts, c, cell_size, threshold) + offset;

                vpush(&mut verts, &mut idxs, pt_a);
                vpush(&mut verts, &mut idxs, pt_b);
                vpush(&mut verts, &mut idxs, pt_c);
            }
        }
    }

    (verts, idxs)
}

fn vpush(verts: &mut Vec<[f32; 2]>, idxs: &mut Vec<u32>, vert: Vec2) {
    let next_idx = verts.len();
    verts.push(vert.into());
    idxs.push(next_idx as u32);
}
