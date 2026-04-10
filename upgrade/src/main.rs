use bstr::ByteSlice;
use git2::{ObjectType, Repository};
use regex::Regex;
use std::env;
use toml_edit::{DocumentMut, value};

fn main() {
    env::set_current_dir(concat!(env!("CARGO_MANIFEST_DIR"), "/..")).unwrap();
    let mut cargo_toml = std::fs::read_to_string("./ash/Cargo.toml")
        .unwrap()
        .parse::<DocumentMut>()
        .unwrap();
    let version = &mut cargo_toml["package"]["version"];
    let version_regex = Regex::new(r#"(\d+)\.(\d+)\.(\d+)\+(\d+)\.(\d+)\.(\d+)"#).unwrap();
    let captures = version_regex.captures(version.as_str().unwrap()).unwrap();
    let old_crate_version = [
        captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
    ];
    let old_tag = [
        captures.get(4).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(5).unwrap().as_str().parse::<u32>().unwrap(),
        captures.get(6).unwrap().as_str().parse::<u32>().unwrap(),
    ];
    let Some(new_tag) = checkout_new_tag(old_tag) else {
        return;
    };
    let mut new_crate_version = old_crate_version;
    for n in &mut new_crate_version {
        if *n != 0 {
            *n += 1;
            break;
        }
    }
    let new_version = format!(
        "{}.{}.{}+{}.{}.{}",
        new_crate_version[0],
        new_crate_version[1],
        new_crate_version[2],
        new_tag[0],
        new_tag[1],
        new_tag[2],
    );
    *version = value(&new_version);
    std::fs::write("./ash/Cargo.toml", cargo_toml.to_string()).unwrap();
}

fn checkout_new_tag(old_tag: [u32; 3]) -> Option<[u32; 3]> {
    let tag_regex = Regex::new(r#"^refs/tags/v(\d+)\.(\d+)\.(\d+)$"#).unwrap();
    let repo = Repository::open("generator/Vulkan-Headers").unwrap();
    let mut newest_tag = old_tag;
    let mut tag_commit = None;
    repo.tag_foreach(|a, b| {
        let Some(captures) = tag_regex.captures(b.as_bstr().to_str().unwrap()) else {
            return true;
        };
        let tag = [
            captures.get(1).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(2).unwrap().as_str().parse::<u32>().unwrap(),
            captures.get(3).unwrap().as_str().parse::<u32>().unwrap(),
        ];
        if tag <= newest_tag {
            return true;
        }
        newest_tag = tag;
        let obj = repo.find_object(a, None).unwrap();
        let commit = match obj.kind().unwrap() {
            ObjectType::Commit => obj.into_commit(),
            ObjectType::Tag => obj.as_tag().unwrap().target().unwrap().into_commit(),
            ObjectType::Any | ObjectType::Tree | ObjectType::Blob => unreachable!(),
        };
        tag_commit = Some(commit.unwrap());
        true
    })
    .unwrap();
    let commit = tag_commit?;
    repo.checkout_tree(&commit.as_object(), None).unwrap();
    repo.set_head_detached(commit.id()).unwrap();
    Some(newest_tag)
}
