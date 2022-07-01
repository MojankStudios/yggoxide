# yggoxide

This crate currently implements the REST API for:

| Service                  | Exposed at                   | Minecraft Production              | Coverage      |
| ------------------------ | ---------------------------- | --------------------------------- | ------------- |
| Yggdrasil authentication | `/` and `/authserver`        | https://authserver.mojang.com     | 100%          |
| Minecraft sessions       | `/` and `/sessionserver`     | https://sessionserver.mojang.com  | 100%          |
| Minecraft services       | `/` and `/minecraftservices` | https://api.minecraftservices.com | Only 2 routes |

It does not currently implement:

- Mojang Accounts Service

This is intended to be used as a library where the user implements the `YggoxideImpl` trait to actually give functionality to the API, take a look at `examples/mock_server.rs` to find out how to do this.

### Try it out

To run the spoof server:

```bash
git clone https://github.com/MojankStudios/yggoxide
cd yggoxide
cargo run --example spoof_server
```

You can now see the entire exposed API at http://localhost:8000/swagger.

If you point your client's authentication at the server above then:

- You can log in with any username and password.
- Usernames are mapped to real Minecraft players.
- If a Minecraft player does not exist, it will create a fake player.

### Implementations

You can find implementations of yggoxide in the `examples` folder:

| Implementation | File                                         | Description                                                                                        |
| -------------- | -------------------------------------------- | -------------------------------------------------------------------------------------------------- |
| Spoof Server   | [spoof_server.rs](/examples/spoof_server.rs) | Implements a basic cache and maps players to real Minecraft players, it also handles fake players. |
| Mock Server    | [mock_server.rs](/examples/mock_server.rs)   | Returns sample data on all routes.                                                                 |
| Todo Server    | [todo_server.rs](/examples/todo_server.rs)   | Traits are implemented using `todo!()`.                                                            |

![Amogus](https://c.tenor.com/z561VExaPEcAAAAd/amogus.gif)
