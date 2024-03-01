use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(version, about)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Authorise your b2 account
    #[command(alias = "authorize")]
    Authorise,
    // TODO: CancelAllUnfinishedLargeFiles {},
    // TODO: CancelLargeFile {},
    // TODO: ClearAccount {},
    // TODO: CopyFileById {},
    // TODO: CreateBucket {},
    // TODO: CreateKey {},
    // TODO: DeleteBucket {},
    // TODO: DeleteFileVersion {},
    // TODO: DeleteKey {},
    /// Download a file from a bucket
    Download {
        /// The file into which the contents will be downloaded -- If not specified, it will download into the current directory using the file name on b2
        #[arg(short = 'O', long, value_name = "file")]
        output: Option<PathBuf>,
        /// The bucket from which to download the file
        #[arg(value_name = "bucket")]
        bucket: String,
        /// The path from which to download the file
        #[arg(value_name = "file")]
        file: PathBuf,
    },
    Cat {
        /// Force the file to be printed even if it is not text
        #[arg(short, long)]
        force: bool,
        /// The bucket from which to download the file
        #[arg(value_name = "bucket")]
        bucket: String,
        /// The path from which to download the file
        #[arg(value_name = "file")]
        file: PathBuf,
    },
    // TODO: GetAccountInfo {},
    // TODO: GetBucket {},
    // TODO: FileInfo {},
    // TODO: GetDownloadAuth {},
    // TODO: GetDownloadUrlWithAuth {},
    // TODO: HideFile {},
    /// List the buckets (also force-updates the bucket cache)
    ListBuckets,
    // TODO: ListKeys {},
    // TODO: ListParts {},
    // TODO: ListUnfinishedLargeFiles {},
    /// Show files in a specific bucket
    Ls {
        #[arg(short, long)]
        long: bool,
        bucket: String,
    },
    // TODO: Rm {},
    // TODO: GetUrl {},
    // TODO: Sync {},
    // TODO: UpdateBucket {},
    /// Upload a file to b2, if `dest` is not specified, then it will take the name of the file
    /// that is uploaded.
    Upload {
        /// Upload the file using the "parts" api
        /// Note: this is automatically enabled if the file that is being uploaded is more than 1GiB
        #[arg(short, long)]
        parts: bool,
        /// Manually override the Content Type of the file rather than trying to guess from the
        /// file extension
        #[arg(short, long, value_name = "content-type")]
        content_type: Option<String>,
        /// Upload directories recursively
        #[arg(short, long)]
        recursive: bool,
        /// The path to the file to upload
        #[arg(value_name = "file")]
        file: PathBuf,
        /// The bucket into which the file should be uploaded
        #[arg(value_name = "bucket")]
        bucket: String,
        /// The destination file path relative to the root of the bucket
        #[arg(value_name = "dest")]
        dest: Option<PathBuf>,
    },
    // TODO: UploadUnboundStream {},
    // TODO: UpdateFileLegalHold {},
    // TODO: UpdateFileRetention {},
    // TODO: ReplicationSetup {},
    // TODO: ReplicationDelete {},
    // TODO: ReplicationPause {},
    // TODO: ReplicationUnpause {},
    // TODO: ReplicationStatus {},
    // TODO: Version {},
    // TODO: License {},
    // TODO: InstallAutocomplete {},
}
