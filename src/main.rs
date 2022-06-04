use ggez::event::{self};
use ggez::graphics;
use std::{env, path};
mod app;
use app::Game;

fn main() {
    // 资源路径
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("res");
        path
    } else {
        path::PathBuf::from("./res")
    };
    // 设置默认窗口启动模式
    let win_mode = ggez::conf::WindowMode {
        width: 700.,
        height: 660.,
        ..Default::default()
    };
    // 创建上下文。
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("Chess", "last59s")
        .window_mode(win_mode)
        .add_resource_path(resource_dir)
        .build()
        .expect("System error!");

    graphics::set_window_title(&ctx, "Chinses Chess");
    graphics::set_window_icon(&mut ctx, Some("/r10.png")).unwrap();
    // 创建事件处理程序的实例。
    // 通常，您应该在设置游戏时为其提供上下文对象。
    let state = Game::new(&mut ctx).expect("Game error!");

    // Run!
    event::run(ctx, event_loop, state);
}
