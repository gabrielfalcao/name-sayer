use fast_glob::glob_match;
use name_sayer::Exit;

fn main() -> Exit
{
    let glob = "*.sentry-cdn.com";
    let path = "www.sentry-cdn.com";
    let matched = glob_match(glob, path);
    eprintln!("glob: {glob:#?}");
    eprintln!("path: {path:#?}");
    eprintln!("matched: {matched:#?}");
    Exit::Success
}
