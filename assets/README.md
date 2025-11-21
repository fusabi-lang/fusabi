# Fusabi Brand Assets

This directory contains official Fusabi brand assets for use in documentation, applications, and marketing materials.

## Files

### logo.svg
- **Format**: SVG (vector)
- **Dimensions**: 100x100 viewBox
- **Colors**: Wasabi green gradient (#99CC33 to #78A659)
- **Usage**: General purpose logo, README headers, documentation
- **Style**: Abstract "F" merging with organic leaf shape

### icon.ico
- **Format**: ICO (multi-resolution)
- **Sizes**: 16x16, 32x32, 48x48, 256x256
- **Usage**: Windows executable icon, favicon
- **Generated from**: logo.svg

### social_preview.png
- **Format**: PNG
- **Dimensions**: 1280x640px
- **Usage**: GitHub social media preview, Open Graph image
- **Content**: Logo + tagline + color palette
- **Background**: Dark (#1E1E1E)

## Usage Guidelines

### Do's
- Use the SVG logo for web and documentation
- Maintain aspect ratio when resizing
- Use on backgrounds that provide sufficient contrast
- Center-align in most contexts

### Don'ts
- Don't alter the logo colors
- Don't add effects or filters
- Don't rotate or skew the logo
- Don't use low-resolution raster versions when SVG is available

## Color Reference

- **Wasabi Electric**: #99CC33 (RGB: 153, 204, 51)
- **Wasabi Natural**: #78A659 (RGB: 120, 166, 89)
- **Dark Background**: #1E1E1E (RGB: 30, 30, 30)
- **Light Text**: #F0F0F0 (RGB: 240, 240, 240)

See `docs/BRANDING.md` for complete brand guidelines.

## Generating ICO from SVG

If you need to regenerate `icon.ico`:

```bash
# Using ImageMagick
magick convert -background none -define icon:auto-resize=256,48,32,16 assets/logo.svg assets/icon.ico

# Or using an online converter
# Upload logo.svg to: https://convertio.co/svg-ico/
```

## Generating Social Preview

The social preview should be generated with:
- Logo centered
- Tagline: "Small. Potent. Functional."
- Color palette swatches
- Dark background (#1E1E1E)

See `scripts/generate_social_preview.sh` (if available) or create manually with design tools.

## License

These assets are part of the Fusabi project and licensed under MIT License.

For questions about asset usage, open an issue: https://github.com/fusabi-lang/fusabi/issues
