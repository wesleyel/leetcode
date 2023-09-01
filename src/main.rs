use leetcode_cli::helper::HTML;
use leetcode_cli::{Cache, Error};
use serde_json::Value;
use std::fs;
use std::io::Write;
use std::path::Path;
use tokio::runtime::Builder;
const TEMPLATE_RS: &str = r#"
/**
 * [__PROBLEM_ID__] __PROBLEM_TITLE__
 *
 * __PROBLEM_DESC__
 */
pub struct Solution {}

// problem: __PROBLEM_LINK__
// discuss: __DISCUSS_LINK__

// submission codes start here

__PROBLEM_DEFAULT_CODE__

// submission codes end

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test___PROBLEM_ID__() {

    }
}
"#;

fn build_desc(content: &str) -> String {
    let mut object: Value = serde_json::from_str(content).unwrap();
    let content = object
        .get_mut("content")
        .unwrap()
        .as_str()
        .unwrap()
        .to_string()
        .render();
    content.replace("\n", "\n * ")
}

pub async fn init() -> Result<(), Error> {
    let cache = Cache::new().unwrap();
    // accept fid: i32 from args
    let fid = std::env::args()
        .nth(1)
        .unwrap_or_else(|| {
            println!("Usage: leetcode <fid>");
            std::process::exit(1);
        })
        .parse::<i32>()
        .unwrap_or_else(|_| {
            println!("Usage: leetcode <fid>");
            std::process::exit(1);
        });

    let problem = cache.get_problem(fid)?;
    let question = cache.get_question(fid).await?;
    let code = question.defs.0.iter().filter(|x| x.value == "rust");
    let code = code.to_owned().next().unwrap().code.as_str();

    let source = TEMPLATE_RS
        .replace("__PROBLEM_TITLE__", problem.name.as_str())
        .replace("__PROBLEM_ID__", problem.fid.to_string().as_str())
        .replace("__PROBLEM_DESC__", &build_desc(problem.desc.as_str()))
        .replace(
            "__PROBLEM_LINK__",
            format!("https://leetcode.cn/problems/{}/", problem.slug).as_str(),
        )
        .replace(
            "__DISCUSS_LINK__",
            format!(
                "https://leetcode.cn/problems/{}/discuss/?currentPage=1&orderBy=most_votes&query=",
                problem.slug
            )
            .as_str(),
        )
        .replace("__PROBLEM_DEFAULT_CODE__", code);
    let file_name = format!("p{:04}", problem.fid);
    let file_path = Path::new("./src/bin").join(format!("{}.rs", file_name));
    if file_path.exists() {
        panic!("problem already initialized");
    }

    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&file_path)
        .unwrap();

    file.write_all(source.as_bytes()).unwrap();
    drop(file);
    Ok(())
}

fn main() {
    if let Err(err) = Builder::new_multi_thread()
        .enable_all()
        .build()
        .expect("Build tokio runtime failed")
        .block_on(init())
    {
        println!("{:?}", err);
    }
}
