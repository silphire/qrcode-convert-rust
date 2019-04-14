# qrcode-convert-rust

QRcode encoder/decoder works with Rust.

This work is derived from [ZXing](https://github.com/zxing/zxing).

## Why are you implement QRcode encoder/decoder from ZXing.

When I seek a QRcode decoder written in Rust, I found the [qrcode-rust](https://github.com/kennytm/qrcode-rust). It is good library  but has only encoder. Finally, I could not find a QRcode decoder written in Rust. 

So, I need to write QRcode decoder. I found the most popular QRcode decoder library written in Javascript (based on GitHub star count),  [jsqrcode](https://github.com/LazarSoft/jsqrcode), is port of ZXing. It is good idea for me to follow the same idea what the jsqrcode have already did. So, I decided to try to port the ZXing to Rust. 
