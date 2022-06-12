use crate::app::Game;
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
        // println!("loc:\tx:{}\t,y:{}", loc.x, loc.y);
        // println!("m:\tx:{}\t,y:{}", m.x, m.y);
        let mut x = loc.x / 60.;
        let mut y = loc.y / 60.;
        REC[y as usize][x as usize] = '+';
        // println!("原：\tx:{}\ty:{}", x as usize, y as usize);
        x = m.x / 60. - 1.;
        y = m.y / 60. - 1.;
        REC[y as usize][x as usize] = color;
        // println!("后：x:{}\ty:{}", x, y);
    }
}

impl Game {
    pub fn which_piece(&self, id: u8) -> Option<Vec<Vec2>> {
        let v = match id {
            0 => Some(self.shuai(id)),
            1..=2 => Some(self.shi(id)),
            3..=4 => Some(self.xiang(id)),
            5..=6 => Some(self.ma(id)),
            1..=8 => Some(self.che(id)),
            9..=10 => Some(self.pao(id)),
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
    }
    pub fn pao(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        let mut c = '+'; // 对方棋子颜色
        let p = if self.select.1 == 'r' {
            c = 'b';
            Some(self.red.get(&id).unwrap())
        } else if self.select.1 == 'b' {
            c = 'r';
            Some(self.black.get(&id).unwrap())
        } else {
            None
        };
        if let Some(p) = p {
            unsafe {
                let x = (p.loc.x / 60.) as i32;
                let y = (p.loc.y / 60.) as i32;
                let mut _state = false;
                for i in (0..x).rev() { // 左
                    if REC[y as usize][i as usize] != '+' {
                        if REC[y as usize][i as usize] == c && _state == true {
                            v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                            _state = false;
                            break;
                        }
                        _state = true;
                    }
                    if _state == false {
                        v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                    }
                }
                _state = false;
                for i in x + 1..9 {     //右
                    if REC[y as usize][i as usize] != '+' {
                        if REC[y as usize][i as usize] == c && _state == true {
                            v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                            _state = false;
                            break;
                        }
                        _state = true;
                    }
                    if _state == false {
                        v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                    }
                }
                _state = false;
                for i in (0..y).rev() { // 上
                    if REC[i as usize][x as usize] != '+' {
                        if REC[i as usize][x as usize] == c && _state == true {
                            v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                            println!("3-vec1\tx:{}y:{}",p.loc,((i + 1) * 60 - 24) as f32);
                            _state = false;
                            break;
                        }
                        _state = true;
                    }
                    if _state == false {
                        println!("3-vec2\tx:{}y:{}",p.loc,((i + 1) * 60 - 24) as f32);
                        v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                    }
                }
                _state = false;
                for i in y + 1..10 {    // 下
                    if REC[i as usize][x as usize] != '+' {
                        if REC[i as usize][x as usize] == c && _state == true {
                            v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                            _state = false;
                            break;
                        }
                        _state = true;
                    }
                    if _state == false {
                        v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                    }
                }
            }
        }
        v
    }
    pub fn che(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        let mut c = '+'; // 对方棋子颜色
        let p = if self.select.1 == 'r' {
            c = 'b';
            Some(self.red.get(&id).unwrap())
        } else if self.select.1 == 'b' {
            c = 'r';
            Some(self.black.get(&id).unwrap())
        } else {
            None
        };
        if let Some(p) = p {
            unsafe {
                let x = (p.loc.x / 60.) as i32;
                let y = (p.loc.y / 60.) as i32;
                for i in (0..x).rev() {
                    if REC[y as usize][i as usize] == p.color {
                        break;
                    } else if REC[y as usize][i as usize] == c {
                        v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                        break;
                    }
                    v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                }
                for i in x + 1..9 {
                    if REC[y as usize][i as usize] == p.color {
                        break;
                    } else if REC[y as usize][i as usize] == c {
                        v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                        break;
                    }
                    v.push(Vec2::new(((i + 1) * 60 - 24) as f32, p.loc.y));
                }
                for i in (0..y).rev() {
                    if REC[i as usize][x as usize] == p.color {
                        break;
                    } else if REC[i as usize][x as usize] == c {
                        v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                        break;
                    }
                    v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                }
                for i in y + 1..10 {
                    if REC[i as usize][x as usize] == p.color {
                        break;
                    } else if REC[i as usize][x as usize] == c {
                        v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                        break;
                    }
                    v.push(Vec2::new(p.loc.x, ((i + 1) * 60 - 24) as f32));
                }
            }
        }
        v
    }
    pub fn ma(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        unsafe {
            let target = [
                ([-1, 2], [1, 2]),
                ([2, 1], [2, -1]),
                ([-2, 1], [-2, -1]),
                ([1, -2], [-1, -2]),
            ];
            let obstacle = [[0, 1], [1, 0], [-1, 0], [0, -1]]; // 下，右，左，上
            let p = if self.select.1 == 'r' {
                Some(self.red.get(&id).unwrap())
            } else if self.select.1 == 'b' {
                Some(self.black.get(&id).unwrap())
            } else {
                None
            };
            if let Some(p) = p {
                for i in 0..4 {
                    let x = (p.loc.x / 60.) as i32 + obstacle[i][0];
                    let y = (p.loc.y / 60.) as i32 + obstacle[i][1];
                    if x < 0 || x > 8 || y < 0 || y > 9 {
                        continue;
                    }
                    if REC[y as usize][x as usize] == '+' {
                        let mut _x = p.loc.x + (target[i].0[0] as f32) * 60.;
                        let mut _y = p.loc.y + (target[i].0[1] as f32) * 60.;
                        v.push(Vec2::new(_x, _y));
                        _x = p.loc.x + (target[i].1[0] as f32) * 60.;
                        _y = p.loc.y + (target[i].1[1] as f32) * 60.;
                        v.push(Vec2::new(_x, _y));
                    }
                }
            }
        }
        self.is_friend(&mut v);
        v
    }
    pub fn xiang(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        let mut rec = [[0, 2], [0, 6], [2, 0], [2, 4], [2, 8], [4, 2], [4, 6]];
        let p = if self.select.1 == 'r' {
            for i in rec.iter_mut() {
                i[1] = 9 - i[1];
            }
            Some(self.red.get(&id).unwrap())
        } else if self.select.1 == 'b' {
            Some(self.black.get(&id).unwrap())
        } else {
            None
        };
        if let Some(p) = p {
            let x = (p.loc.x / 60.) as usize;
            let y = (p.loc.y / 60.) as usize;
            unsafe {
                for i in rec.iter() {
                    if REC[(y + i[1]) / 2][(x + i[0]) / 2] == '+' {
                        let _x = (i[0] as f32 + 1.) * 60. - 24.;
                        let _y = (i[1] as f32 + 1.) * 60. - 24.;
                        v.push(Vec2::new(_x, _y));
                    }
                }
            }
        }
        self.is_friend(&mut v);
        v
    }
    pub fn shi(&self, id: u8) -> Vec<Vec2> {
        let mut v: Vec<Vec2> = Vec::new();
        match self.select.1 {
            'r' => {
                let p = self.red.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x + 60., p.loc.y - 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y - 60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 480. || loc.y + 24. > 600. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            'b' => {
                let p = self.black.get(&id).unwrap();
                v.push(Vec2::new(p.loc.x + 60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x + 60., p.loc.y - 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y + 60.));
                v.push(Vec2::new(p.loc.x - 60., p.loc.y - 60.));
                for loc in v.iter_mut() {
                    if loc.y + 24. < 60. || loc.y + 24. > 180. {
                        loc.x = -60.;
                        loc.y = -60.;
                    }
                }
            }
            _ => println!("Error in shi."),
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
