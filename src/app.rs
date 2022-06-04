use ggez::event::EventHandler;
use ggez::graphics::{self, Color, Image};
use ggez::{Context, GameResult};
use glam::Vec2;
use std::collections::HashMap;

// #[derive(Debug)]
// #[derive(Clone)]
struct Piece {
    // id: u8,
    // selected: bool,
    // x: f32,
    // y: f32,
    img: Image,
    // color: char,
    vec: Vec2,
}
// #[derive(Debug)]
pub struct Game {
    board: Image,
    at: Image, // 选中框
    red: HashMap<u8, Piece>,
    black: HashMap<u8, Piece>,
    m: Vec2,     // 鼠标绝对坐标
    select: u8,  //(id，颜色)
    state: bool, // 状态
    player: u8,  // 玩家(0 & 1)
}
impl Game {
    pub fn new(ctx: &mut Context) -> GameResult<Game> {
        let board = Image::new(ctx, "/board.png")?;
        let at = Image::new(ctx, "/at.png")?;
        let mut game = Game {
            board,
            at,
            red: HashMap::new(),
            black: HashMap::new(),
            select: 0,
            state: false,
            m: Vec2::new(-60., -60.),
            player: 0,
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
                // selected: false,
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
    // 返回棋子ip，并设置状态ture
    fn piece_select(&mut self, color: char) -> bool {
        let p;
        if color == 'r' {
            p = self.red.iter();
        } else {
            p = self.black.iter();
        }
        for (id, p) in p {
            if p.vec.x + 24. == self.m.x && p.vec.y + 24. == self.m.y {
                self.select = *id;
                self.state = true;
                println!("{}:{}", color, self.select);
                return true;
            }
        }
        false
    }
    // 移动棋子，并设置状态为false
    fn piece_move(&mut self, color: char) -> Option<()> {
        // 吃掉对方棋子
        let i = self.select;
        let p;
        if color == 'r' {
            self.piece_find_and_delect('b');
            p = self.red.get_mut(&i)?;
        } else {
            self.piece_find_and_delect('r');
            p = self.black.get_mut(&i)?;
        }
        // 更新坐标
        let vec = Vec2::new(self.m.x - 24., self.m.y - 24.);
        p.vec = vec;
        // 恢复状态
        self.state = false;
        self.player += 1;
        Some(())
    }
    fn piece_find_and_delect(&mut self, color: char) {
        let p;
        if color == 'r' {
            p = self.red.iter_mut();
        } else {
            p = self.black.iter_mut();
        }
        for (_, p) in p {
            if p.vec.x + 24. == self.m.x && p.vec.y + 24. == self.m.y {
                p.vec = Vec2::new(-60., -60.);
            }
        }
    }
}
impl EventHandler for Game {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        // if  {
        // 帅将x坐标<0,游戏结束
        // }
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
        // 绘制选定框at
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
        // 防止选定框出界
        if x > 0 && x < 10 && y > 0 && y < 11 {
            self.m.x = x as f32 * 60.;
            self.m.y = y as f32 * 60.;
        }
        // 棋子移动
        match self.player % 2 {
            0 => {
                if !self.state {
                    self.piece_select('r');
                } else {
                    self.piece_move('r');
                }
                println!("{}", self.player);
            }
            1 => {
                if !self.state {
                    self.piece_select('b');
                } else {
                    self.piece_move('b');
                }
                println!("{}", self.player);
            }
            _ => panic!("Player error!"),
        }
    }
}
