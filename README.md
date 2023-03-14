# Chatty

A TCP Chat Server impelement with Rust & Async.

```shell
$ cargo run
```

## User A
```shell
$ telnet localhost 8000
Connected to localhost.
Escape character is '^]'.
Welcome to chatty!
Login A
Welcome A
To B: Hello
Hi
bye
A, see you soon!
```

## User B
```shell
$ telnet localhost 8000
Connected to localhost.
Escape character is '^]'.
Welcome to chatty!
Login B
Welcome B
Hello
To A: Hi
bye
B, see you soon!
```
