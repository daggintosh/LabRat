# LabRat
Set of applications to bridge the gap missing between Proton's vrclient and vrserver for switching between two different vr applications.

Requires custom proton to be built with IVRApplications_LaunchInternalProcess set for win to unix path conversion in vrclient_x64's gen_wrapper.py

# How it works
1. The Lab requests the OpenVR API to launch Robot Repair or Secret Shop, then closes itself.
2. SteamVR attempts to launch the application with the system's file type handler.
3. The replacement exe for Robot Repair (vr-rr.exe) is launched instead in the system wine installation.
4. vr-rr uses the bare minimum set of environment variables and a command to launch the real Robot Repair in VR.

# Prerequisites
Rust, mingw-w64-gcc, and a system installation of Wine

# Instructions
1. Build and install Proton from https://github.com/daggintosh/Proton
2. Build this repository `cargo build --target x86_64-pc-windows-gnu --release`
4. Under `TheLab/win64`, rename `TheLab.exe` and `TheLab_Data` to `TheLab-real.exe` and `TheLab-real_Data`, respectively.
5. Copy `TheLab.exe` from `target/x86_64-pc-windows-gnu/release` into here.
6. Under `RobotRepair/bin/win64`, rename `vr.exe` to `vr.real.exe`
7. Copy `vr-rr.exe` to `RobotRepair/bin/win64` and rename it to `vr.exe`.

# Known Issues
Overlays aren't reset after transitions.