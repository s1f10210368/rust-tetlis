use std::{thread, time};
use getch_rs::{Getch, Key};

// フィールドの大きさを定義
const FIELD_WIDTH:  usize = 11 + 2; // フィールド + 壁
const FIELD_HEIGHT: usize = 20 + 1; // フィールド + 底
type Field = [[usize; FIELD_WIDTH]; FIELD_HEIGHT];

// ブロックの種類定義
#[derive(Clone, Copy)]
enum BlockKind {
    I,
    O,
    S,
    Z,
    J,
    L,
    T,
}

type BlockShape = [[usize; 4]; 4];
const BLOCKS: [BlockShape; 7] = [
    // Iブロック
    [
        [0,0,0,0],
        [0,0,0,0],
        [1,1,1,1],
        [0,0,0,0],
    ],
    // Oブロック
    [
        [0,0,0,0],
        [0,1,1,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Sブロック
    [
        [0,0,0,0],
        [0,1,1,0],
        [1,1,0,0],
        [0,0,0,0],
    ],
    // Zブロック
    [
        [0,0,0,0],
        [1,1,0,0],
        [0,1,1,0],
        [0,0,0,0],
    ],
    // Jブロック
    [
        [0,0,0,0],
        [1,0,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Lブロック
    [
        [0,0,0,0],
        [0,0,1,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
    // Tブロック
    [
        [0,0,0,0],
        [0,1,0,0],
        [1,1,1,0],
        [0,0,0,0],
    ],
];

struct Position {
    x: usize,  // usize型は組み込み整数型の1つであり、ここでは座標を表すために定義。usizeは非負整数を表すのに適している
    y: usize,
}

// ブロックがフィールドに衝突する場合は`true`を返す
fn is_collision(field: &Field, pos: &Position, block: BlockKind) -> bool { // &をつけると共有参照といい実引数の値を不変参照している
    for y in 0..4 {
        for x in 0..4 {
            if field[y+pos.y][x+pos.x] & BLOCKS[block as usize][y][x] == 1 {
                return true;
            }
        }
    }
    false
}

fn main() {
    let field = [
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,0,0,0,0,0,0,0,0,0,0,0,1],
        [1,1,1,1,1,1,1,1,1,1,1,1,1],
    ];

    let mut pos = Position { x: 4, y:0 };
    let g = Getch::new();

    // 画面クリア
    println!("\x1b[2J\x1b[H\x1b[?25l");

/*
    \x1b[H	    カーソルを画面の一番左上へ移動する
    \x1b[2J	    画面をクリアする
    \x1b[?25h	カーソルを表示にする
    \x1b[?25l	カーソルを非表示にする
    これらは組み合わせて使用することも可能
*/

    loop{
        // ブロックを描画するための新たなフィールドを生成
        let mut field_buf = field; // mutがついていると可変であるということを意味する

        // 自由落下
        let new_pos = Position {
            x: pos.x,
            y: pos.y + 1,
        };

        // 当たり判定
        if !is_collision(&field, &new_pos, BlockKind::I){ // 関数の前に!をつけるとそれがfalseの時に中身が実行される
            // posの座標を更新
            pos = new_pos;
        }

        // field_buf にブロックの情報を書き込む
        for y in 0..4 {
            for x in 0..4 {
                if BLOCKS[BlockKind::I as usize][y][x] == 1 {
                    field_buf[y+pos.y][x+pos.x] = 1;
                }
            }
        }

        // フィールドを描画
        println!("\x1b[H"); // カーソルを先頭に移動

        // フィールドを描画
        for y in 0..FIELD_HEIGHT { //縦方向
            for x in 0..FIELD_WIDTH { // 横方向 // print!は改行なしで標準出力
                if field_buf[y][x] == 1 {
                    print!("[]");
                }else{
                    print!(" .");
                }
            }
            println!();
        }
        // 1秒間スリーブする
        thread::sleep(time::Duration::from_millis(1000));

        // 移動キーの入力待ち
        match g.getch() {
            Ok(Key::Left) => {
                let new_pos = Position {
                    x: pos.x -1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // posの座標を更新
                    pos = new_pos;
                }
            }
            Ok(Key::Down) => {
                let new_pos = Position {
                    x: pos.x,
                    y: pos.y + 1,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // 座標更新
                    pos = new_pos;
                }
            }
            Ok(Key::Right) => {
                let new_pos = Position {
                    x: pos.x + 1,
                    y: pos.y,
                };
                if !is_collision(&field, &new_pos, BlockKind::I) {
                    // 座標更新
                    pos = new_pos;
                }
            }
            Ok(Key::Char('q')) => break,
            _ => (),
        }
    }
    // カーソルを再び表示
    println!("\x1b[?25h");
}
