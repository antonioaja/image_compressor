# image_compressor
image_compressor is a CLI program to convert an input image to an output image of the user's choosing (i.e bmp, jpg/jpeg, png, gif). There are many options options/arguments which can be applied to alter the compression/conversion of an image.

For example:

```bash
.\image_compressor.exe --input in.png --output out.jpg
```

Here are all the current options:

![Alt text](image_compressor_options.png)

## Why?

This program was made to fit the needs of TGFS' data collection system. It was simpler to write this code, than to user a more popular software like ImageMagick. I would recommend using that software instead of this program.

## Credits

* [image-rs](https://github.com/image-rs/image)
* [anyhow](https://github.com/dtolnay/anyhow)
* [clap](https://github.com/clap-rs/clap)

## Compiling

Complied using cargo >= 1.63. This can be installed via the [rustup](https://www.rust-lang.org/tools/install) toolchain.