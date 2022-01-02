// トレイトオブジェクトを使用するとコンパイラは動的ディスパッチを使用する
// 実行時コスト、コードのインライン化なし
fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    "Yes".to_string(),
                    "No".to_string(),
                    "Maybe".to_string()
                ]
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: "OK".to_string()
            })
        ]
    };
    screen.run();
}

// トレイトオブジェクトはオブジェクト安全なトレイトしか作成できない
// - 戻り値の型がSelfでない
// - ジェネリックな型引数がない
pub trait Draw {
    fn draw(&self);
}

// Drawトレイトを実装するトレイトオブジェクトのベクタを保持する
pub struct Screen {
    // Box<Draw>のままだとエラーになる
    // trait objects must include the `dyn` keyword
    pub components: Vec<Box<dyn Draw>>,

}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

#[derive(Debug)]
pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String
}

impl Draw for Button {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug)]
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>
}

impl Draw for SelectBox {
    fn draw(&self) {
        println!("{:?}", self);
    }
}

// ジェネリクスの場合はDrawを実装した同種の型のコレクションしか持てない
pub struct ScreenUseGenerics<T: Draw> {
    pub components: Vec<T>,
}

impl<T> ScreenUseGenerics<T>
where
    T: Draw,
{
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}
