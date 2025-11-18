# blog

This is a small static site generator that converts markdown files to HTML for my github pages blog.

## Usage

```bash
cargo build --release
./target/release/generator path/to/input.md path/to/output_directory/
```

## Deployment

Fork and Push to main branch - GitHub Actions automatically builds the generator, processes all markdown files, and deploys to GitHub Pages.
