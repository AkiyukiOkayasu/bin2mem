# bin2mem

Gowin FPGA上のブロックメモリ(SP/DP/SDPB/ROMなど)初期化用 `.hex` ファイルを生成するためのツールです。
RISC-V (PicoRV32) のバイナリファイル (`.bin`) を、Gowin EDAが要求するASCII Hex形式に変換します。

## 概要

- 入力: バイナリファイル (`.bin`)
- 出力: ASCII Hexファイル (`.hex`) - リトルエンディアン 32bit単位, 改行区切り

## Install

```bash
cargo install --git https://github.com/AkiyukiOkayasu/bin2mem
```

## 使い方

```bash
bin2mem <input.bin> <output.hex>
```

## テスト

以下のコマンドでユニットテストを実行できます。

```bash
cargo test
```
