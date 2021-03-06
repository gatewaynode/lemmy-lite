# lemmy-lite
A static, nojs, touch-friendly Lemmy frontend built for legacy web clients and maximum performance

This project is not intended for official use, but rather as a proof-of-concept for pre-rendering Lemmy. Eventually it will transition into a microservice that is ran alongside Lemmy, for example, under a *lite.lemmy.com* sub domain. Ideally it will run on the same machine removing any extra latency in API calls.

### Built With

- [Rust](https://www.rust-lang.org)
- [Actix](https://actix.rs) - [Benchmarks](https://www.techempower.com/benchmarks/#test=composite)
- [Maud](https://maud.lambda.xyz) - [Benchmarks](https://ironoxidizer.github.io/ironoxidizer/blog/20200623-fastest-templating-engine)

## Features

- Open source, [AGPL License](/LICENSE).
- Cross-instance support, get a lite version of any Lemmy instance.
- NoJS using pre-rendered HTML and CSS only.
- Touch-friendly.
- Internet Exporer and NetSurf compatible.
- High performance.
  - Written in rust.
  - Static, only one<sup>1</sup> tiny request per page.
  - Supports arm64 / Raspberry Pi.
  
## Installation

Symlink won't always work since nginx user (root) requires ownership of linked file
```
gzip -kr9 static
cp -rf static /etc/nginx/lemmy-lite
ln -sf lemmy-lite.conf /etc/nginx/sites-enabled/
systemctl start nginx && cargo +nightly run --release
```

## Pictures

lemmy-lite: FireFox
![lemmy-lite: FireFox](https://user-images.githubusercontent.com/60191958/84398555-1872a280-abce-11ea-8e87-a06b3165a77e.png)

lemmy-lite: Mobile
![lemmy-lite: Mobile](https://user-images.githubusercontent.com/60191958/84398664-39d38e80-abce-11ea-862d-d2d5cb98a89b.png)


## Footnotes

1. First load fetches a stylesheet, favicon, and svgs. These are cached so all subsequent pages require only a single HTML request.
2. I use CSS Tables because it's [faster](https://benfrain.com/css-performance-test-flexbox-v-css-table-fight) and simpler than FlexBox, and because Grid is broken on IE11 and NetSurf. Using CSS tables over HTML tables avoids excess DOM objects.
3. Each page refresh is limited to API critical chain of 1 to limit the impact on instances and to keep page times fast.
4. 1.0.0 is set for when account functionality is stabilized.
5. Implement paging, page size, list sorting
6. Implement post search
7. Use collapsable elements without JS: https://jsfiddle.net/gSPqX/1/
8. Use 1 letter HTML class names.
9. Use 1 letter static file names except for favicon and index.
10. Consider not supporting UTF-8 and only using ASCII charcters for better legacy font support.
