# Minecraft Server Mappings

> Convert server addresses (`mc.hypixel.net`) to pretty, human readable names (`Hypixel`)

[![Tests](https://github.com/craftserverbot/server-mappings/actions/workflows/tests.yml/badge.svg)](https://github.com/craftserverbot/server-mappings/actions/workflows/tests.yml)

## Usage

Exports one function, `get_pretty_name`:

```rs
let pretty_name = get_pretty_name("mc.hypixel.net");
assert_eq!(pretty_name, Some("Hypixel"));
```
