# kamome_armの競技プログラミング用ライブラリ

## 方針
- `cargo_snippet`を使用して、スニペットとして管理する

## 改善点
- `trait`等を使ってより汎用性を高めたい
    - スニペットとして管理することを考えたら、やりにくい気もする
        - `#[snippet(include = "")]`を使用するとインクルード先が被ったときに、削除するのは大変そう
        - `trait`類はあらかじめコードに書いておいて、`#[snippet(include = "")]`はなるべく使わないとする？
    - 命名規則がはっきりとしていない
    - ドキュメントの体裁もバラバラなので直したい
    - `test`全然書いてない

