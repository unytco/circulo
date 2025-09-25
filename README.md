<div align="center">

# <img src="src-tauri/icons/circulo-logo.svg" alt="Circulo" width="300">

![Latest Release](https://img.shields.io/github/v/release/unytco/circulo?style=flat-square&color=blue)
![Downloads](https://img.shields.io/github/downloads/unytco/circulo/total?style=flat-square&color=green)
![License](https://img.shields.io/github/license/unytco/circulo?style=flat-square)

### A Playful Take on p2p Payments

<div align="left">
<h3>

</h3>
<h3> 
Introduction
</h3>
<p>
Circulo is a generosity oriented peer-to-peer payments system, built using the Unyt mutual credit accounting engine. 
</p>
<p>
Everyone starts with an account balance of zero and a small credit limit. Your credit limit determines the max you can spend below zero. 
</p>
<p>
Circulo makes use of a very simple credit algorithm: every time you send someone some units, your credit limit increases by 10.  This is a very simple, and easily gameable credit algorithm. It also makes it easy for people to play with it.
</p>

<p>
In Circulo, as soon as you send someone units, your account will be debited.  However, the receiver's account balance won't include those units until they exercize their own agency and choose to accept them. If they never accept them, they remain available for them to collect. No one else is able to claim them.
</p>
<p>
By anchoring activity in the agency of the participants themselves, Circulo opens up some interesting new efficiencies and possibilities. These will get explored in more detail in future releases.
</p>
<h3>
Testing
</h3> 
<p>
Circulo is a toddler as far as software goes. But feel free to try and trip it up. 
</p>
<p>
If you see an error, please <a href="https://github.com/unytco/circulo/issues">report it as an issue</a> in this repo (if not already reported). 
</p>
<p>
To get past an error:

1. Reload: When in doubt try: Right click > Reload
2. Restart: When a few reload tries don't work, please quit and restart
3. Wait: making yourself a cup of tea resolves most things

</p>
<p>
We are using software called Sentry to help track errors so we can better understand their frequency and context. But you reporting issues is helpful.
</p>

<p>

</p>
</div>

</div>

## Downloads Zero Arc releases

<div align="center">

<table>
<tr>
<td width="25%" align="center">

### **Windows**

---

[MSI Installer (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_x64_windows.msi)

[EXE Setup (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_x64_windows.exe)

</td>
<td width="25%" align="center">

### **MacOS**

---

[Silicon (arm64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_aarch64_darwin)

[Intel (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_x64_darwin.dmg)

</td>
<td width="25%" align="center">

### **Linux**

---

[AppImage](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_amd64_linux.AppImage)

[Debian (.deb)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_zero-arc_0.0.2_amd64_linux.deb)

</td>
<!-- <td width="25%" align="center">

**Android**

[<img src="https://img.shields.io/badge/-Download-green?style=flat-square&logoColor=white" height="35">](https://github.com/unytco/circulo/releases/download/v0.0.1/app-universal-release.apk)

Android Universal APK • [AAB Bundle](https://github.com/unytco/circulo/releases/download/v0.0.1/app-universal-release.aab)

</td> -->
</tr>
</table>

</div>

## Download Full Arc releases

<div align="center">

<table>
<tr>
<td width="25%" align="center">

### **Windows**

---

[MSI Installer (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_x64_windows.msi)

[EXE Setup (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_x64_windows.exe)

</td>
<td width="25%" align="center">

### **MacOS**

---

[Silicon (arm64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_aarch64_darwin)

[Intel (x64)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_x64_darwin.dmg)

</td>
<td width="25%" align="center">

### **Linux**

---

[AppImage](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_amd64_linux.AppImage)

[Debian (.deb)](https://github.com/unytco/circulo/releases/download/v0.0.2/Circulo_0.0.2_amd64_linux.deb)

</tr>
</table>

</div>

> **Note:** Download links point to the v0.0.1 release. For the latest version, visit the [releases page](https://github.com/unytco/circulo/releases).

## Installation

<details>
<summary><strong>Windows</strong></summary>

1. Download the `.msi` installer
2. Run the installer and follow the setup wizard
3. Launch Circulo from the Start menu

</details>

<details>
<summary><strong>macOS</strong></summary>

1. Download the `.dmg` file
2. Open the DMG and drag Circulo to your Applications folder
3. Launch from Applications (you may need to allow the app in System Preferences > Security)

</details>

<details>
<summary><strong>Linux</strong></summary>

**AppImage (Recommended)**

1. Download the `.AppImage` file
2. Make it executable: `chmod +x circulo_0.1.0_amd64.AppImage`
3. Run: `./circulo_0.1.0_amd64.AppImage`

**Debian/Ubuntu**

1. Download the `.deb` package
2. Install: `sudo dpkg -i circulo_0.1.0_amd64.deb`
3. Run: `circulo`

</details>

<!-- <details>
<summary><strong>Android</strong></summary>

1. Download the appropriate APK for your device architecture
2. Enable "Install from unknown sources" in your device settings
3. Install the APK file
4. Launch Circulo from your app drawer

</details> -->

## System Requirements

| Platform    | Minimum Requirements                    |
| ----------- | --------------------------------------- |
| **Windows** | Windows 10 (64-bit) or later            |
| **macOS**   | macOS 10.15 (Catalina) or later         |
| **Linux**   | Ubuntu 18.04+ / equivalent distribution |

 <!--        | **Android**                             | Android 7.0 (API level 24) or later | -->

**Recommended:** 4GB RAM, 1GB free disk space, internet connection for updates

## Support

- [Report Issues](https://github.com/unytco/circulo/issues)
- [Discussions](https://link-to-telegram)

## License

This project is licensed under the terms specified in the [LICENSE](LICENSE) file.
