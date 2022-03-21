# Solana Escrow

## Solana Program vs Ethereum Smart Contract
Ethereumではユーザーの残高をすべてコントラクトに保存されるが、Solanaでは分散されたアカウントに保存される。
Solana Programを呼び出すとき動かしたいアカウントをすべてProgramに教えて処理してもらう。
Solanaプログラムは状態を持たす、処理だけ行う。状態は渡したアカウントに保管される。

## Glossary

* Account

Solana上のアカウントはいろんな意味を持ってる。
1. ウォレットとしてトークンを保管する場所もアカウント
2. プログラムもアカウントに保存される
3. プログラムのデータを保存する場所もアカウント


* BPF: Berkeley Packet Filter

https://ja.wikipedia.org/wiki/Berkeley_Packet_Filter

* PDA: Program Derived Addresses

秘密を鍵を持ってないアカウント、アドレス。Ethereumで言うとスマートコントラクトのアドレスに近いもの

* lamport

ソラナの最大の技術的影響力を持つLeslie Lamportにちなんで名付けられています。
ETHで言うGweiと同じで、小数点以下9桁まで。
```
1 lamport = 0.000000001 SOL
1 Gwei = 0.000000001 Ether
```

* SPL: Solana Program Library

ソラナ（Solana）ではSPLというトークン規格が定められており、基本的にはSPLで定義されている規格にのっとった形でトークンの発行を行います。
SolanaのSPLに該当するものとしてはイーサリアムのERC20が該当します。

## Build
```
$ cd src

$ cargo bulid-bpf
BPF SDK: /Users/dongri/.local/share/solana/install/releases/1.9.9/solana-release/bin/sdk/bpf
cargo-build-bpf child: rustup toolchain list -v
cargo-build-bpf child: cargo +bpf build --target bpfel-unknown-unknown --release
    Finished release [optimized] target(s) in 2.04s
cargo-build-bpf child: /Users/dongri/.local/share/solana/install/releases/1.9.9/solana-release/bin/sdk/bpf/dependencies/bpf-tools/llvm/bin/llvm-readelf --dyn-symbols /Users/dongri/go/src/github.com/dongri/solana/solana-escrow/target/deploy/solana_escrow.so

To deploy this program:
  $ solana program deploy /Users/dongri/go/src/github.com/dongri/solana/solana-escrow/target/deploy/solana_escrow.so
The program address will default to this keypair (override with --program-id):
  /Users/dongri/go/src/github.com/dongri/solana/solana-escrow/target/deploy/solana_escrow-keypair.json
```

## npm
```
$ npm install -g ts-node
$ npm install
```

## generate-keypair
```
$ solana-keygen new -o keys/payer.json
$ solana-keygen new -o keys/alice.json
$ solana-keygen new -o keys/bob.json

$ solana address -k keys/payer.json
FaJfHzyBJgiJcBMb8MGAEcsqQg4JmoDKyxG579VL2n2d

$ vim keys/payer.json
"FaJfHzyBJgiJcBMb8MGAEcsqQg4JmoDKyxG579VL2n2d"

$ solana address -k keys/alice.json
$ vim keys/alice_pub.json

$ solana address -k keys/bob.json
$ vim keys/bob_pub.json
```

# Deploy program
```
$ solana program deploy target/deploy/solana_escrow.so
Program Id: DcGE6p1tP1dpUqp4ENZa62XGPrPWwnZz1zqJ2DyZVLrU

$ vim keys/program_pub.json
"DcGE6p1tP1dpUqp4ENZa62XGPrPWwnZz1zqJ2DyZVLrU"
```

# Fund the transaction `payer` in advance
```
$ solana transfer FaJfHzyBJgiJcBMb8MGAEcsqQg4JmoDKyxG579VL2n2d 100 --allow-unfunded-recipient
$ solana balance FaJfHzyBJgiJcBMb8MGAEcsqQg4JmoDKyxG579VL2n2d
100 SOL
```

# Run

* run setup.ts to mint the tokens to be exchanged:
```
$ ts-node ts/setup.ts

Requesting SOL for Alice...
Requesting SOL for Bob...
Requesting SOL for Client...
Creating Mint X...
Creating Alice TokenAccount for X...
Creating Bob TokenAccount for X...
Sending 50X to Alice's X TokenAccount...
Creating Mint Y...
Creating Alice TokenAccount for Y...
Creating Bob TokenAccount for Y...
Sending 50Y to Bob's Y TokenAccount...
✨Setup complete✨

┌─────────┬───────────────────────┬───────────────────────┬─────────────────────┬─────────────────────┐
│ (index) │ Alice Token Account X │ Alice Token Account Y │ Bob Token Account X │ Bob Token Account Y │
├─────────┼───────────────────────┼───────────────────────┼─────────────────────┼─────────────────────┤
│    0    │          50           │           0           │          0          │         50          │
└─────────┴───────────────────────┴───────────────────────┴─────────────────────┴─────────────────────┘
```

* run alice.ts to initialize the escrow program:
```
$ ts-node ts/alice.ts

Sending Alice's transaction...
✨Escrow successfully initialized. Alice is offering 5X for 3Y✨

┌─────────┬───────────────────────┬───────────────────────┬─────────────────────┬─────────────────────┬───────────────────────────┐
│ (index) │ Alice Token Account X │ Alice Token Account Y │ Bob Token Account X │ Bob Token Account Y │ Temporary Token Account X │
├─────────┼───────────────────────┼───────────────────────┼─────────────────────┼─────────────────────┼───────────────────────────┤
│    0    │          45           │           0           │          0          │         50          │             5             │
└─────────┴───────────────────────┴───────────────────────┴─────────────────────┴─────────────────────┴───────────────────────────┘
```

* run bob.ts to exchange and close the escrow account:
```
$ ts-node ts/bob.ts

Sending Bob's transaction...
✨Trade successfully executed. All temporary accounts closed✨

┌─────────┬───────────────────────┬───────────────────────┬─────────────────────┬─────────────────────┐
│ (index) │ Alice Token Account X │ Alice Token Account Y │ Bob Token Account X │ Bob Token Account Y │
├─────────┼───────────────────────┼───────────────────────┼─────────────────────┼─────────────────────┤
│    0    │          45           │           3           │          5          │         47          │
└─────────┴───────────────────────┴───────────────────────┴─────────────────────┴─────────────────────┘
```
