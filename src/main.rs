mod block;
mod game;

use std::sync::{Arc, Mutex}; // スレッド間で安全に変数の参照と変更をしたいため
use std::{thread, time};
use getch_rs::{Getch, Key};
use game::*;

fn main() {
    let game = Arc::new(Mutex::new(Game::new()));

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

    // フィールド描画
    draw(&game.lock().unwrap());

/*
    \x1b[H	    カーソルを画面の一番左上へ移動する
    \x1b[2J	    画面をクリアする
    \x1b[?25h	カーソルを表示にする
    \x1b[?25l	カーソルを非表示にする
    これらは組み合わせて使用することも可能
*/

    // 自然落下処理
    {
        let game = Arc::clone(&game);
        let _ = thread::spawn(move || {
            loop {
                // 1秒スリープ
                thread::sleep(time::Duration::from_millis(1000));
                // 自然落下
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y+1,
                };
                if !is_collision(&game.field, &new_pos, &game.block){
                    // posの座標を更新
                    game.pos = new_pos; // 更新する際にアクセスしたいためポイント演算子を追加
                } else {
                    if landing(&mut game).is_err(){
                        // ブロックを生成できないならゲームオーバー
                        gameover(&game);
                        break;
                    }
                }
                //フィールドを更新
                draw(&game);
            }
        });
    }

    // キーの入力処理
    let g = Getch::new();
    loop {
        // 移動キーの入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x.checked_sub(1).unwrap_or(game.pos.x),
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Down) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x,
                    y: game.pos.y + 1,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Right) => {
                let mut game = game.lock().unwrap();
                let new_pos = Position {
                    x: game.pos.x + 1,
                    y: game.pos.y,
                };
                move_block(&mut game, new_pos);
                draw(&game);
            }
            Ok(Key::Char('a')) => {
                // 左回転
                let mut game = game.lock().unwrap();
                rotate_left(&mut game);
                draw(&game);
            }
            Ok(Key::Char('d')) => {
                // 右回転
                let mut game = game.lock().unwrap();
                rotate_right(&mut game);
                draw(&game);
            }
            Ok(Key::Char('q')) => {
                break;
            }
            Ok(Key::Up) => {
                // ハードドロップ
                let mut game = game.lock().unwrap();
                hard_drop(&mut game);
                if landing(&mut game).is_err() {
                    // ブロックを生成できないならゲームオーバー
                    gameover(&game);
                    break;
                }
                draw(&game);
            }
            _ => (),
        }
    }

    // 終了処理
    quit();
}
