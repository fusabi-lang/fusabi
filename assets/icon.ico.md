# icon.ico Placeholder

Binary ICO file should be generated from logo.svg using ImageMagick:

```bash
magick convert -background none -define icon:auto-resize=256,48,32,16 assets/logo.svg assets/icon.ico
```

Or use an online converter: https://convertio.co/svg-ico/

**Required Sizes**: 16x16, 32x32, 48x48, 256x256
**Format**: Multi-resolution ICO
**Usage**: Windows executable icon
