# Godot Action RPG w/Rust
A [GDNative](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/index.html) implementation in [Rust <img src="https://www.rustacean.net/assets/rustacean-flat-happy.svg" width="40"/>](https://www.rust-lang.org/)  of you-tuber [HeartBeast <img src="https://yt3.ggpht.com/ytc/AKedOLSkfuN8VUPKr8FmL_42T-u3nd4MMx6VoY16V17BxA=s176-c-k-c0x00ffffff-no-rj" width=25>](https://www.youtube.com/c/uheartbeast)'s great step-by-step tutorial series creating a [Godot Action RPG](https://tinyurl.com/5t7rstyx), using the [godot-rust <img src="https://godot-rust.github.io/godot-ferris.svg" width="25"/>](https://godot-rust.github.io/) <img src="https://crates.io/assets/Cargo-Logo-Small-c39abeb466d747f3be442698662c5260.png" width=35 />

## Windows Build
from within the ```action-rpg-rs``` folder

_build release and copy dll to godot project_

```shell
~\...\action-rpg-rs > .\build.ps1 release
```

_build debug and copy dll to godot project_

```shell
~\...\action-rpg-rs > .\build.ps1 debug
```

## MacOS / Linux Build
from within the ```action-rpg-rs``` folder

_build release and copy library so to godot project_

```shell
~/.../action-rpg-rs> ./build.sh release
```

_build debug and copy library so to godot project_

```shell
~/.../action-rpg-rs> ./build.sh debug
```

_* will require ```llvm``` tools, see_ [godot-rust](https://godot-rust.github.io/book/getting-started/setup.html) setup instructions

_* this project only works with [Godot 3.5.1](https://godotengine.org/download/archive/#3.5-beta1) and uses the last version of [GDNative 0.11.3](https://crates.io/crates/gdnative/0.11.3)_

## \* LLVM Dependencies

_This project breaks with LLVM 22+, you will need to setup at most LLVM 21 (20 also worked on a Mac)._
