# Vdo**De**Cipher

![logo](assets/icon.png)

 A simple Rust project that runs whatever website you want to access that has videos locked behind the [VdoCipher](https://www.vdocipher.com) DRM.

*This is just a simple fork of the amazing [`prime-wine`](https://github.com/NelloKudo/prime-wine) project, they do the hard work there.*

## Changing the app URL path

Currently it is set to [DuckDuckGo](https://duckduckgo.com) that way you aren't restricted to a single site.

To change the path go to [`src/launcher.rs`](src/launcher.rs) and modify the URL after `--app=` in the `BRAVE_ARGS` array.

*Note: this should be a temporary fix as I am going to implement a way to set the destination at runtime.*

## Building from source

Everything is just plain Rust plus a bash script for packaging:

```
git clone https://github.com/AMA147000/vdodecipher.git
cd vdodecipher
./build-appimage.sh
```

## (-_-)

*If that project somehow gets seen by someone working for VdoCipher:* **Sincerely, go fuck yourself for making our lives difficult (especially that your DRM is used mainly for educational courses).**
