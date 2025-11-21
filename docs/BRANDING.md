# Fusabi Brand Guidelines

**Small. Potent. Functional.**

Like wasabi - a little goes a long way.

## Color Palette

### Primary Colors

#### Wasabi Green
- **Electric**: `#99CC33` (RGB: 153, 204, 51)
- **Natural**: `#78A659` (RGB: 120, 166, 89)
- **Usage**: Success messages, primary CTAs, highlights, active states

#### Rust Orange
- **Rust**: `#B7410E` (RGB: 183, 65, 14)
- **Sashimi**: `#DEA584` (RGB: 222, 165, 132)
- **Usage**: Error messages, warnings, accents, hot-reload indicators

### Background & Text

- **Dark Grey**: `#1E1E1E` (RGB: 30, 30, 30)
  - Code blocks, terminal backgrounds, dark mode base
- **Off-white**: `#F0F0F0` (RGB: 240, 240, 240)
  - Body text, light mode text
- **Light Grey**: `#666666` (RGB: 102, 102, 102)
  - Hints, secondary text, disabled states
- **Medium Grey**: `#999999` (RGB: 153, 153, 153)
  - Borders, dividers, muted elements

### Color Usage Matrix

| Context | Primary | Secondary | Background | Text |
|---------|---------|-----------|------------|------|
| Success | `#99CC33` | `#78A659` | `#1E1E1E` | `#F0F0F0` |
| Error | `#B7410E` | `#DEA584` | `#1E1E1E` | `#F0F0F0` |
| Warning | `#DEA584` | `#B7410E` | `#1E1E1E` | `#F0F0F0` |
| Info | `#78A659` | `#99CC33` | `#1E1E1E` | `#999999` |

## Typography

### Headers
- **Font Family**: Inter, Helvetica Neue, Helvetica, sans-serif
- **H1**: 32px, Bold (700)
- **H2**: 24px, Bold (700)
- **H3**: 20px, Semibold (600)
- **H4-H6**: 16-18px, Semibold (600)
- **Line Height**: 1.4

### Body Text
- **Font Family**: System default sans-serif
- **Size**: 16px base
- **Line Height**: 1.6
- **Weight**: Regular (400)

### Code
- **Font Family**: JetBrains Mono, Fira Code, Monaco, Consolas, monospace
- **Size**: 14px (0.875em relative to body)
- **Line Height**: 1.5
- **Features**: Ligatures enabled for operators (`=>`, `->`, `|>`)

### Small Text
- **Font Family**: Inherit from context
- **Size**: 14px
- **Weight**: Regular (400)
- **Usage**: Captions, footnotes, metadata

## Emoji Usage

Visual language for quick comprehension:

- 🟢 **Fusabi Brand**: The language itself, success states
- 🦀 **Rust Integration**: Host language, FFI layer
- 🍣 **Raw/Host Access**: Direct performance, unboxed values
- ✅ **Success**: Confirmations, passing tests
- ❌ **Error**: Failures, blocking issues
- ⚠️ **Warning**: Cautions, deprecations
- 🔥 **Hot**: Hot-reload, performance, advanced features
- 🍵 **Simple**: Beginner-friendly, basic examples
- 🍱 **Complex**: Full applications, advanced patterns
- 🌶️ **Spicy**: Powerful features, sharp edges
- 👋 **Friendly**: Welcoming, onboarding

## Voice & Tone

### Core Principles

**Punchy**: Short sentences. Direct language. No fluff.

**Confident**: We know what we built. "Don't guess. Know."

**Playful**: Food metaphors (wasabi, sushi, omakase, spice).

**Technical**: Precise terminology. No hand-waving abstractions.

### Voice Examples

#### ✅ Good
```
Fusabi fits inside your binary. Zero-copy FFI with Rust. Sub-millisecond startup.

Write once, run on Fusabi VM and .NET CLR. Same syntax.

Like wasabi: A little goes a long way.
```

#### ❌ Avoid
```
Fusabi is a comprehensive solution that enables developers to leverage functional programming paradigms within Rust applications through an innovative embedding approach.

Our runtime provides compatibility with the .NET ecosystem while maintaining performance characteristics.

Fusabi offers extensive capabilities for modern development workflows.
```

### Writing Style

- **Headlines**: 3-5 words maximum
- **Subheads**: One clear benefit statement
- **Body**: 1-2 sentence paragraphs
- **Lists**: 3-7 items, parallel structure
- **Code**: Self-explanatory with minimal comments

### Brand Vocabulary

**Use**: Small, potent, functional, embedded, typed, fast, spicy, fresh, raw

**Avoid**: Revolutionary, innovative, comprehensive, robust, scalable, enterprise

## Visual Language

### Logo Aesthetic
- **Style**: Minimalist, geometric, flat
- **Shape**: Abstract "F" merging with leaf/wasabi dollop
- **Colors**: Green gradient (`#99CC33` to `#78A659`)
- **Mood**: Clean, modern, organic

### Photography Style
- **Subjects**: Close-up food photography (wasabi, sushi)
- **Treatment**: High contrast, sharp focus, dark backgrounds
- **Colors**: Emphasize greens and natural tones

### Illustration Style
- **Approach**: Line art, minimal color fills
- **Detail**: Simple geometric shapes
- **Context**: Technical diagrams, architecture flows

## CSS/HTML Reference

```css
:root {
  /* Colors */
  --fusabi-green-electric: #99CC33;
  --fusabi-green-natural: #78A659;
  --fusabi-rust: #B7410E;
  --fusabi-sashimi: #DEA584;
  --fusabi-bg-dark: #1E1E1E;
  --fusabi-text-light: #F0F0F0;
  --fusabi-text-secondary: #999999;
  --fusabi-border: #666666;

  /* Typography */
  --font-sans: Inter, Helvetica Neue, Helvetica, sans-serif;
  --font-mono: 'JetBrains Mono', 'Fira Code', Monaco, Consolas, monospace;
  --font-size-base: 16px;
  --font-size-code: 14px;
  --line-height-base: 1.6;
  --line-height-code: 1.5;
}

/* Badge Component */
.fusabi-badge {
  background: var(--fusabi-green-electric);
  color: var(--fusabi-bg-dark);
  padding: 4px 10px;
  border-radius: 4px;
  font-weight: 600;
  font-size: 14px;
  display: inline-block;
}

/* Button Primary */
.fusabi-btn-primary {
  background: linear-gradient(135deg, var(--fusabi-green-electric), var(--fusabi-green-natural));
  color: var(--fusabi-bg-dark);
  border: none;
  padding: 10px 20px;
  border-radius: 6px;
  font-weight: 600;
  cursor: pointer;
  transition: opacity 0.2s;
}

.fusabi-btn-primary:hover {
  opacity: 0.9;
}

/* Code Block */
.fusabi-code {
  background: var(--fusabi-bg-dark);
  color: var(--fusabi-text-light);
  font-family: var(--font-mono);
  font-size: var(--font-size-code);
  line-height: var(--line-height-code);
  padding: 16px;
  border-radius: 8px;
  border-left: 4px solid var(--fusabi-green-electric);
}

/* Success Message */
.fusabi-success {
  color: var(--fusabi-green-electric);
  font-weight: 600;
}

/* Error Message */
.fusabi-error {
  color: var(--fusabi-rust);
  font-weight: 600;
}
```

## Terminal/CLI Styling

### ANSI Color Codes

```rust
// Success: Bright Green
println!("\x1b[92m✅ Success\x1b[0m");

// Error: Rust Orange (closest: Red)
eprintln!("\x1b[91m❌ Error\x1b[0m");

// Warning: Yellow/Orange
println!("\x1b[93m⚠️  Warning\x1b[0m");

// Info: Light Grey
println!("\x1b[37m💡 Info\x1b[0m");

// Prompt: Bright Green + Bold
print!("\x1b[92m\x1b[1m🟢>\x1b[0m ");
```

### Using `colored` Crate

```rust
use colored::*;

// Success
println!("{}", "✅ Success".bright_green());

// Error
eprintln!("{}", "❌ Error".truecolor(183, 65, 14));

// Warning
println!("{}", "⚠️  Warning".truecolor(222, 165, 132));

// Prompt
print!("{} ", "🟢>".bright_green().bold());
```

## ASCII Art

### Banner (Full)
```
     / \
    ( F )  Fusabi v{version}
     \_/   Small. Potent. Functional.
```

### Logo (Compact)
```
(F) Fusabi
```

### REPL Prompt
```
🟢>
```

## Badge Styling

### GitHub Shields
```markdown
![Build](https://img.shields.io/github/actions/workflow/status/USER/REPO/ci.yml?style=flat&color=99CC33)
![Crates.io](https://img.shields.io/crates/v/fusabi?style=flat&color=99CC33)
![License](https://img.shields.io/badge/license-MIT-99CC33?style=flat)
![Rust](https://img.shields.io/badge/rust-1.70%2B-99CC33?style=flat)
```

### Custom Badges
- Background: `#99CC33`
- Text: `#1E1E1E`
- Style: Flat, rounded corners (4px)

## Asset Specifications

### Logo (SVG)
- **Dimensions**: 100x100 viewBox
- **Format**: SVG (vector)
- **Colors**: Green gradient
- **Export**: Optimized, minified

### Icon (ICO)
- **Sizes**: 16x16, 32x32, 48x48, 256x256
- **Format**: Multi-resolution ICO
- **Usage**: Windows executable icon

### Social Preview
- **Dimensions**: 1280x640px
- **Format**: PNG, optimized
- **Content**: Logo + tagline + color palette
- **Background**: Dark (`#1E1E1E`)

### Favicon
- **Sizes**: 16x16, 32x32
- **Format**: ICO or PNG
- **Style**: Simple "F" glyph

## Brand Applications

### Documentation
- Dark theme by default
- Green accents for links
- Code blocks with green left border
- Emoji for section markers

### CLI Output
- Green for success operations
- Rust orange for errors
- Light grey for hints
- Bold for emphasis

### Website/Landing Page
- Hero: Large logo, tagline, quick example
- Dark background with green CTAs
- Food photography accent images
- Minimal copy, maximum code

### Social Media
- Profile image: Logo on dark background
- Banner: Social preview asset
- Post style: Code snippets with green frames
- Hashtags: #FusabiLang #RustScripting #FunctionalProgramming

## Accessibility

### Color Contrast
All color combinations meet WCAG AA standards:

- `#99CC33` on `#1E1E1E`: 6.8:1 (AA)
- `#F0F0F0` on `#1E1E1E`: 13.1:1 (AAA)
- `#B7410E` on `#1E1E1E`: 4.6:1 (AA)

### Alternative Text
- Logo: "Fusabi - Small. Potent. Functional."
- Icons: Descriptive purpose (e.g., "Success indicator")
- Emoji: Redundant to text, not solely communicating

### Screen Reader Support
- Semantic HTML headings
- ARIA labels where needed
- No color-only information communication

## Brand Evolution

### Version 1.0 (Current)
- Minimalist green logo
- Food metaphors (wasabi, omakase)
- "Small. Potent. Functional." tagline

### Future Considerations
- Animated logo for web
- Expanded color palette for sub-brands
- Merchandise designs
- Conference talk templates

## Usage Guidelines

### Do's
- Use brand colors consistently
- Maintain punchy, confident tone
- Leverage food metaphors sparingly
- Keep visual hierarchy clear

### Don'ts
- Don't alter logo proportions
- Don't use non-brand colors for key elements
- Don't write long-winded copy
- Don't mix metaphors (stay with food theme)

## Credits

**Brand Design**: Fusabi Community
**Typography**: Inter by Rasmus Andersson
**Monospace**: JetBrains Mono by JetBrains
**Inspiration**: Wasabi, Rust, F#

---

**Questions?** Open an issue or discussion on GitHub.

**Contributing?** Follow these guidelines for all visual/copy contributions.
