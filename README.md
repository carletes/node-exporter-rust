## Building the `node-exporter` binary for the Turing Pi 2 BMC

With [cross][]:

    $ cross build --target armv7-unknown-linux-gnueabi --release
    [..]
    $ ls -l $ ls -l target/armv7-unknown-linux-gnueabi/release/node-exporter
    -rwxr-xr-x 2 carlos users 3884904 Jan 30 18:43 target/armv7-unknown-linux-gnueabi/release/node-exporter
    $
  
