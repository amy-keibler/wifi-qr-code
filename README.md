# wifi-qr-code

[![crates.io](https://img.shields.io/crates/v/wifi-qr-code.svg)](https://crates.io/crates/wifi-qr-code)
[![docs.rs](https://docs.rs/wifi-qr-code/badge.svg)](https://docs.rs/wifi-qr-code/)
![Test status badge](https://github.com/amy-keibler/wifi-qr-code/workflows/Test/badge.svg)

Wifi QR codes are a way to encode wifi connection information and credentials into a QR code so that it can be scanned. They are supported via the latest Android and iOS phones, as well as other platforms. Unfortunately, there does not appear to be a standardized format for the data, so this implementation consults existing implementations (linked in the [References](#references) section). If a standard becomes available, please let me know.

It is important to take into account that QR codes do not provide any security mechanisms that would prevent someone from just reading the code and recovering the password for the network. Android requires that you re-authenticate before it will display the QR code on the screen to make sure the user is allowed to share that information, for example. My particular use-case is that I want a way to share my guest wifi network information in a convenient manner with people visiting my place, so people with access to the code have already been trusted with access to the space.

## Usage

This library wraps the [qrcode-generator](https://crates.io/crates/qrcode-generator) library's basic functions that allow a user to output the QR code as a matrix, as raw image data, as a PNG image, and as an SVG image. Additionally, the direct string representation is available if a different QR code library is desired. The documentation and the `examples` folder have code that demonstrate usage.

## License

Like most Rust projects, this is dual-licensed under the [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE) licenses.

## Contributing

Contributions are welcome! This project follows the same [Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct) that the Rust project uses.

## References

* [Wikipedia Description](https://en.wikipedia.org/wiki/QR_code#WiFi_network_login)
* [JavaScript Implementation](https://github.com/evgeni/qifi)
* [Android Barcode Scanner with Wifi QR Functionality](https://github.com/zxing/zxing)
  * [Encoder](https://github.com/zxing/zxing/blob/0cf3b9be71680f50c90a71ca26ce0d33664b0dd6/zxing.appspot.com/src/main/java/com/google/zxing/web/generator/client/WifiGenerator.java)
  * [Decoder](https://github.com/zxing/zxing/blob/0cf3b9be71680f50c90a71ca26ce0d33664b0dd6/core/src/main/java/com/google/zxing/client/result/WifiResultParser.java)
