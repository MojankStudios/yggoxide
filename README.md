# yggoxide

This crate implements Mojang's Yggdrasil authentication server (https://authserver.mojang.com) as well as the session server (https://sessionserver.mojang.com).

This is intended to be used as a library where the user implements the `YggoxideImpl` trait to actually give functionality to the API, take a look at `examples/mock_server.rs` to find out how to do this.

### Try it out

To run the mock server:

```git
git clone https://github.com/MojankStudios/yggoxide
cd yggoxide
cargo run --example mock_server
```

You can now see the entire exposed API at http://localhost:8000/swagger.

![Amogus](https://c.tenor.com/z561VExaPEcAAAAd/amogus.gif)
