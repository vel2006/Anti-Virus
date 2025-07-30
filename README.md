# Anti-Virus

A PoC zero trust anti virus written in Rust.

## Creator

That1EthicalHacker

### Warning

This is currently a PoC program and should not be used on real servers due to possible design flaws and other mistakes that could make server's vulnerable to cyberattacks. If a zero trust solution is to be implemented, please use ThreatLocker's solution.

## Usage

### Version 0.1

Version 0.1 is terminal based and requires the script to be ran with admin perms (will not prompt for admin perms) to remove users. Configuration of script settings is done before building, which is to be changed in future updates. When a new program or user is detected the script will either automatically remove the content or prompt the user if they wish to allow or deny the possible threat. If the user does not type in 'allow' the program will asume that the user wishes to deny the media and will act acordingly. If the user does allow the possible threat, the program will automatically add it to the whitelist.

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

## Planned Updates

### Version 0.1 -> 0.2

    1) Adding a basic GUI
    2) Adding a black list
    3) Adding logging (already written)
    4) Prompt user for Administrator account access through UAC