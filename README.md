## Building the `node-exporter` binary for the Turing Pi 2 BMC

With [cross][]:

    $ cross build --target armv7-unknown-linux-gnueabi --release
    [..]
    $ ls -l target/armv7-unknown-linux-gnueabi/release/node-exporter
    -rwxr-xr-x 2 carlos users 3942240 Mar  3 17:29 target/armv7-unknown-linux-gnueabi/release/node-exporter
    $
  
[cross]: https://github.com/cross-rs/cross
