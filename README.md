# Image-Processing-CLI-in-Rust [![Build Status](https://travis-ci.org/spejss/Image-Processing-CLI-in-Rust.svg?branch=master)](https://travis-ci.org/spejss/Image-Processing-CLI-in-Rust)

CLI for processing images in Rust. Some implementation is custom and for some functionality it uses 3rd party libraries.

This project uses following libraries:

https://github.com/PistonDevelopers/image

Processed images are being stored in the same folder as the source image using the name of the source image with an appended suffix.

Source: hotelroom.jpg

Copy: hotelroomCopy.jpg

Blur: hotelroomBlur.jpg

Generated histograms are also stored this way.

## Implemented functions

* ***Binary threshold*** -o binaryThreshold -v 200
* ***(NAIVE) Histogram for colors (RGB)*** -o histogram
* ***(NAIVE) Histogram for grayscale images*** -o histogramGrayscale
* ***Average color*** -o average
* ***Copy:*** -o copy
* ***Thumbnail:*** -o thumbnail -v 96
* ***blur:*** -o blur -v 4.0
* ***brighten:*** -o brighten -v 10
* ***huerotate:*** -o huerotate -v 10
* ***contrast:*** -o contrast -v 20.0
* ***grayscale***: -o grayscale
* ***invert*** -o invert

## Examples

### Copy image

```bash
ipcli -o copy --image "D:\Image\hotelroom.jpg"
```

### Create a thumbnail

```bash
ipcli -o thumbnail -v 96 --image "D:\Image\hotelroom.jpg"
```

### Generate a histogram of colors

```bash
ipcli -o histogram -i "D:\Image\hotelroom.jpg"
```

![Example histogram of colors](assets/histogram.png)

```text
IPCLI  0.1
Mikolaj Wawrzyniak <mikolaj.wawrzyniak at fh-dortmund.de>
Basic CLI for image processing

USAGE:
    ipcli [OPTIONS] --image <FILE> --operation <operation>

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -i, --image <FILE>             Opens specified image file and uses it for transformations.
    -o, --operation <operation>    Specifies operation to be done on the image
    -v, --value <value>            Value for the transformation. To see what values are needed, check the
                                   documentation.
```
