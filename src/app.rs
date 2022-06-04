use ggez::event::EventHandler;
use ggez::graphics::{self, Color, Image};
use ggez::{Context, GameResult};
use glam::Vec2;
use std::collections::HashMap;

// #[derive(Debug)]
struct Piece {
    // id: u16,
    // x: f32,
    // y: f32,
    img: Image,
    // color: char,
    vec: Vec2,
}
// #[derive(Debug)]
pub struct Game {
    board: Image,
    red: HashMap<u16, Piece>,
    black: HashMap<u16, Piece>,
    at: Image,
    m: Vec2,
    player: u8,
}
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let board = Image::new(ctx, "/board.png")?;
        let at = Image::new(ctx, "/at.png")?;
        let mut game = Game {
            board,
            red: HashMap::new(),
            black: HashMap::new(),
            at,                   // 选中框
            m: Vec2::new(0., 0.), // 鼠标绝对坐标
            player: 0,            // 玩家(0 & 1)
        };
        game.read_img('r', ctx);
        game.read_img('b', ctx);
        Ok(game)
    }
    fn read_img(&mut self, c: char, ctx: &mut Context) {
        let x: Vec<f32> = vec![
            300., 240., 360., 180., 420., 120., 480., 60., 540., 120., 480., 60., 180., 300., 420.,
            540.,
        ];
        let mut y: Vec<f32> = vec![
            60., 60., 60., 60., 60., 60., 60., 60., 60., 180., 180., 240., 240., 240., 240., 240.,
        ];
        if c == 'r' {
            y = y.iter().map(|x| 660. - x).collect();
        }
        for id in 0..16 {
            let path = format!("/{}{}.png", c, id + 10);
            let img = Image::new(ctx, path).expect("No file!");
            let piece = Piece {
                // id,
                // x: x[id as usize],
                // y: y[id as usize],
                img,
                // color: c,
                vec: Vec2::new(x[id as usize] - 24., y[id as usize] - 24.),
            };

            if c == 'r' {
                self.red.insert(id, piece);
            } else {
                self.black.insert(id, piece);
            }
        }
    }
    // fn find_piece() {
            
    // }
}
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // 设置背景颜色
        graphics::clear(ctx, Color::from_rgb(0xec, 0xcc, 0x68));
        // 渲染棋盘
        graphics::draw(ctx, &self.board, (Vec2::new(0.0, 0.0), 0.0, Color::WHITE))?;
        // 渲染棋子
        let b = &self.black;
        let r = &self.red;
        for id in 0..16 {
            graphics::draw(ctx, &r[&id].img, (r[&id].vec, 0.0, Color::WHITE))?;
            graphics::draw(ctx, &b[&id].img, (b[&id].vec, 0.0, Color::WHITE))?;
        }
        // 绘制选中光标at
        graphics::draw(
            ctx,
            &self.at,
            (Vec2::new(self.m.x - 30., self.m.y - 30.), 0.0, Color::WHITE),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
    // 按下鼠标按钮
    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        _button: ggez::event::MouseButton,
        _x: f32,
        _y: f32,
    ) {
        let x = (_x + 30.) as i32 / 60;
        let y = (_y + 30.) as i32 / 60;

        self.m.x = x as f32 * 60.;
        self.m.y = y as f32 * 60.;

        println!("x:{}\ty:{}", x * 60, y * 60);
    }
    // 鼠标按钮已释放
    // fn mouse_button_up_event(
    //     &mut self,
    //     _ctx: &mut Context,
    //     _button: ggez::event::MouseButton,
    //     _x: f32,
    //     _y: f32,
    // ) {
    //     // println!("x:{}\ty:{}", _x, _y);
    // }
    // 鼠标进入或左窗口区域
    // fn mouse_enter_or_leave(&mut self, _ctx: &mut Context, _entered: bool) {}
    // 移动鼠标；它在窗口中同时提供了绝对X和Y坐标，并且相对X和Y坐标与其最后一个位置相比。
    // fn mouse_motion_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32, _dx: f32, _dy: f32) {}
    // 将鼠标轮滚动，垂直滚动（y，远离用户和负面）或水平（x，右侧为阳性，右侧为正，左侧为负）。
    // fn mouse_wheel_event(&mut self, _ctx: &mut Context, _x: f32, _y: f32) {

    // }
}
