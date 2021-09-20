# static-server
A simple Rust HTTP static server :crab:

It is a toy project to learn Rust and HTTP protocol.

## How its works
This will server HTML files based on command path :open_file_folder:, example:

```shell
 ./
├──  index.html
├──  page2/
│  └──  index.html
└──  page3.html
```
Then run static-server on this folder
```
static-server 
```
Then the server start to respond on `http://localhost:8080` following for each path a specific file

- Acessing `http://localhost:8080` wil render `index.html` and
- `http://localhost:8080/page2` will render `page2/index.html`
- `http://localhost:8080/page3` will render `page3.html`

## How to install
Download the binary on [releases page](https://github.com/GuilhermeVBeira/static-server/releases) or build by yourself
```
cargo build --release
```
And copy you binary to user folder
```
cp target/release/static-server /usr/local/bin
```

## Caveats
its serve only HTML files

## How to test?
I didn't learn this part yet :man_shrugging:

