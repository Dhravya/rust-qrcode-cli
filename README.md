<h1 align="center">Welcome to rust-qrcode-cli 👋</h1>
<p>
  <a href="#" target="_blank">
    <img alt="License: MIT" src="https://img.shields.io/badge/License-MIT-yellow.svg" />
  </a>
  <a href="https://twitter.com/dhravyashah" target="_blank">
    <img alt="Twitter: dhravyashah" src="https://img.shields.io/twitter/follow/dhravyashah.svg?style=social" />
  </a>
</p>

> A CLI I made while practicing rust to easily make QR codes with just one command, all in your terminal.

![Gif](https://us-east-1.tixte.net/uploads/img.dhravya.dev/d39.gif)

## Install

```sh
git clone https://github.com/dhravya/rust-qrcode-cli && cd rust-qrcode-cli && cargo run
```

## Usage

```sh
qrcode-cli.exe --help
```

## Parameters

`data` (`d`) * : String to be encoded in the QR code.
`output` (`o`) : Path to the output file - Defaults to `qrcode.png`.

A QR code will be generated with the given data and saved to the given output file.
![demo](./qrcode.png)

## Author

👤 **Dhravya Shah**

* Website: https://dhravya.dev
* Twitter: [@dhravyashah](https://twitter.com/dhravyashah)
* Github: [@dhravya](https://github.com/dhravya)

## Show your support

Give a ⭐️ if this project helped you!

***
_This README was generated with ❤️ by [readme-md-generator](https://github.com/kefranabg/readme-md-generator)_