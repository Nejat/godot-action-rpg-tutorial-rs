# Godot Action RPG w/Rust
A [GDNative](https://docs.godotengine.org/en/stable/tutorials/plugins/gdnative/index.html) implementation in [Rust <img src="https://www.rustacean.net/assets/rustacean-flat-happy.svg" width="40"/>](https://www.rust-lang.org/)  of youtuber [HeartBeast <img src="https://yt3.ggpht.com/ytc/AKedOLSkfuN8VUPKr8FmL_42T-u3nd4MMx6VoY16V17BxA=s176-c-k-c0x00ffffff-no-rj" width=25>](https://www.youtube.com/c/uheartbeast)'s great step-by-step turoial series creating a [Godot Action RPG](https://tinyurl.com/5t7rstyx), using the [godot-rust <img src="https://godot-rust.github.io/godot-ferris.svg" width="25"/>](https://godot-rust.github.io/) <img src="https://crates.io/assets/Cargo-Logo-Small-c39abeb466d747f3be442698662c5260.png" width=35 />

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

## Linux Build
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