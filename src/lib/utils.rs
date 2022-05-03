#![allow(dead_code)]
use amethyst::ui::{
    Anchor, UiTransform,
};
use amethyst_rendy::palette::Srgba;

pub const DPI: f32 = 1.25;
pub const BACKGROUND_COLOR: [u32; 4] = [60, 179, 113, 255];
pub const BLACK: [u32; 4] = [0, 0, 0, 255];
pub const WHITE: [u32; 4] = [255, 255, 255, 255];
pub const GRAY_1: [u32; 4] = [224, 224, 224, 255];
pub const GRAY_2: [u32; 4] = [192, 192, 192, 255];
pub const GRAY_3: [u32; 4] = [128, 128, 128, 255];

pub fn get_color([r, g, b, a]: [u32; 4]) -> [f32; 4] {
    let (new_r, new_g, new_b, _) = Srgba::new(
        r as f32 / 255., g as f32 / 255., b as f32 / 255., 1.)
            .into_linear()
            .into_components();
    [new_r, new_g, new_b, a as f32 / 255.]
}

pub const CAMERA_ALPHA: f32 = 0.1;
pub const PLAYER_SPEED: i32 = 8;
pub const PLAYER_JUMP: i32 = 30;
pub const GRAVITY: i32 = 2;
pub const LOWER_BOUND: i32 = -200;

pub fn anchor_to_tuple(anchor: Anchor) -> (i32, i32) {
    match anchor {
        Anchor::BottomLeft => { (-1, -1) }
        Anchor::BottomMiddle => { (0, -1) }
        Anchor::BottomRight => { (1, -1) }
        Anchor::MiddleLeft => { (-1, 0) }
        Anchor::Middle => { (0, 0) }
        Anchor::MiddleRight => { (1, 0) }
        Anchor::TopLeft => { (-1, 1) }
        Anchor::TopMiddle => { (0, 1) }
        Anchor::TopRight => { (1, 1) }
    }
}

pub fn get_local_point(uitrans: UiTransform, pivot: Anchor) -> (f32, f32) {
    let target_tuple = anchor_to_tuple(pivot);
    let pivot_tuple = anchor_to_tuple(uitrans.pivot);
    let (mut x, mut y) = (uitrans.local_x, uitrans.local_y);
    x += (target_tuple.0 - pivot_tuple.0) as f32 * 0.5 * uitrans.width;
    y += (target_tuple.1 - pivot_tuple.1) as f32 * 0.5 * uitrans.height;
    (x, y)
}

// UiTransform
pub fn compare(uitrans1: UiTransform, uitrans2: UiTransform) -> Anchor {
    // Corners
    let bl1 = get_local_point(uitrans1.clone(), Anchor::BottomLeft);
    let br1 = get_local_point(uitrans1.clone(), Anchor::BottomRight);
    let tl1 = get_local_point(uitrans1.clone(), Anchor::TopLeft);
    let tr1 = get_local_point(uitrans1.clone(), Anchor::TopRight);
    let bl2 = get_local_point(uitrans2.clone(), Anchor::BottomLeft);
    let br2 = get_local_point(uitrans2.clone(), Anchor::BottomRight);
    let tl2 = get_local_point(uitrans2.clone(), Anchor::TopLeft);
    let tr2 = get_local_point(uitrans2.clone(), Anchor::TopRight);
    if bl1.1 >= tr2.1 && bl1.0 >= tr2.0 { return Anchor::TopRight; }
    else if br1.1 >= tl2.1 && br1.0 <= tl2.0 { return Anchor::TopLeft; }
    else if tl1.1 <= br2.1 && tl1.0 >= br2.0 { return Anchor::BottomRight; }
    else if tr1.1 <= bl2.1 && tr1.0 <= bl2.0 { return Anchor::BottomLeft; }
    else if bl1.1 >= tl2.1 { return Anchor::TopMiddle; }
    else if tl1.1 <= bl2.1 { return Anchor::BottomMiddle; }
    else if bl1.0 >= br2.0 { return Anchor::MiddleRight; }
    else if br1.0 <= bl2.0 { return Anchor::MiddleLeft; }
    else { return Anchor::Middle; }
}
