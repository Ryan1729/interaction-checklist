#![deny(unused)]
#![deny(bindings_with_variant_name)]

// In case we decide that we care about no_std/not allocating
type StrBuf = String;

type PlayX = DrawLength;
type PlayY = DrawLength;
type PlayW = DrawLength;
type PlayH = DrawLength;

#[derive(Clone, Debug, Default)]
pub struct PlayXYWH {
    pub x: PlayX,
    pub y: PlayY,
    pub w: PlayW,
    pub h: PlayH,
}

type BoardX = DrawLength;
type BoardY = DrawLength;
type BoardW = DrawLength;
type BoardH = DrawLength;

#[derive(Clone, Debug, Default)]
pub struct BoardXYWH {
    pub x: BoardX,
    pub y: BoardY,
    pub w: BoardW,
    pub h: BoardH,
}

pub type DrawX = DrawLength;
pub type DrawY = DrawLength;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DrawXY {
    pub x: DrawX,
    pub y: DrawY,
}

impl core::ops::Add for DrawXY {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl core::ops::AddAssign for DrawXY {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            x: self.x + other.x,
            y: self.y + other.y,
        };
    }
}

pub type DrawLength = f32;
pub type DrawW = DrawLength;
pub type DrawH = DrawLength;

#[derive(Copy, Clone, Debug, Default, PartialEq)]
pub struct DrawWH {
    pub w: DrawW,
    pub h: DrawH,
}

pub type TileCount = usize;
pub type TileSideLength = DrawLength;

#[derive(Clone, Debug, Default)]
pub struct Rect {
    pub min_x: DrawX,
    pub min_y: DrawY,
    pub max_x: DrawX,
    pub max_y: DrawY,
}

impl Rect {
    pub fn contains(&self, point: DrawXY) -> bool {
        // Use half-open ranges so the rects can abut each other, but no
        // location is in two different areas, unless the rects overlap.
        // (TODO test this? Are we going to end up wanting fixed point here?)
        point.x >= self.min_x
        && point.x < self.max_x
        && point.y >= self.min_y
        && point.y < self.max_y
    }
}

#[derive(Clone, Debug, Default)]
pub struct Sizes {
    pub draw_wh: DrawWH,
    pub play_xywh: PlayXYWH,
    pub board_xywh: BoardXYWH,
    pub tile_side_length: TileSideLength,
}

use crate::tile;

const LEFT_UI_WIDTH_TILES: TileCount = 9;
const RIGHT_UI_WIDTH_TILES: TileCount = 9;
const CENTER_UI_WIDTH_TILES: TileCount = if tile::X::COUNT < tile::Y::COUNT {
    tile::X::COUNT as TileCount
} else {
    tile::Y::COUNT as TileCount
};

pub(crate) const BOARD_W_TILES: TileCount = tile::X::COUNT as TileCount;
pub(crate) const BOARD_H_TILES: TileCount = tile::Y::COUNT as TileCount;

const TOP_LABELS_HEIGHT_TILES: TileCount = 1;
const CENTER_UI_HEIGHT_TILES: TileCount = CENTER_UI_WIDTH_TILES + TOP_LABELS_HEIGHT_TILES;
const NON_BOARD_H_TILES: TileCount = CENTER_UI_HEIGHT_TILES - BOARD_H_TILES;

const DRAW_WIDTH_TILES: TileCount = LEFT_UI_WIDTH_TILES 
    + CENTER_UI_WIDTH_TILES 
    + RIGHT_UI_WIDTH_TILES;

pub fn fresh_sizes(wh: DrawWH) -> Sizes {
    let w_length_bound = wh.w / DRAW_WIDTH_TILES as DrawW;
    let h_length_bound = wh.h / CENTER_UI_HEIGHT_TILES as DrawH;

    let (raw_bound, tile_side_length, board_x_offset, mut board_y_offset) = {
        if (w_length_bound - h_length_bound).abs() < 0.5 {
            (h_length_bound, h_length_bound.trunc(), h_length_bound.fract() / 2., h_length_bound.fract() / 2.)
        } else if w_length_bound > h_length_bound {
            (h_length_bound, h_length_bound.trunc(), 0., h_length_bound.fract() / 2.)
        } else if w_length_bound < h_length_bound {
            (w_length_bound, w_length_bound.trunc(), w_length_bound.fract() / 2., 0.)
        } else {
            // NaN ends up here
            // TODO return a Result? Panic? Take only known non-NaN values?
            (100., 100., 0., 0.)
        }
    };

    board_y_offset += (NON_BOARD_H_TILES as DrawLength * tile_side_length) / 2.;

    let play_area_w = raw_bound * DRAW_WIDTH_TILES as PlayW;
    let play_area_h = raw_bound * CENTER_UI_HEIGHT_TILES as PlayH;
    let play_area_x = (wh.w - play_area_w) / 2.;
    let play_area_y = (wh.h - play_area_h) / 2.;

    let board_area_w = tile_side_length * BOARD_W_TILES as BoardW;
    let board_area_h = tile_side_length * BOARD_H_TILES as BoardH;
    let board_area_x = play_area_x + board_x_offset + (play_area_w - board_area_w) / 2.;
    let board_area_y = play_area_y + board_y_offset + (play_area_h - board_area_h) / 2.;

    Sizes {
        draw_wh: wh,
        play_xywh: PlayXYWH {
            x: play_area_x,
            y: play_area_y,
            w: play_area_w,
            h: play_area_h,
        },
        board_xywh: BoardXYWH {
            x: board_area_x,
            y: board_area_y,
            w: board_area_w,
            h: board_area_h,
        },
        tile_side_length,
    }
}

pub(crate) fn margin(sizes: &Sizes) -> DrawLength {
    let smaller_side = if sizes.play_xywh.w < sizes.play_xywh.h {
        sizes.play_xywh.w
    } else {
        // NaN ends up here.
        sizes.play_xywh.h
    };

    smaller_side / 32.
}

pub(crate) fn label_wh(sizes: &Sizes) -> DrawWH {
    DrawWH {
        w: sizes.tile_side_length,
        h: sizes.tile_side_length,
    }
}

pub(crate) fn top_label_rect(sizes: &Sizes) -> Rect {
    let label_wh = label_wh(sizes);

    let zero_xy = draw_xy_from_tile(sizes, <_>::default());

    Rect {
        min_x: zero_xy.x,
        min_y: zero_xy.y - label_wh.h,
        max_x: zero_xy.x + crate::LABEL_COUNT as DrawX * label_wh.w,
        max_y: zero_xy.y,
    }
}

pub(crate) fn left_label_rect(sizes: &Sizes) -> Rect {
    let label_wh = label_wh(sizes);

    let zero_xy = draw_xy_from_tile(sizes, <_>::default());

    Rect {
        min_x: zero_xy.x - label_wh.h,
        min_y: zero_xy.y,
        max_x: zero_xy.x + label_wh.h,
        max_y: zero_xy.y + crate::LABEL_COUNT as DrawY * label_wh.h,
    }
}

pub(crate) fn draw_xy_from_tile(sizes: &Sizes, txy: tile::XY) -> DrawXY {
    let w_frac = tile::Coord::from(txy.x) as DrawLength / BOARD_W_TILES as DrawLength;
    let h_frac = tile::Coord::from(txy.y) as DrawLength / BOARD_H_TILES as DrawLength;

    DrawXY {
        x: sizes.board_xywh.x + sizes.board_xywh.w * w_frac,
        y: sizes.board_xywh.y + sizes.board_xywh.h * h_frac,
    }
}

pub(crate) fn tile_xy_from_draw(sizes: &Sizes, dxy: DrawXY) -> Option<tile::XY> {
    tile::X::try_from(((dxy.x - sizes.board_xywh.x) / sizes.board_xywh.w) * tile::X::COUNT as DrawLength)
        .ok()
        .and_then(|x| {
            tile::Y::try_from(((dxy.y - sizes.board_xywh.y) / sizes.board_xywh.h) * tile::Y::COUNT as DrawLength)
                .ok()
                .map(|y| tile::XY {
                    x,
                    y,
                })
        })
}

#[test]
fn all_the_tile_xys_round_trip_through_draw_xy() {
    let sizes = draw::fresh_sizes(EXAMPLE_WH);

    for txy in tile::all_xys() {
        let round_tripped = tile_xy_from_draw(
            &sizes,
            draw_xy_from_tile(&sizes, txy)
        ).unwrap();

        assert_eq!(round_tripped, txy);
    }
}

#[test]
fn all_the_tile_xys_round_trip_through_draw_xy_when_offset_slightly() {
    let sizes = draw::fresh_sizes(EXAMPLE_WH);

    for txy in tile::all_xys() {
        let mut draw_xy = draw_xy_from_tile(&sizes, txy);
        draw_xy.x += sizes.tile_side_length / 8.;
        draw_xy.y += sizes.tile_side_length / 8.;

        let round_tripped = tile_xy_from_draw(
            &sizes,
            draw_xy,
        ).unwrap();

        assert_eq!(round_tripped, txy);
    }
}

#[cfg(test)]
const EXAMPLE_WH: DrawWH = DrawWH { w: 1366., h: 768. };

use crate::{cell::UiState, ArrowKind, Dir, NineSlice, NineSliceKind};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpriteKind {
    NeutralEye,
    DirEye(Dir),
    Arrow(Dir, ArrowKind),
    SmallPupilEye,
    NarrowLeftEye,
    NarrowCenterEye,
    NarrowRightEye,
    ClosedEye,
    HalfLidEye,
    Unchecked(UiState),
    Checked(UiState),
    NineSlice(NineSlice, NineSliceKind),
}

impl Default for SpriteKind {
    fn default() -> Self {
        Self::NeutralEye
    }
}

#[derive(Debug)]
pub enum Command {
    Sprite(SpriteSpec),
    Text(TextSpec),
}

#[derive(Debug)]
pub struct SpriteSpec {
    pub sprite: SpriteKind,
    pub xy: DrawXY,
}

/// This is provided to make font selection etc. easier for platform layers.
#[derive(Clone, Copy, Debug)]
pub enum TextKind {
    UI,
}

#[derive(Clone, Debug)]
pub struct TextSpec {
    pub text: StrBuf,
    pub xy: DrawXY,
    /// We'd rather define a rectangle for the text to (hopefully) lie inside than
    /// a font size directly.
    pub wh: DrawWH,
    pub kind: TextKind,
}
