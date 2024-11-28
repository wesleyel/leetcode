use regex::Regex;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;

struct Problem {
    id: String,
    name: String,
    link: String,
    category: String,
    tags: String,
    difficulty: String,
    path: String,
}

const README_TEMPLATE: &str = r#"# LeetCode rust

这是我在 LeetCode 上的 rust 解题仓库，主要用于记录自己的解题思路和代码。

<MARKDOWN_TABLE>

## Config

- 使用魔改的vscode插件[LeetCode modified](https://marketplace.visualstudio.com/items?itemName=magicwenli.vscode-leetcode-modified)
- 修改git pre-commit hook

```shell
#!/bin/sh
cargo fmt --all
cargo run --bin readme_gen
git add README.md
```

template修改:

使用magicwenli.vscode-leetcode-modified最新版会自动修改rust的预制模板。

<details><summary>手动修改vsc-leetcode-cli</summary

- 手动修改template: `vscode-insiders\data\extensions\magicwenli.vscode-leetcode-modified-0.18.4\node_modules\vsc-leetcode-cli\templates\detailed.tpl`, 给template加上一些额外的东西

```rust
struct Solution;
fn main() {}

${comment.singleLine} @lc code=start
${code}
${comment.singleLine} @lc code=end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {}
}
```
</details>

## License

MIT

"#;

fn main() -> io::Result<()> {
    let bin_dir = Path::new("src/bin");
    let readme_path = Path::new("README.md");
    let mut problems: Vec<Problem> = Vec::new();

    // Define regex patterns
    let re_id = Regex::new(r"@lc app=leetcode\.cn id=(\d+) lang=rust").unwrap();
    let re_name = Regex::new(r"\[\d+\] (.+)").unwrap();
    let re_link = Regex::new(r"https?://leetcode\.cn/problems/[^\s]+").unwrap();
    let re_category = Regex::new(r" \* category: ([\w,-]+)").unwrap();
    let re_tags = Regex::new(r" \* tags: ([\w,-]+)").unwrap();
    let re_difficulty = Regex::new(r"(\w+) \(\d+\.\d+%\)").unwrap();

    // Iterate over each .rs file in src/bin
    for entry in fs::read_dir(bin_dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("rs") {
            // 读取整个文件内容
            let content = fs::read_to_string(&path)?;
            let mut problem = Problem {
                id: String::new(),
                name: String::new(),
                link: String::new(),
                category: String::new(),
                tags: String::new(),
                difficulty: String::new(),
                path: path.file_name().unwrap().to_str().unwrap().to_string(),
            };
            // 在全文中搜索并提取信息
            if let Some(caps) = re_id.captures(&content) {
                problem.id = caps[1].to_string();
            }
            if let Some(caps) = re_name.captures(&content) {
                problem.name = caps[1].trim().to_string();
            }
            if let Some(caps) = re_link.captures(&content) {
                problem.link = caps[0].to_string();
            }
            if let Some(caps) = re_category.captures(&content) {
                problem.category = caps[1].to_string();
            }
            if let Some(caps) = re_tags.captures(&content) {
                problem.tags = caps[1].to_string();
            }
            if let Some(caps) = re_difficulty.captures(&content) {
                problem.difficulty = caps[1].to_string();
            }

            if !problem.id.is_empty() {
                problems.push(problem);
            }
        }
    }

    problems.sort_by(|a, b| {
        a.id.parse::<i32>()
            .unwrap()
            .cmp(&b.id.parse::<i32>().unwrap())
    });

    // Generate Markdown table
    let mut markdown = String::from("| ID | 名称 | 分类 | 标签 | 难度 | 链接 | 路径 |\n");
    markdown.push_str("|----|------|------|------|------|------|------|\n");
    for p in problems {
        markdown.push_str(&format!(
            "| {} | {} | {} | {} | {} | [链接]({}) | {} |\n",
            p.id,
            p.name,
            p.category,
            p.tags,
            p.difficulty,
            p.link,
            format!("[源码](src/bin/{})", p.path)
        ));
    }

    // Write to README.md
    let mut readme = File::create(readme_path)?;
    let markdown = README_TEMPLATE.replace("<MARKDOWN_TABLE>", &markdown);
    readme.write_all(markdown.as_bytes())?;

    Ok(())
}
