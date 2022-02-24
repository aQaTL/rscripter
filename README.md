# rscripter

# Recommended quick setup

I recommend leveraging [cargo-generate](https://github.com/cargo-generate/cargo-generate).

1. If you don't have cargo-generate already, install it:

```shell
cargo install cargo-generate
```

2. Generate your repo using this one as a template:

```shell
cargo generate -n my-rust-scripts aQaTL/rscripter 
```

Take a look at examples in the `examples` directory. 

3. Let's say you want to create a script that pings `1.1.1.1` 3 times.

- Start by creating a file in `src/bin/`

```rust
/// src/bin/ping_cldflr.rs

use rscripter::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	cmd!("ping", "-c", "3", "1.1.1.1")?;
	
	Ok(())
}
```

- Test it

```shell
$ cargo run --bin ping_cldflr
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/ping_cldflr`
ping -c 3 1.1.1.1
PING 1.1.1.1 (1.1.1.1) 56(84) bytes of data.
64 bytes from 1.1.1.1: icmp_seq=1 ttl=56 time=51.9 ms
64 bytes from 1.1.1.1: icmp_seq=2 ttl=56 time=51.7 ms
64 bytes from 1.1.1.1: icmp_seq=3 ttl=56 time=43.6 ms

--- 1.1.1.1 ping statistics ---
3 packets transmitted, 3 received, 0% packet loss, time 2003ms
rtt min/avg/max/mdev = 43.574/49.041/51.858/3.866 ms
```

- You can install all scripts from the `bin` directory 

```shell
cargo install --path . -f
```

- Now you can use the scripts without cargo anyhwere

```shell
me@pc:~$ ping_cldflr
ping -c 3 1.1.1.1
PING 1.1.1.1 (1.1.1.1) 56(84) bytes of data.
64 bytes from 1.1.1.1: icmp_seq=1 ttl=56 time=51.9 ms
64 bytes from 1.1.1.1: icmp_seq=2 ttl=56 time=51.7 ms
64 bytes from 1.1.1.1: icmp_seq=3 ttl=56 time=43.6 ms
me@pc:~$ 
```