# Hello World on Solana

## Install

### Rust
https://www.rust-lang.org/tools/install
```
$ curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

$ rustup component add rustfmt

$ rustup update
$ rustup install 1.59.0
$ rustc -V
rustc 1.59.0 (9d1b2106e 2022-02-23)
$ cargo -V
cargo 1.59.0 (49d8809dc 2022-02-10)
```

### solana-cli
https://docs.solana.com/cli/install-solana-cli-tools
```
$ sh -c "$(curl -sSfL https://release.solana.com/v1.9.11/install)"

$ solana -V
solana-cli 1.9.11 (src:450404f8; feat:3246413280)
```

### node
```
$ curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.34.0/install.sh | bash
$ source ~/.bashrc

$ nvm ls-remote
$ nvm install v14.15.3
$ nvm alias default v14.15.3

$ node -v
v14.15.3
```

### Configure CLI
```
$ solana config set --url localhost
```
### Create new keypair
```
$ solana-keygen new
```

### Start local Solana cluster
```
$ solana-test-validator
```

### Install npm dependencies
```
$ npm install
```

### build rust
```
$ npm run build:program-rust
```

### Deploy the on-chain program
```
$ solana program deploy dist/program/helloworld.so
Program Id: AodEjGKNQTTbotHS7yBqzhBkwKD9DCyP8zV8vda1zRoF

```

### Run front-end
```
$ npm run start

> helloworld@0.0.1 start /Users/dongri/go/src/github.com/dongri/solana/example-helloworld
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 3246413280, 'solana-core': '1.9.9' }
Using account 8mf8aVsYYwMDUAXs1HehgUZqYrcdbUkzaiKfbW7LQLJC containing 499999999.1528037 SOL to pay for fees
Using program AodEjGKNQTTbotHS7yBqzhBkwKD9DCyP8zV8vda1zRoF
Saying hello to 7C8pfyFoMFc6n1DKrbKEfPdiJG8p29fj1Vx58y5hLyR9
7C8pfyFoMFc6n1DKrbKEfPdiJG8p29fj1Vx58y5hLyR9 has been greeted 5 time(s)
Success
```

# Chagne Account
```
$ vim src/client/hello_world.ts

const GREETING_SEED = 'hello2';
```

### Rerun
```
$ npm run start

> helloworld@0.0.1 start /Users/dongri/go/src/github.com/dongri/solana/example-helloworld
> ts-node src/client/main.ts

Let's say hello to a Solana account...
Connection to cluster established: http://localhost:8899 { 'feature-set': 3246413280, 'solana-core': '1.9.9' }
Using account 8mf8aVsYYwMDUAXs1HehgUZqYrcdbUkzaiKfbW7LQLJC containing 499999999.15278375 SOL to pay for fees
Using program AodEjGKNQTTbotHS7yBqzhBkwKD9DCyP8zV8vda1zRoF
Creating account 3J2QQKoxCJsiHKr5C1uEnGt5mWnJqs5UKk6XJZxfSDT3 to say hello to
Saying hello to 3J2QQKoxCJsiHKr5C1uEnGt5mWnJqs5UKk6XJZxfSDT3
3J2QQKoxCJsiHKr5C1uEnGt5mWnJqs5UKk6XJZxfSDT3 has been greeted 1 time(s)
Success
```
(count was reset.)

https://solana-labs.github.io/solana-web3.js/classes/PublicKey.html#createWithSeed

Derive a public key from another key, a seed, and a program ID. The program ID will also serve as the owner of the public key, giving it permission to write data to the account.
