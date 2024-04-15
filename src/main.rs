use std::{
    env,
    fs::{self, File},
    io::{BufRead, BufReader, Write},
    path::PathBuf,
    process::{exit, Command},
    str::FromStr,
};

use clap::Parser;
use csv::ReaderBuilder;
use regex::Regex;
use url::Url;
use walkdir::WalkDir;

use mega_tool::command::{Cli, Commands};

fn main() {
    // convert_origin();
    // convert0817();
    // move_file_0826_github();
    // convert_script();
    // convert_cratesio_csv();
    let args = Cli::parse();
    match args.command {
        Commands::Upload => {
            add_and_push_to_remote(args.workspace);
        }
    }
}

pub fn add_and_push_to_remote(workspace: PathBuf) {
    for entry in WalkDir::new(workspace)
        .max_depth(2)
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.file_type().is_dir() && entry.depth() == 2 {
            if let Err(err) = env::set_current_dir(entry.path()) {
                eprintln!("Error changing directory: {}", err);
                exit(1);
            }

            let output = Command::new("git")
                .arg("remote")
                .arg("-v")
                .output()
                .unwrap();

            if output.status.success() {
                let stdout = String::from_utf8_lossy(&output.stdout);
                // Create a regular expression pattern to match URLs
                let re = Regex::new(r"https://github\.com/[^\s]+").unwrap();

                // Iterate over matches in the input string

                // for capture in re.captures_iter(&stdout) {
                let mut capture = re.captures_iter(&stdout);
                if let Some(capture) = capture.next() {
                    let mut url = Url::parse(&capture[0]).unwrap();
                    url.set_host(Some("localhost")).unwrap();
                    url.set_scheme("http").unwrap();
                    url.set_port(Some(8000)).unwrap();
                    let path = url.path().to_owned();
                    let new_path = format!("/third-part{}", path);
                    url.set_path(&new_path);

                    println!("Found URL: {}", url);

                    Command::new("git")
                        .arg("remote")
                        .arg("remove")
                        .arg("nju")
                        .output()
                        .unwrap();

                    Command::new("git")
                        .arg("remote")
                        .arg("add")
                        .arg("nju")
                        .arg(url.to_string())
                        .output()
                        .unwrap();
                    let push_res = Command::new("git")
                        .arg("push")
                        .arg("nju")
                        // .arg("main")
                        .output()
                        .unwrap();

                    println!("Push res: {}", String::from_utf8_lossy(&push_res.stdout));
                    // break;
                }
            } else {
                let stderr = String::from_utf8_lossy(&output.stderr);
                eprintln!("Error running 'git remote -v':\n{}", stderr);
            }
            // println!("Directory: {:?}", entry.path());
        }
    }
}

pub fn convert_script() {
    let work_dir = PathBuf::from("/media/parallels/Lexar/GitHub");
    let clone_script = work_dir.join("github.sh");
    let reader = BufReader::new(File::open(clone_script).unwrap());
    let clone_script_new = work_dir.join("github_0826.sh");
    let mut new_script = File::create(clone_script_new).unwrap();
    for line in reader.lines() {
        let url = Url::parse(&line.unwrap().replace("git clone ", "")).unwrap();
        let strs: Vec<&str> = url.path().split('/').collect();
        let username = strs[1];
        let reponame = strs[2];
        let combine = format!("{}/{}", username, reponame);
        // mkdir -p trustwallet && git clone https://github.com/trustwallet/assets ./trustwallet
        let line = format!(
            "mkdir -p {} && git clone https://github.com/{} ./{}",
            combine, combine, combine
        );
        new_script.write_all(line.as_bytes()).unwrap();
        new_script.write_all(b"\n").unwrap();
    }
}

pub fn move_file_0826_github() {
    // let work_dir = PathBuf::from("/Users/yetianxing/workdir/");
    let work_dir = PathBuf::from("/media/parallels/Lexar/Gitee");
    let temp = work_dir.join("temp");
    if !temp.exists() {
        fs::create_dir(&temp).unwrap();
    }
    let clone_script = work_dir.join("gitee.sh");

    let reader = BufReader::new(File::open(clone_script).unwrap());
    for line in reader.lines() {
        let url = Url::parse(&line.unwrap().replace("git clone ", "")).unwrap();
        let strs: Vec<&str> = url.path().split('/').collect();
        let username = strs[1];
        let reponame = strs[2];
        let current_name = work_dir.join(reponame);
        let target_name = work_dir.join(username).join(reponame);
        let temp_name = temp.join(reponame);

        if current_name.exists() && current_name.is_dir() && !target_name.exists() {
            println!("{:?}, {:?}, {:?}", current_name, temp_name, target_name);
            std::fs::rename(&current_name, &temp_name).unwrap();
            std::fs::create_dir_all(target_name.clone()).unwrap();
            std::fs::rename(&temp_name, &target_name).unwrap();
        }
    }
    //remove tmep
    fs::remove_dir_all(temp).unwrap();
}

pub fn convert_origin() {
    let file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/gha_repo_list_top_100000.csv").unwrap();
    let github_file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/all_repositories_github.log").unwrap();
    let gitee_file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/all_repositories_gitee.log").unwrap();
    let file = File::open(file_path).unwrap();
    let mut github_file = File::create(github_file_path).unwrap();
    let mut gitee_file = File::create(gitee_file_path).unwrap();
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);
    // Iterate over the CSV records
    for result in rdr.records() {
        // Unwrap the record or handle the error
        let record = result.unwrap();

        let git_url = record.get(0).unwrap_or("");
        if !git_url.is_empty() {
            println!("Field 1: {}", git_url);
            let url = "git clone ".to_owned() + git_url;
            if git_url.contains("github.com") {
                github_file.write_all(url.as_bytes()).unwrap();
                github_file.write_all(b"\n").unwrap();
            } else {
                gitee_file.write_all(url.as_bytes()).unwrap();
                gitee_file.write_all(b"\n").unwrap();
            }
        }
    }
}

pub fn convert0817() {
    let file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/gha_repo_list_top_100000.csv").unwrap();
    let github_file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/gha_repo_list_top_100000_output.sh")
            .unwrap();

    let file = File::open(file_path).unwrap();
    let mut output = File::create(github_file_path).unwrap();
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);
    // Iterate over the CSV records
    for result in rdr.records() {
        // Unwrap the record or handle the error
        let record = result.unwrap();

        let owner = record.get(0).unwrap_or("");
        let git_url = record.get(2).unwrap_or("");
        if !git_url.is_empty() && !owner.is_empty() {
            println!("Field 1: {}", git_url);
            let command = format!("mkdir -p {} && git clone {} ./{}", owner, git_url, owner);
            output.write_all(command.as_bytes()).unwrap();
            output.write_all(b"\n").unwrap();
        }
    }
}

pub fn convert_cratesio_csv() {
    let file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/2023-10-07-020047/data/crates.csv").unwrap();
    let github_file_path =
        PathBuf::from_str("/Users/yetianxing/Downloads/crates_repo_output.log").unwrap();

    let file = File::open(file_path).unwrap();
    let mut output = File::create(github_file_path).unwrap();
    // Create a CSV reader
    let mut rdr = ReaderBuilder::new().from_reader(file);
    // Iterate over the CSV records
    for result in rdr.records() {
        // Unwrap the record or handle the error
        let record = result.unwrap();

        // let owner = record.get(0).unwrap_or("");
        let git_url = record.get(9).unwrap_or("");
        if !git_url.is_empty() && git_url.contains("github.com") {
            println!("Field 1: {}", git_url);
            let url = Url::parse(git_url).expect("Failed to parse URL");
            // Get the path segments
            let path_segments: Vec<&str> = url.path_segments().unwrap().collect();
            let owner = path_segments[0];
            let command = format!("{},{}", owner, git_url);
            output.write_all(command.as_bytes()).unwrap();
            output.write_all(b"\n").unwrap();
        }
    }
}