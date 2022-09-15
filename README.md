# PNGme

Sneak naughty messages into your innocent cat pictures.
[tutorial](https://picklenerd.github.io/pngme_book/)

## PNG Explained

A PNG file is built from chunks. Each chunk has a 4-letter chunk type
(like `IDAT` or `tEXt`); Some types are critical to the image rendering,
while others are ignored by decoders, which means we can add a chunk with
a custom chunk type like `teSt` or `nsFw` and hide message in it.

Check out [PNG specs](https://www.w3.org/TR/2003/REC-PNG-20031110/)
for more information.

## Commands

```
pngme encode ./cat.png teSt "Hello world!"
pngme decode ./cat.png teSt
pngme remove ./cat.png teSt
pngme print ./cat.png
```

## Follow-ups

- [ ] Check the properties of input chunk type
- [ ] Split into library and binary
- [ ] Decode, remove or print multiple chunks
- [ ] Detect potential message automatically
