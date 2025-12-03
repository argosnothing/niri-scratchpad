# Dynamic Niri Scratchpad

Dynamically assign windows as scratchpads against numerical register. 

The program will also manage windows that have been deleted since running the command last. If you invoke `niri-scratchpad create 1` and register 1 has been deleted since last excution, it will bind the currently focused window to that register instead as a scratchpad. 

Scratchpad memory does not persist on logging out on session.

```kdl
    workspace "stash" {
        open-on-output "DP-1" // Your output name, or just omit this property entirely
    }
```

For binding to a keybind you would do: 
```kdl
    Mod+Q            { spawn "niri-scratchpad" "create" "1"; }
```
This will take the currently focused window and bind it to niri-scratchpad register 1. Pressing this keybind again will move the scratchpad to the stash workspace. 

A separate command is available for removing a scratchpad at a particular register. 
```kdl
    Mod+Ctrl+Q            { spawn "niri-scratchpad" "delete" "1"; }
```
delete scratchpad at register 1

## Installation
It's just a rust binary, you'll need to build it with `niri_ipc` `serde` and `clap` crates. I provide a flake you can also consume as an input. 
```nix
inputs {
    niri-scratchpad.url = "github:argosnothing/niri-scratchpad";
}
```

To put it in your path on nix:
```nix

    environment.systemPackages = [
      inputs.niri-scratchpad.packages.${pkgs.system}.default
    ];
```

Additional goodies: 
Niri scratchpad currently supports per command output of scratchpad data through the --output option as well as a dedicated get action for titles. This allows you to create things like dynamic buttons that show the title of the scratchpad on them. 
https://cdn.discordapp.com/attachments/1133342558165151778/1445860834302759054/tmp.Q9FEivTv68.mp4?ex=6931e255&is=693090d5&hm=fcd525e9c568f61b6194b8d18fa93078dbb11a04a675a301296eaa8f6c182eb5&
