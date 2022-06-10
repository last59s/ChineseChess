// use std::collections::HashMap;

use crate::app::Game;
// use super::_app;
use glam::Vec2;

static mut REC: [[char; 9]; 10] = [
    ['b', 'b', 'b', 'b', 'b', 'b', 'b', 'b', 'b'],
    ['+'; 9],
    ['+', 'b', '+', '+', '+', '+', '+', 'b', '+'],
    ['b', '+', 'b', '+', 'b', '+', 'b', '+', 'b'],
    ['+'; 9],
    ['+'; 9],
    ['r', '+', 'r', '+', 'r', '+', 'r', '+', 'r'],
    ['+', 'r', '+', '+', '+', '+', '+', 'r', '+'],
    ['+'; 9],
    ['r', 'r', 'r', 'r', 'r', 'r', 'r', 'r', 'r'],
];
pub fn update_rec(color: char, loc: &Vec2, m: &Vec2) {
    unsafe {
        println!("loc:\tx:{}\t,y:{}", loc.x, loc.y);
        println!("m:\tx:{}\t,y:{}", m.x, m.y);
        let mut x = loc.x / 60.;
        let mut y = loc.y / 60.;
        REC[y as usize][x as usize] = '+';
        println!("原：\tx:{}\ty:{}", x as usize, y as usize);
        // println!("x:{}\ty:{}", x, y);
        x = m.x / 60. - 1.;
        y = m.y / 60. - 1.;
        REC[y as usize][x as usize] = color;
        println!("后：x:{}\ty:{}", x, y);
    }
}

impl Game {
    pub fn which_piece(&self, id: u8) -> Option<Vec<Vec2>> {
        let v = match id {
            0 => Some(self.shuai(id)),
            1..=2=>Some(self.shi(id)),
            11..=15 => Some(self.bing(id)),
            _ => None,
        };
        v
    }
    pub fn is_friend(&self, v: &mut Vec<Vec2>) {
        let p = match self.select.1 {
            'r' => Some(&self.red),
            'b' => Some(&self.black),
            _ => None,
        };
        if let Some(p) = p {
            for (_, p) in p.iter() {
                for (i, loc) in v.iter().enumerate() {
                    if p.loc.x == loc.x && p.loc.y == loc.y {
                        v.remove(i);
                        break;
                    }
                }
            }
        }
    }
    pub fn bing(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        match self.select.1 {
            'r' => {
                let p = self.red.get(&id).unwrap();
                if p.loc.y < 330. {
                    v.push(Vec2::new(p.loc.x - 60., p.loc.y));
                    v.push(Vec2::new(p.loc.x + 60., p.loc.y));
                }
                v.push(Vec2::new(p.loc.x, p.loc.y - 60.));
            }
            'b' => {
                let p = self.black.get(&id).unwrap();
                if p.loc.y > 330. {
                    v.push(Vec2::new(p.loc.x - 60., p.loc.y));
                    v.push(Vec2::new(p.loc.x + 60., p.loc.y));
                }
                v.push(Vec2::new(p.loc.x, p.loc.y + 60.));
            }
            _ => println!("Error in bing."),
        }
        self.is_friend(&mut v);
        v
        // if self.select.1 == 'r' {
        //     let p = self.red.get(&id).unwrap();
        //     if p.loc.y < 330. {
        //         v.push(Vec2::new(p.loc.x - 60., p.loc.y));
        //         v.push(Vec2::new(p.loc.x + 60., p.loc.y));
        //     }
        //     v.push(Vec2::new(p.loc.x, p.loc.y - 60.));
        // } else if self.select.1 == 'b' {
        //     let p = self.black.get(&id).unwrap();
        //     if p.loc.y > 330. {
        //         v.push(Vec2::new(p.loc.x - 60., p.loc.y));
        //         v.push(Vec2::new(p.loc.x + 60., p.loc.y));
        //     }
        //     v.push(Vec2::new(p.loc.x, p.loc.y + 60.));
        // }
    }
    // pub fn pao(&self) {}
    // pub fn p_ju(&self) {}
    // pub fn p_ma(&self) {}
    // pub fn p_xiang(&self) {}
    pub fn shi(&self ,id:u8) ->Vec<Vec2>{
        let mut v: Vec<Vec2> = Vec::new();
        match self.select.1 {
            'r' => {
                let p = self.red.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y+60.));
                v.push(Vec2::new(p.loc.x+60., p.loc.y - 60.));
                v.push(Vec2::new(p.loc.x-60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y-60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 480. || loc.y + 24. > 600. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            'b' => {
                let p = self.black.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y+60.));
                v.push(Vec2::new(p.loc.x+60., p.loc.y - 60.));
                v.push(Vec2::new(p.loc.x-60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y-60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 60. || loc.y + 24. > 180. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            _ => println!("Error in shuai."),
        }
        for loc in v.iter_mut() {
            if loc.x + 24. < 240. || loc.x + 24. > 360. {
                loc.x = -60.;
                loc.y = -60.;
            }
        }
        self.is_friend(&mut v);
        v
    }
    pub fn shuai(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        match self.select.1 {
            'r' => {
                let p = self.red.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y));
                v.push(Vec2::new(p.loc.x, p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x, p.loc.y - 60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 480. || loc.y + 24. > 600. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            'b' => {
                let p = self.black.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y));
                v.push(Vec2::new(p.loc.x, p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x, p.loc.y - 60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 60. || loc.y + 24. > 180. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            _ => println!("Error in shuai."),
        }
        for loc in v.iter_mut() {
            if loc.x + 24. < 240. || loc.x + 24. > 360. {
                loc.x = -60.;
                loc.y = -60.;
            }
        }
        self.is_friend(&mut v);
        v
    }
}
