use crate::rule::update_rec;
use ggez::event::EventHandler;
use ggez::graphics::{self, Color, Image};
use ggez::{Context, GameResult};
use glam::Vec2;
use std::collections::HashMap;

#[derive(Clone)]
pub struct Piece {
    // id: u8,
    pub color: char,
    img: Image,
    pub loc: Vec2, // 当前坐标
}
#[derive(Clone)]
pub struct Game {
    board: Image,
    at: Image,
    pub red: HashMap<u8, Piece>,
    pub black: HashMap<u8, Piece>,
    m: Vec2,                        // 鼠标坐标
    pub select: (Option<u8>, char), // 当前选择(id,颜色)
    state: bool,                    //
    player: u16,                    // 当前玩家
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
            select: (None, '+'),
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
        for id in 0..=15 {
            let path = format!("/{}{}.png", c, id + 10);
            let img = Image::new(ctx, path).expect("No file!");
            let piece = Piece {
                // id: id,
                color: c,
                img,
                loc: Vec2::new(x[id as usize] - 24., y[id as usize] - 24.),
            };

            if c == 'r' {
                self.red.insert(id, piece);
            } else {
                self.black.insert(id, piece);
            }
        }
    }
    /// 依据坐标m寻找棋子，设置状态ture
    fn piece_find(&mut self, color: char) -> bool {
        let p = match color {
            'r' => Some(self.red.iter()),
            'b' => Some(self.black.iter()),
            _ => None,
        };
        if let Some(p) = p {
            for (id, p) in p {
                if p.loc.x + 24. == self.m.x && p.loc.y + 24. == self.m.y {
                    self.state = true;
                    self.select = (Some(*id), p.color);
                    // println!("find id={}",id);
                    return true;
                }
            }
        }

        // self.select = None;
        self.select.0 = None;
        self.select.1 = '+';
        false
    }
    /// 移动棋子，并设置状态为false
    fn piece_move(&mut self) {
        if let Some(id) = self.select.0 {
            println!("move id={id}");
            let v;
            // 吃掉对方棋子 && 同颜色禁止移动
            match self.select.1 {
                'r' => {
                    for (_, p) in self.red.iter() {
                        if p.loc.x + 24. == self.m.x && p.loc.y + 24. == self.m.y {
                            // 移动失败重置状态
                            self.state = false;
                            self.select = (None, '+');
                            return;
                        }
                    }
                    v = self.which_piece(id);
                    // 移除黑棋
                    if self.update_loc(v, id) {
                        for (_, p) in self.black.iter_mut() {
                            if p.loc.x + 24. == self.m.x && p.loc.y + 24. == self.m.y {
                                p.loc = Vec2::new(-60., -60.);
                            }
                        }
                    }
                }
                'b' => {
                    for (_, p) in self.black.iter() {
                        if p.loc.x + 24. == self.m.x && p.loc.y + 24. == self.m.y {
                            // 移动失败重置状态
                            self.state = false;
                            self.select = (None, '+');
                            return;
                        }
                    }
                    v = self.which_piece(id);
                    // 移除红棋
                    if self.update_loc(v, id) {
                        println!("update");
                        for (_, p) in self.red.iter_mut() {
                            if p.loc.x + 24. == self.m.x && p.loc.y + 24. == self.m.y {
                                p.loc = Vec2::new(-60., -60.);
                            }
                        }
                    }
                }
                _ => return,
            }
        }
    }
    fn update_loc(&mut self, v: Option<Vec<Vec2>>, id: u8) -> bool {
        let p = match self.select.1 {
            'r' => Some(self.red.get_mut(&id).unwrap()),
            'b' => Some(self.black.get_mut(&id).unwrap()),
            _ => None,
        };
        if let Some(p) = p {
            if let Some(v) = v {
                for _v in v {
                    if _v.x == self.m.x - 24. && _v.y == self.m.y - 24. {
                        // 被选中棋子更新
                        let loc = Vec2::new(self.m.x - 24., self.m.y - 24.);
                        update_rec(p.color, &p.loc, &self.m);
                        p.loc = loc;
                        // 重置状态
                        self.state = false;
                        self.player += 1;
                        return true;
                    }
                }
            }
        }
        false
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
            graphics::draw(ctx, &r[&id].img, (r[&id].loc, 0.0, Color::WHITE))?;
            graphics::draw(ctx, &b[&id].img, (b[&id].loc, 0.0, Color::WHITE))?;
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
        // println!("x:{}\ty:{}",x,y);
        // 防止选定框出界
        if x > 0 && x < 10 && y > 0 && y < 11 {
            self.m.x = x as f32 * 60.;
            self.m.y = y as f32 * 60.;
            // println!("x:{}\ty:{}", self.m.x, self.m.y);
        }
        // 棋子移动
        match self.player % 2 {
            0 => {
                if !self.state {
                    self.piece_find('r');
                    // println!("{:?}", self.select);
                } else {
                    self.piece_move();
                }
                println!("{}", self.player);
            }
            1 => {
                if !self.state {
                    self.piece_find('b');
                    // println!("{:?}", self.select);
                } else {
                    self.piece_move();
                }
                println!("{}", self.player);
            }
            _ => panic!("Player error!"),
        }
    }
}
