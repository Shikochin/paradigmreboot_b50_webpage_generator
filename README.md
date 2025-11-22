# Paradigm: Reboot B50 Webpage Generator

This is a tool used to generate a webpage showcasing Paradigm: Reboot Best 50.

## Usage

1. Install Rust (https://rust-lang.org)

2. Place the `records.csv` file (available at <http://prp.icel.site>) in the project root directory.

3. Save [the wiki page](https://paradigmrebootzh.miraheze.org/wiki/曲目列表) as `wiki.html` and place it in the project root directory, or just simply run `deno --allow-all get_wikihtml.ts` to get it.

4. Run `cargo run`, which will automatically download song covers and generate the `b50_slides.html` file.

## Thanks
- Gemini 3 Pro
- Copilot
- [范式：起源 中文维基](https://paradigmrebootzh.miraheze.org/)

## LICENSE

[The Unlicense](https://unlicense.org)