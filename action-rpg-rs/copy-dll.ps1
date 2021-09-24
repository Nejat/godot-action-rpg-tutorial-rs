if ($args -eq "release") {
    cargo build --release
} elseif ($args -eq "debug") {
    cargo build
}

cp ./target/$args/action_rpg_rs.dll ../action-rpg/action-rpg.dll