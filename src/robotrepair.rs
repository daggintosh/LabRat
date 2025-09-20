use std::process::Command;

fn main() {
    Command::new("start")
    .arg("/unix")
    .arg("/bin/bash") 
    .arg("-c")
    .arg("cd \"$HOME/.local/share/Steam/steamapps/common/The Lab\" && env STEAM_COMPAT_DATA_PATH=\"$HOME/.local/share/Steam/steamapps/compatdata/450390\" STEAM_COMPAT_CLIENT_INSTALL_PATH=\"$HOME/.local/share/Steam\" SteamGameId=450390 $HOME/.local/share/Steam/compatibilitytools.d/proton_10.0-local/proton run \"$HOME/.local/share/Steam/steamapps/common/The Lab/RobotRepair/bin/win64/vr.real.exe\" -noassert -retail -autofidelity +map vr_aperture_main +vr_multitool_wormhole 1 +closecaption 1 +cc_subtitles 1 -language english")
    .spawn()
    .expect("Failed to launch Robot Repair");
}