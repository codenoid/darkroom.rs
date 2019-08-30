# Darkroom Rust Version

This application is `still` slow af, do not use for production, inspired by [Gojek's Darkroom](https://www.gojek.io/darkroom/)


## Roadmap

- [x] Has same image transform function like Gojek's darkroom (75%)
- [ ] Implement channel for `&image` processing
- [ ] Implement storage (custom storage)
- [ ] Response as image (with image mime type)

## Example

```
git clone https://github.com/codenoid/darkroom.rs
cd darkroom.rs
cargo run // accessible via localhost:3000/?path=./images/boris-satay.jpg&mono=true&flip=h
```

`pathtoimg = ./images/boris-satay.jpg`

|  original | /?path=pathtoimg;mono=true;flip=h  |
|-----------|---------------------------------------------------|
|  ![Original](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/images/boris-satay.jpg)  | ![Result](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/flip_h_mono.jpg)  |

|  original | /?path=pathtoimg;flip=h;rotate=45  |
|-----------|---------------------------------------------------|
|  ![Original](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/images/boris-satay.jpg)  | ![Result](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/flip_h_rotate_45.jpg)  |

|  original | /?path=pathtoimg;flip=v;rotate=90  |
|-----------|---------------------------------------------------|
|  ![Original](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/images/boris-satay.jpg)  | ![Result](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/flip_v_rotate_90.jpg)  |

|  original | /?path=pathtoimg;emboss=true  |
|-----------|---------------------------------------------------|
|  ![Original](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/images/boris-satay.jpg)  | ![Result](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/emboss.jpg)  |

|  original | /?path=pathtoimg;brightness=0.3  |
|-----------|---------------------------------------------------|
|  ![Original](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/images/boris-satay.jpg)  | ![Result](https://raw.githubusercontent.com/codenoid/darkroom.rs/master/brightness_0.3.jpg)  |