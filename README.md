# WebAssembly on Rust for kintone customize

この OSS は私個人の著作物であり，サイボウズ株式会社，その他，私の所属する組織とは一切関係ありません。

This OSS is my own personal work and does not have any relationship with Cybozu Inc. or any other organization which I belong to.

## ビルド方法

    $ ./build.sh

./pkg 以下に Wasm ファイルおよび JS バインディングが生成されます。

GitHub Actions を使って自動ビルドし，GitHub Pages に成果物を配置しています。
- https://uchan-nos.github.io/rust-wasm-kintone/pkg/rust-wasm-kintone.js
- https://uchan-nos.github.io/rust-wasm-kintone/pkg/rust-wasm-kintone_bg.wasm
- https://uchan-nos.github.io/rust-wasm-kintone/kintone/customize.js

## kintone カスタマイズの適用方法

このリポジトリで公開している kintone カスタマイズを適用するには 2 つの作業が必要です。
1. kintone にカスタマイズ適用対象のアプリを作る
2. JavaScript カスタマイズを設定する

### カスタマイズ適用対象のアプリを作る

次のフィールドを持ったアプリを作ります。

| フィールドコード | フィールドタイプ | 役割 |
|------------------|------------------|------|
| name             | 文字列（1行）    | `greet()` 関数の引数に指定する文字列 |
| greet            | スペース         | `greet()` 関数の戻り値を表示する領域 |

### JavaScript カスタマイズを設定する

作成したアプリの設定から「JavaScript / CSSでカスタマイズ」画面を開き， `pkg/rust-wasm-kintone.js` と `kintone/customize.js` をこの順で指定します。

![register javascript files to kintone](https://github.com/uchan-nos/rust-wasm-kintone/blob/main/docs/images/register-js-files.png?raw=true)
