# phubble
Persistent homology mapping telescope

## How to build

1. You will need a [rust toolchain](https://doc.rust-lang.org/cargo/getting-started/installation.html)
2. Ideally, switch into a python virtual environment.
3. Run `pip install maturin[zig]`
4. Located in the `phubble` directory run
```
maturin dev --release
```
5. Within your virtual environment, you should now be able to
```
from phubble import build_telescope
```
