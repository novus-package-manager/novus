# Novus
A blazingly fast and futuristic package manager for windows.

# Why Novus

### Fast

Using Novus, you can install packages using multiple threads, making it around 8 times faster than normal downloads.

### Silent Installations

Novus installs apps silently, in a manner such that users don't need to accept any prompts during the installation.

### Non-Admin Installations

Novus supports non-admin installations. You don't have to go through the effort of opening a separate admin terminal.

### Portable Installations

Novus supports the installation of portable packages.

### Open Source

Novus is open-source, making it easier to contribute and add packages to the manifest.

### Package Manifests

All of Novus's packages are updated and monitored daily, using a fail-safe auto-updating script.

# Difference from Chocolatey

### Speed

Novus is significantly faster than chocolatey due to it's multithreaded nature.

<img src="https://i.imgur.com/wSj375R.png" />

### Non-Admin Installations

Using Novus, you can install packages without having to use an admin terminal. Novus automatically elevates to an admin terminal without bypassing UAC.

### Additional Features

Novus has a lot of other useful features. For example, Novus can easily quit and forcequit applications with just one command.

# Installation

Novus is extremely easy to install.

This can be done using the official installer or via powershell.

## Powershell Installation

You can install Novus using the powershell command.

First, you must set the PowerShell Execution Policy from Restricted to RemoteSigned or Unrestricted to allow local PowerShell scripts to run.

```powershell
Set-ExecutionPolicy RemoteSigned
```

You can then run this command on your powershell window to install Novus

```powershell
iwr -useb https://storage.googleapis.com/novus_bucket/api/powershell_install.ps1 | iex
```

## Official Installer

The official installer for Novus can be found [here](https://github.com/novus-package-manager/novus/releases/latest).

Check out the [Github Page](https://github.com/novus-package-manager/novus/releases)

After downloading the installer, follow the simple installation prompts.

# Official Site

Visit our [website](https://www.novuspkg.com/) for more information about Novus.

Take a look at the [docs](https://docs.novuspkg.com/docs/getting-started/installation) on how to get started.

# Supporters
[![Stargazers repo roster for @novus-package-manager/novus](https://reporoster.com/stars/novus-package-manager/novus)](https://github.com/novus-package-manager/novus/stargazers)

# Authors
[ZaphodElevated](https://www.github.com/ZaphodElevated) - Founder And Developer Of Novus

Find other contributors [here](https://github.com/novus-package-manager/novus/graphs/contributors)

# Credits
[Credits](https://github.com/novus-package-manager/novus/blob/main/CREDITS.md)
