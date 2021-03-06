#![deny(unused)]
#![deny(bindings_with_variant_name)]

#[allow(unused)]
macro_rules! compile_time_assert {
    ($assertion: expr) => {{
        #[allow(unknown_lints, eq_op)]
        // Based on the const_assert macro from static_assertions;
        const _: [(); 0 - !{$assertion} as usize] = [];
    }}
}

// In case we decide that we care about no_std/not directly allocating ourself
pub trait ClearableStorage<A> {
    fn clear(&mut self);

    fn push(&mut self, a: A);
}

/// This type alias makes adding a custom newtype easy.
pub type X = f32;
/// This type alias makes adding a custom newtype easy.
pub type Y = f32;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct XY {
    pub x: X,
    pub y: Y,
}

pub mod draw;

pub use draw::{
    DrawLength,
    DrawX,
    DrawY,
    DrawXY,
    DrawW,
    DrawH,
    DrawWH,
    SpriteKind,
    SpriteSpec,
    Sizes,
};
use draw::{
    draw_xy_from_tile,
    tile_xy_from_draw,
    label_wh,
    top_label_rect,
    left_label_rect,
    zero_tile_xy,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ArrowKind {
    Red,
    Green
}

impl Default for ArrowKind {
    fn default() -> Self {
        Self::Red
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LRThreeSlice {
    Left,
    Center,
    Right,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NineSlice {
    UpperLeft,
    Upper,
    UpperRight,
    Left,
    NoEdges,
    Right,
    LowerLeft,
    Lower,
    LowerRight,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BorderKind {
    WhiteEdge,
    YellowEdge
}

impl Default for BorderKind {
    fn default() -> Self {
        Self::WhiteEdge
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dir {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

impl Default for Dir {
    fn default() -> Self {
        Self::Up
    }
}

mod tile {
    use crate::{DrawX, DrawY};

    pub type Count = u32;

    pub type Coord = u8;

    pub(crate) const COORD_MAX: Coord = 0b1111;
    pub(crate) const COORD_COUNT: Coord = COORD_MAX + 1;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct X(Coord);

    impl X {
        pub const MAX: Coord = COORD_MAX;
        pub const COUNT: Count = (X::MAX as Count) + 1;

        pub fn saturating_add_one(&self) -> Self {
            Self(core::cmp::min(self.0.saturating_add(1), Self::MAX))
        }

        pub fn saturating_sub_one(&self) -> Self {
            Self(self.0.saturating_sub(1))
        }
    }

    type XError = ();

    impl TryFrom<DrawX> for X {
        type Error = XError;
        fn try_from(draw_x: DrawX) -> Result<Self, Self::Error> {
            if draw_x >= 0. && draw_x < (Self::MAX + 1) as DrawX {
                Ok(Self(draw_x as Coord))
            } else {
                Err(())
            }
        }
    }

    impl TryFrom<Coord> for X {
        type Error = XError;
        fn try_from(coord: Coord) -> Result<Self, Self::Error> {
            if coord <= Self::MAX {
                Ok(Self(coord))
            } else {
                Err(())
            }
        }
    }

    impl From<X> for Coord {
        fn from(X(c): X) -> Self {
            c
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct Y(Coord);

    impl Y {
        pub const MAX: Coord = COORD_MAX;
        pub const COUNT: Count = (Y::MAX as Count) + 1;

        pub fn saturating_add_one(&self) -> Self {
            Self(core::cmp::min(self.0.saturating_add(1), Self::MAX))
        }

        pub fn saturating_sub_one(&self) -> Self {
            Self(self.0.saturating_sub(1))
        }
    }

    type YError = ();

    impl TryFrom<DrawX> for Y {
        type Error = YError;
        fn try_from(draw_y: DrawY) -> Result<Self, Self::Error> {
            if draw_y >= 0. && draw_y < (Self::MAX + 1) as DrawY {
                Ok(Self(draw_y as Coord))
            } else {
                Err(())
            }
        }
    }

    impl TryFrom<Coord> for Y {
        type Error = YError;
        fn try_from(coord: Coord) -> Result<Self, Self::Error> {
            if coord <= Self::MAX {
                Ok(Self(coord))
            } else {
                Err(())
            }
        }
    }

    impl From<Y> for Coord {
        fn from(Y(c): Y) -> Self {
            c
        }
    }

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct XY {
        pub x: X,
        pub y: Y,
    }

    impl XY {
        pub const COUNT: Count = X::COUNT * Y::COUNT;

        pub fn move_up(&mut self) {
            self.y = self.y.saturating_sub_one();
        }

        pub fn move_down(&mut self) {
            self.y = self.y.saturating_add_one();
        }

        pub fn move_left(&mut self) {
            self.x = self.x.saturating_sub_one();
        }

        pub fn move_right(&mut self) {
            self.x = self.x.saturating_add_one();
        }
    }

    #[allow(unused)]
    pub fn xy_to_i(xy: XY) -> usize {
        xy_to_i_usize((usize::from(xy.x.0), usize::from(xy.y.0)))
    }

    pub fn xy_to_i_usize((x, y): (usize, usize)) -> usize {
        y * Y::COUNT as usize + x
    }

    pub fn i_to_xy(index: usize) -> XY {
        XY {
            x: X(to_coord_or_default(
                (index % X::COUNT as usize) as Count
            )),
            y: Y(to_coord_or_default(
                ((index % (XY::COUNT as usize) as usize)
                / X::COUNT as usize) as Count
            )),
        }
    }

    fn to_coord_or_default(n: Count) -> Coord {
        core::convert::TryFrom::try_from(n).unwrap_or_default()
    }

    #[cfg(test)]
    pub fn all_xys() -> Vec<XY> {
        let mut output = Vec::with_capacity(XY::COUNT as usize);

        for y in 0..Y::MAX {
            for x in 0..X::MAX {
                output.push(XY {x: X(x), y: Y(y)});
            }
        }

        output
    }
}

mod cell {
    use crate::draw::SpriteKind;

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub enum UiState {
        Idle,
        Hover,
        Pressed
    }

    #[derive(Copy, Clone, Debug, Eq, PartialEq)]
    pub(crate) enum Status {
        Unchecked,
        Checked,
    }

    impl Default for Status {
        fn default() -> Self {
            Self::Unchecked
        }
    }

    impl Status {
        pub(crate) fn sprite_fn(self) -> fn(UiState) -> SpriteKind {
            match self {
                Self::Unchecked => SpriteKind::Unchecked,
                Self::Checked => SpriteKind::Checked,
            }
        }
    }
}
pub use cell::UiState;

type TileData = cell::Status;

pub const TILES_LENGTH: usize = tile::XY::COUNT as _;

type TileDataArray = [TileData; TILES_LENGTH as _];

#[derive(Clone, Debug)]
pub struct Tiles {
    tiles: TileDataArray,
}

impl Default for Tiles {
    fn default() -> Self {
        Self {
            tiles: [TileData::default(); TILES_LENGTH as _],
        }
    }
}

#[derive(Debug)]
enum EyeState {
    Idle,
    Moved(Dir),
    NarrowAnimLeft,
    NarrowAnimCenter,
    NarrowAnimRight,
    SmallPupil,
    Closed,
    HalfLid,
}

impl Default for EyeState {
    fn default() -> Self {
        Self::Idle
    }
}

impl EyeState {
    fn sprite(&self) -> SpriteKind {
        use EyeState::*;
        match self {
            Idle => SpriteKind::NeutralEye,
            Moved(dir) => SpriteKind::DirEye(*dir),
            NarrowAnimLeft => SpriteKind::NarrowLeftEye,
            NarrowAnimCenter => SpriteKind::NarrowCenterEye,
            NarrowAnimRight => SpriteKind::NarrowRightEye,
            SmallPupil => SpriteKind::SmallPupilEye,
            Closed => SpriteKind::ClosedEye,
            HalfLid => SpriteKind::HalfLidEye,
        }
    }
}

#[derive(Debug, Default)]
struct Eye {
    xy: tile::XY,
    state: EyeState,
}

#[derive(Copy, Clone, Debug)]
enum ButtonState {
    Up,
    #[allow(unused)]
    Down,
}

impl Default for ButtonState {
    fn default() -> Self {
        ButtonState::Up
    }
}

pub type CursorXY = DrawXY;

#[derive(Debug, Eq, PartialEq)]
enum ClickArea {
    TileXY(tile::XY),
    Labels,
}

#[derive(Debug)]
enum UiMode {
    Checking,
    EditLabels,
}

impl Default for UiMode {
    fn default() -> Self {
        UiMode::Checking
    }
}

#[derive(Debug, Default)]
pub struct Ui {
    mode: UiMode,
    left_mouse_button: ButtonState,
    sizes: draw::Sizes,
    cursor_xy: CursorXY,
    last_pressed: Option<ClickArea>,
}

impl Ui {
    fn tile_state(&self, txy: tile::XY) -> UiState {
        use ClickArea::*;
        match self.last_pressed {
            Some(TileXY(last_txy)) => {
                match (last_txy == txy, self.left_mouse_button) {
                    (false, _)
                    | (true, ButtonState::Up) => if self.is_hovered(TileXY(txy)) {
                        UiState::Hover
                    } else {
                        UiState::Idle
                    },
                    (true, ButtonState::Down) => UiState::Pressed,
                }
            }
            None | Some(Labels) => {
                match (self.is_hovered(TileXY(txy)), self.left_mouse_button) {
                    (false, _) => UiState::Idle,
                    (true, ButtonState::Up) => UiState::Hover,
                    (true, ButtonState::Down) => UiState::Pressed,
                }
            }
        }
    }

    fn is_hovered(&self, area: ClickArea) -> bool {
        use ClickArea::*;
        let rect = match area {
            TileXY(txy) => {
                let xy: DrawXY = draw_xy_from_tile(&self.sizes, txy);

                draw::Rect {
                    min_x: xy.x,
                    min_y: xy.y,
                    max_x: xy.x + self.sizes.tile_side_length,
                    max_y: xy.y + self.sizes.tile_side_length,
                }
            },
            Labels => top_label_rect(&self.sizes),
        };

        rect.contains(self.cursor_xy)
    }

    fn click_area(&self) -> Option<ClickArea> {
        if top_label_rect(&self.sizes).contains(self.cursor_xy) {
            return Some(ClickArea::Labels);
        }

        tile_xy_from_draw(&self.sizes, self.cursor_xy)
            .map(ClickArea::TileXY)
    }
}

type Label = String;

const LABEL_COUNT: usize = tile::COORD_COUNT as usize;

#[derive(Debug, Default)]
struct Board {
    tiles: Tiles,
    labels: [Label; LABEL_COUNT],
    eye: Eye,
}

/// 64k animation frames ought to be enough for anybody!
type AnimationTimer = u16;

/// We use this because it has a lot more varied factors than 65536.
const ANIMATION_TIMER_LENGTH: AnimationTimer = 60 * 60 * 18;

#[derive(Debug, Default)]
pub struct State {
    ui: Ui,
    board: Board,
    animation_timer: AnimationTimer
}

pub fn sizes(state: &State) -> draw::Sizes {
    state.ui.sizes.clone()
}

pub type InputFlags = u16;

pub const INPUT_UP_PRESSED: InputFlags              = 0b0000_0000_0000_0001;
pub const INPUT_DOWN_PRESSED: InputFlags            = 0b0000_0000_0000_0010;
pub const INPUT_LEFT_PRESSED: InputFlags            = 0b0000_0000_0000_0100;
pub const INPUT_RIGHT_PRESSED: InputFlags           = 0b0000_0000_0000_1000;

pub const INPUT_UP_DOWN: InputFlags                 = 0b0000_0000_0001_0000;
pub const INPUT_DOWN_DOWN: InputFlags               = 0b0000_0000_0010_0000;
pub const INPUT_LEFT_DOWN: InputFlags               = 0b0000_0000_0100_0000;
pub const INPUT_RIGHT_DOWN: InputFlags              = 0b0000_0000_1000_0000;

pub const INPUT_INTERACT_PRESSED: InputFlags        = 0b0000_0001_0000_0000;
pub const INPUT_INTERACT_DOWN: InputFlags           = 0b0000_0010_0000_0000;

/// Should be set if the mouse button was pressed or released this frame.
pub const INPUT_LEFT_MOUSE_CHANGED: InputFlags      = 0b0000_0100_0000_0000;
pub const INPUT_LEFT_MOUSE_DOWN: InputFlags         = 0b0000_1000_0000_0000;

#[derive(Clone, Copy, Debug)]
enum Input {
    NoChange,
    Dir(Dir),
    Interact,
}

impl Input {
    fn from_flags(flags: InputFlags) -> Self {
        use Input::*;
        use crate::Dir::*;
        if INPUT_INTERACT_PRESSED & flags != 0 {
            Interact
        } else if (INPUT_UP_DOWN | INPUT_RIGHT_DOWN) & flags == (INPUT_UP_DOWN | INPUT_RIGHT_DOWN) {
            Dir(UpRight)
        } else if (INPUT_DOWN_DOWN | INPUT_RIGHT_DOWN) & flags == (INPUT_DOWN_DOWN | INPUT_RIGHT_DOWN) {
            Dir(DownRight)
        } else if (INPUT_DOWN_DOWN | INPUT_LEFT_DOWN) & flags == (INPUT_DOWN_DOWN | INPUT_LEFT_DOWN) {
            Dir(DownLeft)
        } else if (INPUT_UP_DOWN | INPUT_LEFT_DOWN) & flags == (INPUT_UP_DOWN | INPUT_LEFT_DOWN) {
            Dir(UpRight)
        } else if INPUT_UP_DOWN & flags != 0 {
            Dir(Up)
        } else if INPUT_DOWN_DOWN & flags != 0 {
            Dir(Down)
        } else if INPUT_LEFT_DOWN & flags != 0 {
            Dir(Left)
        } else if INPUT_RIGHT_DOWN & flags != 0 {
            Dir(Right)
        } else {
            NoChange
        }
    }
}

const TEXT_BOX_W_TILES: u8 = 15;

fn text_box_rect(sizes: &Sizes, i: usize) -> draw::Rect {
    let tile_side_length = sizes.tile_side_length;

    let zero_xy = zero_tile_xy(sizes);
    let x = zero_xy.x;
    let y = zero_xy.y + i as DrawLength * tile_side_length;

    let number_w = tile_side_length;

    let label_x = x + number_w;

    draw::Rect {
        min_x: label_x,
        min_y: y,
        max_x: label_x
               + TEXT_BOX_W_TILES as DrawLength * tile_side_length,
        max_y: y + tile_side_length,
    }
}

pub type AsciiByte = u8;

pub type TextInput = [AsciiByte; 8];

pub fn update(
    state: &mut State,
    commands: &mut dyn ClearableStorage<draw::Command>,
    input_flags: InputFlags,
    text_input: TextInput,
    cursor_xy: CursorXY,
    draw_wh: DrawWH,
) {
    use draw::{TextSpec, TextKind, Command::*};

    if draw_wh != state.ui.sizes.draw_wh {
        state.ui.sizes = draw::fresh_sizes(draw_wh);
    }
    state.ui.cursor_xy = cursor_xy;

    let left_mouse_button_down = input_flags & INPUT_LEFT_MOUSE_DOWN != 0;

    state.ui.left_mouse_button =
        if left_mouse_button_down {
            ButtonState::Down
        } else {
            ButtonState::Up
        };

    commands.clear();

    let input = Input::from_flags(input_flags);

    use EyeState::*;
    use Input::*;
    use crate::Dir::*;

    const HOLD_FRAMES: AnimationTimer = 30;

    let left_mouse_button_pressed =
        input_flags & INPUT_LEFT_MOUSE_CHANGED != 0
        && left_mouse_button_down;
    let left_mouse_button_released =
        input_flags & INPUT_LEFT_MOUSE_CHANGED != 0
        && !left_mouse_button_down;

    assert!(
        !(left_mouse_button_pressed && left_mouse_button_released)
    );

    if left_mouse_button_pressed {
        state.ui.last_pressed = state.ui.click_area();
    }

    macro_rules! on_clicked {
        (| $click_area: ident | $code: block) => {
            if left_mouse_button_released {
                if state.ui.click_area().is_some()
                && state.ui.last_pressed == state.ui.click_area() {
                    match state.ui.last_pressed {
                        None => {
                            panic!("unexpected last_pressed state");
                        },
                        Some(ref $click_area) => {
                            $code
                        },
                    }
                } else {
                    state.ui.last_pressed = None;
                }
            }
        }
    }

    match state.ui.mode {
        UiMode::Checking => {
            match input {
                NoChange => match state.board.eye.state {
                    Idle => {
                        if state.animation_timer % (HOLD_FRAMES * 3) == 0 {
                            state.board.eye.state = NarrowAnimCenter;
                        }
                    },
                    Moved(_) => {
                        if state.animation_timer % HOLD_FRAMES == 0 {
                            state.board.eye.state = Idle;
                        }
                    },
                    SmallPupil => {
                        if state.animation_timer % (HOLD_FRAMES * 3) == 0 {
                            state.board.eye.state = Closed;
                        }
                    },
                    Closed => {
                        if state.animation_timer % (HOLD_FRAMES) == 0 {
                            state.board.eye.state = HalfLid;
                        }
                    },
                    HalfLid => {
                        if state.animation_timer % (HOLD_FRAMES * 5) == 0 {
                            state.board.eye.state = Idle;
                        }
                    },
                    NarrowAnimCenter => {
                        let modulus = state.animation_timer % (HOLD_FRAMES * 4);
                        if modulus == 0 {
                            state.board.eye.state = NarrowAnimRight;
                        } else if modulus == HOLD_FRAMES * 2 {
                            state.board.eye.state = NarrowAnimLeft;
                        }
                    },
                    NarrowAnimLeft | NarrowAnimRight => {
                        if state.animation_timer % HOLD_FRAMES == 0 {
                            state.board.eye.state = NarrowAnimCenter;
                        }
                    },
                },
                Dir(Up) => {
                    state.board.eye.state = Moved(Up);
                    state.board.eye.xy.move_up();
                },
                Dir(UpRight) => {
                    state.board.eye.state = Moved(UpRight);
                    state.board.eye.xy.move_up();
                    state.board.eye.xy.move_right();
                },
                Dir(Right) => {
                    state.board.eye.state = Moved(Right);
                    state.board.eye.xy.move_right();
                },
                Dir(DownRight) => {
                    state.board.eye.state = Moved(DownRight);
                    state.board.eye.xy.move_down();
                    state.board.eye.xy.move_right();
                },
                Dir(Down) => {
                    state.board.eye.state = Moved(Down);
                    state.board.eye.xy.move_down();
                },
                Dir(DownLeft) => {
                    state.board.eye.state = Moved(DownLeft);
                    state.board.eye.xy.move_down();
                    state.board.eye.xy.move_left();
                },
                Dir(Left) => {
                    state.board.eye.state = Moved(Left);
                    state.board.eye.xy.x = state.board.eye.xy.x.saturating_sub_one();
                },
                Dir(UpLeft) => {
                    state.board.eye.state = Moved(UpLeft);
                    state.board.eye.xy.move_up();
                    state.board.eye.xy.move_left();
                },
                Interact => {
                    state.board.eye.state = SmallPupil;
                },
            }

            on_clicked!(
                |area| {
                    match *area {
                        ClickArea::TileXY(txy) => {
                            let i = tile::xy_to_i(txy);

                            state.board.tiles.tiles[i] = match state.board.tiles.tiles[i] {
                                TileData::Checked => TileData::Unchecked,
                                TileData::Unchecked => TileData::Checked,
                            };
                        },
                        ClickArea::Labels => {
                            state.ui.mode = UiMode::EditLabels;
                        }
                    }
                }
            );
        },
        UiMode::EditLabels => {
            on_clicked!(
                |area| {
                    match area {
                        ClickArea::TileXY(_) => {},
                        ClickArea::Labels => {
                            // Will probably want a close button instead.
                            state.ui.mode = UiMode::Checking;
                        }
                    }
                }
            );

            for (i, label) in state.board.labels.iter_mut().enumerate() {
                let text_box_rect = text_box_rect(&state.ui.sizes, i);

                if text_box_rect.contains(state.ui.cursor_xy) {
                    for byte in text_input {
                        match byte {
                            0 => break,
                            // backspace
                            8
                            // delete
                            | 127 => { label.pop(); }
                            _ => { label.push(byte as char); }
                        }
                    }
                }
            }
        }
    }

    match state.ui.mode {
        UiMode::Checking => {
            for i in 0..TILES_LENGTH {
                let tile_data = state.board.tiles.tiles[i];

                let txy = tile::i_to_xy(i);

                commands.push(Sprite(SpriteSpec{
                    sprite: (tile_data.sprite_fn())(state.ui.tile_state(txy)),
                    xy: draw_xy_from_tile(&state.ui.sizes, txy),
                }));
            }

            commands.push(Sprite(SpriteSpec{
                sprite: state.board.eye.state.sprite(),
                xy: draw_xy_from_tile(&state.ui.sizes, state.board.eye.xy),
            }));
        },
        UiMode::EditLabels => {
            state.board.eye.state = Idle;
        },
    }

    // Drawing labelled edges
    {
        let label_wh = label_wh(&state.ui.sizes);
        let top_label_rect = top_label_rect(&state.ui.sizes);
        let left_label_rect = left_label_rect(&state.ui.sizes);

        let mut top_x = top_label_rect.min_x;
        let top_y = top_label_rect.min_y;

        let left_x = left_label_rect.min_x;
        let mut left_y = left_label_rect.min_y;

        let slice_kind = if
            top_label_rect.contains(state.ui.cursor_xy)
            || left_label_rect.contains(state.ui.cursor_xy) {
            BorderKind::YellowEdge
        } else {
            BorderKind::WhiteEdge
        };

        for (i, label) in state.board.labels.iter().enumerate() {
            // background
            let top_slice = if i == 0 {
                NineSlice::UpperLeft
            } else if i == state.board.labels.len() - 1 {
                NineSlice::UpperRight
            } else {
                NineSlice::Upper
            };
            let left_slice = if i == 0 {
                NineSlice::UpperLeft
            } else if i == state.board.labels.len() - 1 {
                NineSlice::LowerLeft
            } else {
                NineSlice::Left
            };

            commands.push(Sprite(SpriteSpec{
                sprite: SpriteKind::NineSlice(top_slice, slice_kind),
                xy: DrawXY { x: top_x, y: top_y },
            }));

            commands.push(Sprite(SpriteSpec{
                sprite: SpriteKind::NineSlice(left_slice, slice_kind),
                xy: DrawXY { x: left_x, y: left_y },
            }));


            // text
            let top_text_spec = TextSpec {
                text: label.to_string(),
                xy: DrawXY { x: top_x, y: top_y },
                wh: label_wh,
                kind: TextKind::CellLabel,
            };

            commands.push(Text(top_text_spec.clone()));
            commands.push(Text(TextSpec {
                xy: DrawXY { x: left_x, y: left_y },
                ..top_text_spec
            }));

            top_x += label_wh.w;
            left_y += label_wh.h;
        }
    }

    match state.ui.mode {
        UiMode::Checking => {/* no extra drawing in this layer yet */},
        UiMode::EditLabels => {
            let tile_side_length = state.ui.sizes.tile_side_length;
            let section_h = tile_side_length;

            for (i, label) in state.board.labels.iter().enumerate() {
                let text_box_rect = text_box_rect(&state.ui.sizes, i);

                let label_x = text_box_rect.min_x;
                let x = label_x - state.ui.sizes.tile_side_length;
                let y = text_box_rect.min_y;

                commands.push(Text(TextSpec{
                    text: format!("{i}:"), // TODO could make n static strs
                    xy: DrawXY { x, y },
                    wh: DrawWH {
                        w: state.ui.sizes.board_xywh.w,
                        h: section_h
                    },
                    kind: TextKind::OneTile,
                }));

                let is_hovered = text_box_rect.contains(state.ui.cursor_xy);

                let border_kind = if is_hovered {
                    BorderKind::YellowEdge
                } else {
                    BorderKind::WhiteEdge
                };

                for i in 0..TEXT_BOX_W_TILES {
                    let three_slice = if i == 0 {
                        LRThreeSlice::Left
                    } else if i == TEXT_BOX_W_TILES - 1 {
                        LRThreeSlice::Right
                    } else {
                        LRThreeSlice::Center
                    };

                    commands.push(Sprite(SpriteSpec{
                        sprite: SpriteKind::LRThreeSlice(
                            three_slice,
                            border_kind
                        ),
                        xy: DrawXY {
                            x: label_x + i as DrawLength * tile_side_length,
                            y
                        },
                    }));
                }

                commands.push(Text(
                    TextSpec{
                        text: label.to_string(), // TODO Copy-on-write in this case?
                        xy: DrawXY { x: label_x, y },
                        wh: DrawWH {
                            w: state.ui.sizes.board_xywh.w,
                            h: section_h
                        },
                        kind: if is_hovered {
                            TextKind::TextBoxWithCursor
                        } else {
                            TextKind::TextBox
                        },
                    }
                ));
            }
        },
    }

    #[cfg(any())]
    {
        const MARGIN: f32 = 16.;

        let left_text_x = state.ui.sizes.play_xywh.x + MARGIN;

        let small_section_h = state.ui.sizes.draw_wh.h / 8. - MARGIN;

        let mut y = MARGIN;

        commands.push(Text(TextSpec{
            text: format!(
                "input: {:?}",
                input
            ),
            xy: DrawXY { x: left_text_x, y },
            wh: DrawWH {
                w: state.ui.sizes.play_xywh.w,
                h: small_section_h
            },
            kind: TextKind::UI,
        }));

        y += small_section_h;

        commands.push(Text(TextSpec{
            text: format!(
                "sizes: {:?}\nanimation_timer: {:?}",
                state.ui.sizes,
                state.animation_timer
            ),
            xy: DrawXY { x: left_text_x, y },
            wh: DrawWH {
                w: state.ui.sizes.play_xywh.w,
                h: state.ui.sizes.play_xywh.h - y
            },
            kind: TextKind::UI,
        }));
    }

    state.animation_timer += 1;
    if state.animation_timer >= ANIMATION_TIMER_LENGTH {
        state.animation_timer = 0;
    }
}
