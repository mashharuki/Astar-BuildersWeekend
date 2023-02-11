# Astar-BuildersWeekend

Astar ✖️ BuildersWeekend で構築する Dapp 用のリポジトリです。

### WebAssembly とは

WebAssembly はブラウザで JavaScript 以外の言語を実行するための技術で、Rust などで書いたものをバイナリコードに変換して、ブラウザで実行できる技術です。スタックベース仮想マシン用のバイナリフォーマットの実行環境。仮想マシン用の ISA。どんな物理マシンでも(つまり OS に依存しない)動くこと。W3C 勧告。Mozira や Google なども開発している。

### バイナリ・フォーマットとは

テキストフォーマットのように、特定の環境に依存することなく、テキストエディタでデータを読み書きすることはできないが、代わりにデータの記録方式はプログラムで任意に決定できるため、情報を効率よく構造化することが可能になる。

### ISA

ISA もバイナリフォーマット(特定の OS への命令文)

- arm64 など

### WASM とは

WebAssembly のコードを実行できる環境のこと。

### WASI(WebAssembly System Interface)とは

ブロックチェーンで使われている WebAssembly Runtime を紹介する前に、Web 以外で WebAssembly を利用するための標準化の取り組みである WASI について紹介します。WASI は WebAssembly System Interface のことで、WebAssembly をブラウザ以外の環境で実行するため、ホストのファイルシステムやネットワークなどの OS 機能へのアクセスを提供するための仕様です。W3C が勧告。docker でやろうとしていたことをもっと効率良くできる。

### ERC20PresetMinterPauser とは

ERC20 の基本機能に発行及び停止機能を追加した規格。初心者には優しいが、トークンセールの機能を実装を行う場合には ERC20 を使った方が良い。

### Why WebAssembly for Smart Contracts?

- High performance: Wasm is high performance — it’s built to be as close to native machine code as possible while still being platform independent.
- Small size: It facilitates small binaries to ship over the internet to devices with potentially slow internet connection. This is a great fit for the space-constrainted blockchain world.
- General VM & bytecode: It was developed so that code can be deployed in any browser with the same result. Contrary to the EVM it was not developed towards a very specific use case, this has the benefit of a lot of tooling being available and large companies putting a lot of resources into furthering Wasm development.
- Efficient JIT execution: 64 and 32-bit integer operation support that maps one-to-one with CPU instructions.
- Minimalistic: Formal spec that fits on a single page.
- Deterministic execution: Wasm is easily made deterministic by removing floating point operations, which is necessary for consensus algorithms.
- Open Standards > Custom Solutions: Wasm is a standard for web browsers developed by W3C workgroup that includes Google, Mozilla, and others. There’s been many years of work put into Wasm, both by compiler and standardisation teams.
- Many languages available: Wasm expands the family of languages available to smart contract developers to include Rust, C/C++, C#, Typescript, Haxe, and Kotlin. This means you can write smart contracts in whichever language you’re familiar with, though we’re partial to Rust due to its lack of runtime overhead and inherent security properties.
- Memory-safe, sandboxed, and platform-independent.
- LLVM support: Supported by the LLVM compiler infrastructure project, meaning that Wasm benefits from over a decade of LLVM’s compiler optimisation.
- Large companies involved: Continually developed by major companies such as Google, Apple, Microsoft, Mozilla, and Facebook.

### Swanky コマンドでサンプルプロジェクトを作成するコマンド

```bash
swanky init test_project
```

result

```bash
? Which contract language should we use? ink
? Which contract template should we use initially? flipper
? What should we name your initial contract? flipper
? What is your name? mashharuki
? What is your email?
? Do you want to download Swanky node? Yes
✔ Checking dependencies OK
✔ Copying template files OK
✔ Processing templates OK
✔ Initializing git OK
✔ Downloading Swanky node OK
✖ Error Installing dependencies
✔ Writing config OK
🎉 😎 Swanky project successfully initialised! 😎 🎉
```

### 生成した成果物には`.git`ファイルが含まれているので削除するコマンド

```bash
sudo rm -r .git
```

### compile command

```bash
swanky contract compile flipper -v
```

result

```bash
======== Compiled flipper ========
======== Compiled all contracts ========
======== Compiling Typechain' code ========
======== Compiled Typechain' code ========
✔ Contract compiled successfully
✔ Copying artifacts OK
✔ Writing config OK
```

### compile command (by +nightly)

```bash
cargo +nightly contract build
```

### swanky check command

```bash
swanky check
```

result

```bash
✔ Check Rust
✔ Check cargo
✔ Check cargo nightly
✔ Check cargo dylint
✔ Check cargo-contract
✔ Read ink dependencies
✔ Verify ink version
{
  tools: {
    rust: 'rustc 1.66.0 (69f9c33d7 2022-12-12)',
    cargo: 'cargo 1.66.0 (d65d197ad 2022-11-15)',
    cargoNightly: 'cargo 1.68.0-nightly (cc0a32087 2022-12-14)',
    cargoDylint: 'cargo-dylint 2.1.1',
    cargoContract: 'cargo-contract 1.5.1-unknown-aarch64-apple-darwin'
  },
  contracts: {
    flipper: {
      ink_primitives: '3.3.1',
      ink_metadata: '3.3.1',
      ink_env: '3.3.1',
      ink_storage: '3.3.1',
      ink_lang: '3.3.1',
      ink_prelude: '3.3.1',
      ink_engine: '3.3.1'
    }
  }
}
```

### test command

```bash
cd contracts/flipper/ && cargo +nightly test
```

result

```bash
running 2 tests
test flipper::tests::it_works ... ok
test flipper::tests::default_works ... ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

### start local node command

```bash
swanky node start
```

result

```bash
2022-12-18 15:56:52 Swanky Node
2022-12-18 15:56:52 ✌️  version 0.9.1-58cf6cc6280
2022-12-18 15:56:52 ❤️  by Astar Network, 2022-2022
2022-12-18 15:56:52 📋 Chain specification: Development
2022-12-18 15:56:52 🏷  Node name: incredible-water-2846
2022-12-18 15:56:52 👤 Role: FULL
2022-12-18 15:56:52 💾 Database: RocksDb at /Users/harukikondo/Library/Application Support/swanky-node/chains/dev/db/full
2022-12-18 15:56:52 ⛓  Native runtime: swanky-node-2 (swanky-node-1.tx1.au1)
2022-12-18 15:56:52 🔨 Initializing Genesis block/state (state: 0x3b6b…37b9, header-hash: 0x6644…9bd6)
2022-12-18 15:56:52 Using default protocol ID "sup" because none is configured in the chain specs
2022-12-18 15:56:52 🏷  Local node identity is: 12D3KooWS9rhYQAb8eViLZAqQ98eZpDk3CxMXdUqJ4rwfe59WtjQ
2022-12-18 15:56:52 💻 Operating system: macos
2022-12-18 15:56:52 💻 CPU architecture: x86_64
2022-12-18 15:56:52 📦 Highest known block at #0
2022-12-18 15:56:52 〽️ Prometheus exporter started at 127.0.0.1:9615
2022-12-18 15:56:52 Running JSON-RPC HTTP server: addr=127.0.0.1:9933, allowed origins=Some(["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org", "https://contracts-ui.substrate.io/"])
2022-12-18 15:56:52 Running JSON-RPC WS server: addr=127.0.0.1:9944, allowed origins=Some(["http://localhost:*", "http://127.0.0.1:*", "https://localhost:*", "https://127.0.0.1:*", "https://polkadot.js.org", "https://contracts-ui.substrate.io/"])
2022-12-18 15:56:52 creating instance on iface 192.168.70.197
```

### deploy contract command

```bash
swanky contract deploy flipper --gas 10000000000 --args false --account alice
```

result

```bash
✔ Initialising OK
✔ Getting WASM OK
✔ Connecting to node OK
✔ Deploying OK
✔ Writing config OK
Contract deployed!
Contract address: 5G6D5daPHFgqph7pkbRsz5GiPW7ZNqUNffk6ixe1pKxUaeSj
```

### deployed contract test

```bash
swanky contract call --contractName=flipper -m get -t 1671346660649
```

result

```bash
    Success! Gas required estimated at 18749980672
Confirm transaction details: (skip with --skip-confirm)
     Message get
        Args
   Gas limit 100000000000
Submit? (Y/n): 2022-12-18T06:59:17.964196Z  WARN jsonrpsee_core::client::async_client: Custom("[backend]: frontend dropped; terminate client")
Y

       Event Balances ➜ Withdraw
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         amount: 86298152
       Event TransactionPayment ➜ TransactionFeePaid
         who: 5GrwvaEF5zXb26Fz9rcQpDWS57CtERHpNehXCPcNoHGKutQY
         actual_fee: 86298152
         tip: 0
       Event System ➜ ExtrinsicSuccess
         dispatch_info: DispatchInfo { weight: 3024627592, class: Normal, pays_fee: Yes }
```

### deploy command to shibuya network

```bash
swanky contract deploy flipper --gas 10000000000 --args false --account test_2 -n shibuya
```

result

```bash
✔ Initialising OK
✔ Getting WASM OK
⠏ Connecting to node2022-12-18 16:15:29        API/INIT: RPC methods not decorated: transaction_unstable_submitAndWatch, transaction_unstable_unwatch
✔ Connecting to node OK
✔ Deploying OK
✔ Writing config OK
Contract deployed!
Contract address: WVLMRqYtaqqv5wiEonHnH8LMXv7TJujF4qMLQz2ncnAcZxt
```

### polkadot_js_example のバージョンについて

4.33 だとうまく動く。それ以外だと動かない。

### ※注意事項※

`sudo`をつけてやること。

### メモ

- Defi の席は空いている。EVM と WASM を生きできると良い。
- ハッカソンを見ると求めているのがわかる。
- 最近は WASM にフォーカスしている。

### !ink の静的テストコードについて

V3 では、コントラクトからコントラクトを呼び出すものはできない。V4 ではでき始めている。

### 参考文献

1. [How to develop a Smart Contract on Astar Network](https://docs.google.com/presentation/d/1nNqcABdysLvQzDEAfvCjFiXWCoQs1jCGJAGkkCZfBiw/edit#slide=id.p)
2. [Hands On of EVM Smart Contract](https://docs.google.com/presentation/d/1IgtVSS_1NkZW-rBaZohmnnnpQgSdBI1l2t1LMOJvNzo/edit#slide=id.p)
3. [Preparing for tomorrow](https://docs.google.com/presentation/d/189FgVkQPWXCd50uLjTPbE_zXh9twHN5MSv3Fcf3R588/edit#slide=id.p)
4. [Astar Docs](https://docs.astar.network/)
5. [ethereum evm illustrated](https://takenobu-hs.github.io/downloads/ethereum_evm_illustrated.pdf)
6. [Shibuya faucet](https://discord.com/channels/644182966574252073/856241970358910976)
7. [Remix](https://remix.ethereum.org/)
8. [hardhat](https://hardhat.org/)
9. [foundry](https://www.paradigm.xyz/2021/12/introducing-the-foundry-ethereum-development-toolbox)
10. [openzeppelin](https://www.openzeppelin.com/)
11. [ERC20PresetMinterPauser.sol](https://github.com/OpenZeppelin/openzeppelin-contracts/blob/master/contracts/token/ERC20/presets/ERC20PresetMinterPauser.sol)
12. [Astar の開発環境構築でハマったことをメモしていく](https://eternalbluebullet.hatenablog.com/entry/2022/11/16/173900)
13. [Web 以外でも期待される WebAssembly - Blockchain との親和性について](https://engineering.linecorp.com/ja/blog/webassembly-expected-to-be-used-beyond-the-web/)
14. [【Opensea】metadata standards](https://docs.opensea.io/docs/metadata-standards)
15. [wizard openzeppelin](https://wizard.openzeppelin.com/#erc721)
16. [substrate install](https://docs.substrate.io/install/macos/)
17. [!ink setup](https://use.ink/getting-started/setup/)
18. [【Astar Docs】Swanky Suite](https://docs.astar.network/docs/wasm/sc-dev/swanky/)
19. [【npm】Swanky CLI](https://www.npmjs.com/package/@astar-network/swanky-cli)
20. [【Qita】WebAssembly とは](https://qiita.com/ShuntaShirai/items/3ac92412720789576f22)
21. [Introduction to WASM contract development on Astar Network](https://docs.google.com/presentation/d/16uS2rYBBZvjRwZsAcYbicKWGKJveN1qrL48amrQBB2s/edit#slide=id.g1ace6b0795c_0_47)
22. [Hands on of WASM smart contrat](https://docs.google.com/presentation/d/1tWGaZywJnIFKZoiOma8nDlpzhGtIspduobv1SbDBWCg/edit#slide=id.g19f78e2a3b2_0_75)
23. [Why WebAssembly for Smart Contracts?](https://use.ink/why-webassembly-for-smart-contracts)
24. [Swanky Docs](https://docs.astar.network/docs/wasm/sc-dev/swanky/)
25. [contracts-UI](https://contracts-ui.substrate.io/)
26. [WebAssembly-benchmark](https://github.com/takahirox/WebAssembly-benchmark)
27. [JS VS WASM Demo](https://takahirox.github.io/WebAssembly-benchmark/)
28. [MDN WebAssembly](https://developer.mozilla.org/ja/docs/WebAssembly)
29. [こわくない LLVM 入門！](https://qiita.com/Anko_9801/items/df4475fecbddd0d91ccc)
30. [wasmer](https://wasmer.io/)
31. [Wasmtime](https://wasmtime.dev/)
32. [OpenBrush](https://openbrush.io/)
33. [OpenBruch Docs](https://docs.openbrush.io/deployment)
34. [最新版の polkadot.js](https://polkadotjs-apps.web.app/#/explorer)
35. [Flipper UI Sample](https://cielo.works/flipper-ui/)
36. [【Github】wasm-showcase-dapps](https://github.com/AstarNetwork/wasm-showcase-dapps/tree/main/nft/ui/viewer)
37. [Rust の文字列操作](https://qiita.com/aflc/items/f2be832f9612064b12c6)
