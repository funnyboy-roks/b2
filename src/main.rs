use std::{
    fs,
    hash::Hasher,
    io::{Seek, SeekFrom, Write},
    ops::Deref,
};

use clap::Parser;
use colored::Colorize;
use humanize_bytes::humanize_bytes_decimal;
use progress_bar::finalize_progress_bar;
use reqwest::blocking as reqwest;
use rs_sha1::{HasherContext, Sha1Hasher};
use serde::Deserialize;

use api::File;
use cli::Command;
use config::Config;

mod api;
mod cli;
mod config;
mod progress;

/// Does what it says on the can: wraps [`Sha1Hasher`] and gives it a [`Write`] implementation
struct Sha1HasherWriterWrapper(Sha1Hasher);
impl Write for Sha1HasherWriterWrapper {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        Hasher::write(&mut self.0, buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
impl Deref for Sha1HasherWriterWrapper {
    type Target = Sha1Hasher;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

fn main() -> anyhow::Result<()> {
    let cli::Cli { command } = cli::Cli::parse();
    let mut cfg = Config::load(None)?;
    match command {
        Command::Authorise => {
            cfg.auth_from_stdin()?;
        }
        Command::ListBuckets => {
            // Always update the buckets when the user asks for us to list them
            cfg.get_buckets()?;

            for bucket in cfg.buckets.keys() {
                println!("{}", bucket);
            }
        }
        Command::Ls { bucket, long } => {
            let bucket_id = cfg
                .get_bucket_id(&bucket)?
                .unwrap_or_else(|| {
                    eprintln!("Bucket `{}` does not exist", bucket);
                    std::process::exit(1);
                })
                .to_string();

            let res: serde_json::Value = cfg
                .get("b2_list_file_names")?
                .query(&[("bucketId", &bucket_id)])
                .send()?
                .json()?;

            let files: Vec<File> = Deserialize::deserialize(res["files"].clone())?;

            if long {
                println!(
                    "  {}   {}   {}",
                    "Size".underline(),
                    "Date Uploaded".underline(),
                    "Name".underline()
                );
                for file in files {
                    println!(
                        "{:>6}   {:>13}   {}",
                        humanize_bytes_decimal!(file.content_length)
                            .strip_suffix("B")
                            .unwrap()
                            .replace(' ', "")
                            .green(),
                        file.upload_timestamp.format("%e %h %Y").to_string().blue(),
                        file.file_name.yellow(),
                    );
                }
            } else {
                for file in files {
                    println!("{}", file.file_name);
                }
            }
        }
        Command::Upload {
            file,
            bucket,
            dest,
            content_type,
        } => {
            cfg.confirm_auth()?;

            if !file.is_file() {
                eprintln!(
                    "{} {}",
                    file.display().to_string().red(),
                    "is not a file.".red()
                );
            }

            let dest = dest
                .unwrap_or_else(|| {
                    file.file_name()
                        .unwrap()
                        .to_str()
                        .expect("Invalid file name")
                        .into()
                })
                .display()
                .to_string();

            let Some(bucket_id) = cfg.get_bucket_id(&bucket)? else {
                eprintln!("{}", format!("Bucket `{}` does not exist", bucket).red());
                std::process::exit(1);
            };

            let bucket_id = bucket_id.to_string();

            let res: serde_json::Value = cfg
                .get("b2_get_upload_url")?
                .query(&[("bucketId", bucket_id)])
                .send()?
                .json()?;

            let upload_url = res["uploadUrl"].as_str().unwrap();
            let auth = res["authorizationToken"].as_str().unwrap();

            let mut file = fs::File::open(file)?;
            let len = file.metadata()?.len();

            let mut sha = Sha1HasherWriterWrapper(Sha1Hasher::default());

            std::io::copy(&mut file, &mut sha)?;

            file.seek(SeekFrom::Start(0))?;

            let hash = HasherContext::finish(&mut sha.0);

            let file = progress::ReaderProgress::new(file, len as usize);

            let file: File = reqwest::Client::new()
                .post(upload_url)
                .header("Authorization", auth)
                .header("X-Bz-File-Name", urlencoding::encode(&dest).to_string())
                .header(
                    "Content-Type",
                    content_type.unwrap_or_else(|| {
                        mime_guess::from_path(dest)
                            .first_raw()
                            .unwrap_or("text/plain")
                            .into()
                    }),
                )
                .header("Content-Length", len)
                .header("X-Bz-Content-Sha1", format!("{:02x}", hash))
                .body(reqwest::Body::new(file))
                .send()?
                .json()?;

            finalize_progress_bar();
            println!(
                "{}",
                format!(
                    "Uploaded {} to {}!",
                    humanize_bytes_decimal!(len),
                    file.file_name
                )
                .green()
            );
        }
        Command::Download {
            output,
            bucket,
            file,
        } => {
            cfg.confirm_auth()?;
            let url = format!("{}/file/{}/{}", &cfg.download_url, bucket, file.display());
            let mut res = reqwest::Client::new()
                .get(&url)
                .header("Authorization", &cfg.auth_token)
                .send()?;

            let output = output
                .unwrap_or_else(|| {
                    file.file_name()
                        .unwrap()
                        .to_str()
                        .expect("Invalid file name")
                        .into()
                })
                .display()
                .to_string();

            let mut file = progress::WriterProgress::new(
                fs::File::create(&output)?,
                res.content_length().unwrap() as usize,
            );

            let n = std::io::copy(&mut res, &mut file)?;

            finalize_progress_bar();
            println!(
                "{}",
                format!("Downloaded {} to {}!", humanize_bytes_decimal!(n), output).green()
            );
        }
    };
    cfg.save()?;
    Ok(())
}
