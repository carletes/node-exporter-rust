## Building the `node-exporter` binary for the Turing Pi 2 BMC

With [cross][]:

    $ cross build --target armv7-unknown-linux-gnueabi --release
    [..]
    $ ls -l target/armv7-unknown-linux-gnueabi/release/node-exporter
    -rwxr-xr-x 2 carlos users 2149008 Mar  9 19:56 target/armv7-unknown-linux-gnueabi/release/node-exporter
    $
  
[cross]: https://github.com/cross-rs/cross
