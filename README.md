![logo](assets/icon.png)

# Vdo**De**Cipher

 A simple Rust project that runs whatever website you want to access that has videos locked behind the [VdoCipher](https://www.vdocipher.com) DRM.

*This is just a simple fork of the amazing [`prime-wine`](https://github.com/NelloKudo/prime-wine) project, they do the hard work there.*

## Changing the app URL path

By default it opens [DuckDuckGo](https://duckduckgo.com).

To change the path just pass the `--manage` arg to open the settings GUI.

Or if you have it installed through an AppImage manager like [Gear Lever](https://github.com/mijorus/gearlever), just right click the icon and choose the `Manage VdoDeCipher settings` option. 

## Building from source

Everything is just plain Rust plus a bash script for packaging:

```
git clone https://github.com/AMA147000/VdoDeCipher.git
cd VdoDeCipher
./build-appimage.sh
```

## (-_-)

*If that project somehow gets seen by someone working for VdoCipher:* **Sincerely, go fuck yourself for making our lives difficult (especially that your DRM is used mainly for educational courses).**
