# Minecraft Server Mappings

> Convert server addresses (`mc.hypixel.net`) to pretty, human readable names (`Hypixel`)

## Usage

Exports one function, `get_pretty_name`:

```rs
let pretty_name = get_pretty_name("mc.hypixel.net");
assert_eq!(pretty_name, Some("Hypixel"));
```
