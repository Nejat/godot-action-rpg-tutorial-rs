if ($args -eq "release") {
    cargo build --release
} else {
    cargo build
}

cp ./target/$args/action_rpg_rs.dll ../action-rpg/action-rpg.dll