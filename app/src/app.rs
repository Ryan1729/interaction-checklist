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
    pub type Count = u32;

    pub type Coord = u8;

    #[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
    pub struct X(Coord);

    impl X {
        pub const MAX: Coord = 0b1111;
        pub const COUNT: Count = (X::MAX as Count) + 1;

        pub fn saturating_add_one(&self) -> Self {
            Self(core::cmp::min(self.0.saturating_add(1), Self::MAX))
        }

        pub fn saturating_sub_one(&self) -> Self {
            Self(self.0.saturating_sub(1))
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
        pub const MAX: Coord = 0b1111;
        pub const COUNT: Count = (Y::MAX as Count) + 1;

        pub fn saturating_add_one(&self) -> Self {
            Self(core::cmp::min(self.0.saturating_add(1), Self::MAX))
        }

        pub fn saturating_sub_one(&self) -> Self {
            Self(self.0.saturating_sub(1))
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
}

fn draw_xy_from_tile(sizes: &Sizes, txy: tile::XY) -> DrawXY {
    DrawXY {
        x: sizes.board_xywh.x + sizes.board_xywh.w * (tile::Coord::from(txy.x) as DrawLength / tile::X::COUNT as DrawLength),
        y: sizes.board_xywh.y + sizes.board_xywh.h * (tile::Coord::from(txy.y) as DrawLength / tile::Y::COUNT as DrawLength),
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
        #[allow(unused)]
        Checked
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

#[derive(Debug, Default)]
pub struct Ui {
    sizes: draw::Sizes,
    cursor_xy: CursorXY,
    left_mouse_button: ButtonState,
}

impl Ui {
    fn tile_state(&self, txy: tile::XY) -> UiState {
        let xy: DrawXY = draw_xy_from_tile(&self.sizes, txy);
        let min_x = xy.x;
        let min_y = xy.y;
        let max_x = min_x + self.sizes.tile_side_length;
        let max_y = min_y + self.sizes.tile_side_length;

        // Use half-open ranges so the cells can abut each other, but no location
        // is on two different cells. (TODO test this? Are we going to end up
        // wanting fixed point here?)
        let is_in_tile =
            self.cursor_xy.x >= min_x
            && self.cursor_xy.x < max_x
            && self.cursor_xy.y >= min_y
            && self.cursor_xy.y < max_y;

        match (is_in_tile, self.left_mouse_button) {
            (false, _) => UiState::Idle,
            (true, ButtonState::Up) => UiState::Hover,
            (true, ButtonState::Down) => UiState::Pressed,
        }
    }
}

#[derive(Debug, Default)]
struct Board {
    tiles: Tiles,
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

pub fn update(
    state: &mut State,
    commands: &mut dyn ClearableStorage<draw::Command>,
    input_flags: InputFlags,
    cursor_xy: CursorXY,
    draw_wh: DrawWH,
) {
    use draw::{TextSpec, TextKind, Command::*};

    if draw_wh != state.ui.sizes.draw_wh {
        state.ui.sizes = draw::fresh_sizes(draw_wh);
    }
    state.ui.cursor_xy = cursor_xy;

    commands.clear();

    let input = Input::from_flags(input_flags);

    use EyeState::*;
    use Input::*;
    use crate::Dir::*;

    const HOLD_FRAMES: AnimationTimer = 30;

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

    let left_text_x = state.ui.sizes.play_xywh.x + MARGIN;

    const MARGIN: f32 = 16.;

    let small_section_h = state.ui.sizes.draw_wh.h / 8. - MARGIN;

    {
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
