use std::process::Command;

fn main() {
    Command::new("start")
    .arg("/unix")
    .arg("/bin/bash") 
    .arg("-c")
    .arg("cd \"$HOME/.local/share/Steam/steamapps/common/The Lab\" && env STEAM_COMPAT_DATA_PATH=\"$HOME/.local/share/Steam/steamapps/compatdata/450390\" STEAM_COMPAT_CLIENT_INSTALL_PATH=\"$HOME/.local/share/Steam\" SteamGameId=450390 $HOME/.local/share/Steam/compatibilitytools.d/proton_10.0-local/proton waitforexitandrun \"$HOME/.local/share/Steam/steamapps/common/The Lab/TheLab/win64/TheLab-real.exe\" -popupwindow")
    .spawn()
    .expect("Failed to launch The Lab");
}