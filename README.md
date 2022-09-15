# PNGme

Sneak naughty messages into your innocent cat pictures.
[tutorial](https://picklenerd.github.io/pngme_book/)

## PNG Explained

TODO: How chunks and chunk types work?

Checkout [PNG specs](https://www.w3.org/TR/2003/REC-PNG-20031110/)
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
