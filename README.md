## Building the `node-exporter` binary for the Turing Pi 2 BMC

With [cross][]:

    $ cross build --target armv7-unknown-linux-gnueabi --release
    [..]
    $ ls -l target/armv7-unknown-linux-gnueabi/release/node-exporter
    -rwxr-xr-x 2 carlos users 2058000 Mar  3 18:52 target/armv7-unknown-linux-gnueabi/release/node-exporter
    $
  
[cross]: https://github.com/cross-rs/cross
