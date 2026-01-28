# Anti-Virus

A PoC zero trust anti virus written in Rust.

## Creator

That1EthicalHacker

### Warning

This is currently a PoC program and should not be used on real servers due to possible design flaws and other mistakes that could make server's vulnerable to cyberattacks. If a zero trust solution is to be implemented, please use ThreatLocker's solution.

## Usage

### Version 0.2

Version 0.2 adds a GUI for the user to interact with. The main difference is that. The back-end and functionality does not change, however it does not work against user accounts currently. This will be addressed within the next update to version 0.3.

## Problems

### Version 0.1

    1) Terminal based only
    2) User input is very selective not checking input for anything but spesific inputs
    3) No black list and will prompt user for choice even if the same program or user is detected
    4) No way to update the program without changing recompiling
    5) Doesnt prompt user for Administrator account access through UAC
    6) Runs in user space with local administrator privileges
    7) Doesnt check for tampering with AV software
    8) Changing settings requires a recompile of the program

### Version 0.2

    1) Doesnt save whitelist or blacklist
    2) Doesnt save logs
    3) No way to update the program either automatically or through GUI
    4) Doesnt prompt user for Administrator account access through UAC
    5) Runs in user space with local administrator privileges
    6) Doesnt check for tampering with AV software

## Planned Updates

### Version 0.1 -> 0.2

    1) Adding a basic GUI
    2) Adding a black list
    3) Adding logging (already written)
    4) Prompt user for Administrator account access through UAC

### Version 0.2 -> 0.3

    1) Adding a basic updater and access to updater through GUI
    2) Prompt user for Aministrator account access through UAC
    3) Save blacklists and whitelists
    4) Save logs
    5) Analize users
    6) Check program hash thorugh elivated service in NT AUTHORITY \ SYSTEM
    7) Load blacklists and whitelists from disk with hash checks