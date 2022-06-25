# yggoxide

This crate currently implements the REST API for:

| Service                  | Exposed at            | Minecraft Production              | Coverage      |
| ------------------------ | --------------------- | --------------------------------- | ------------- |
| Yggdrasil authentication | `/` and `/authserver` | https://authserver.mojang.com     | 100%          |
| Minecraft sessions       | `/session`            | https://sessionserver.mojang.com  | 100%          |
| Minecraft services       | `/services`           | https://api.minecraftservices.com | Only 2 routes |

It does not currently implement:

- Mojang Accounts Service

This is intended to be used as a library where the user implements the `YggoxideImpl` trait to actually give functionality to the API, take a look at `examples/mock_server.rs` to find out how to do this.

### Try it out

To run the mock server:

```bash
git clone https://github.com/MojankStudios/yggoxide
cd yggoxide
cargo run --example mock_server
```

You can now see the entire exposed API at http://localhost:8000/swagger.

### Spoof Server

You can also spin up a "spoof" server which uses Mojang's API to resolve users.

```bash
cargo run --example spoof_server
```

If a player cannot be found, a fake player is created instead.

![Amogus](https://c.tenor.com/z561VExaPEcAAAAd/amogus.gif)
