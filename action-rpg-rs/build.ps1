param(
    [Parameter(Mandatory=$true, Position=0)][ValidateSet('release', 'debug')]
    [string] $build
)

function exit_on_fail() {
    if ($lastExitCode -ne 0) {
        exit 1
    }
}

Write-Output ""

if ($build -eq "release") {
    Write-Output "building release ..."
    Write-Output ""
    cargo build --release
    exit_on_fail
} elseif ($build -eq "debug") {
    Write-Output "building debug ..."
    Write-Output ""
    cargo build
    exit_on_fail
}

Write-Output ""
Write-Output "copying $build action_rpg_rs.dll to ../action-rpg/action_rpg_rs.dll"
Write-Output ""

Copy-Item .\target\$build\action_rpg_rs.dll ..\action-rpg\action-rpg.dll
exit_on_fail

Write-Output "done"
Write-Output ""
