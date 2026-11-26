//! A copy of the ElevenLabs skills (github.com/elevenlabs/skills) in a temp
//! folder, for a coding agent to read: the one way a command hands an agent a
//! skill. The CLI downloads and unpacks the repo itself, so it is the same on
//! every OS and in every shell, and nothing lands in the developer's project.
//! `onboard`'s hand-over names the command that runs it, since its agent only
//! fetches when a skill would help; `migrate` runs it straight away.

use std::io::Read;
use std::path::{Component, Path, PathBuf};
use std::time::Duration;

use fern_cli_sdk::error::CliError;
use fern_cli_sdk::openapi::AppContext;

use super::say::run_async as block_on;

/// The whole repo in one download, from the host that GitHub's archive links
/// redirect to: the CLI's client follows no redirect to another host.
const ARCHIVE: &str = "https://codeload.github.com/elevenlabs/skills/tar.gz/refs/heads/main";

/// A copy served from this machine instead, for tests of an unreleased skill.
const ARCHIVE_ENV: &str = "ELEVENLABS_SKILLS_ARCHIVE";

/// How long one try may take, the download included.
const TRY_LIMIT: Duration = Duration::from_secs(30);

/// Far above the repo's size (about half a megabyte packed), so a download
/// that isn't the skills is dropped before it fills memory or the disk.
const MAX_PACKED: usize = 20 * 1024 * 1024;
const MAX_UNPACKED: u64 = 100 * 1024 * 1024;

/// A file of the copy: its path inside the skills folder, and its contents.
type File = (PathBuf, Vec<u8>);

/// Why a try failed, and whether another one could go differently.
#[derive(Debug)]
struct Failed {
    why: String,
    again: bool,
}

impl Failed {
    fn for_good(why: impl Into<String>) -> Self {
        Self { why: why.into(), again: false }
    }

    fn this_time(why: impl Into<String>) -> Self {
        Self { why: why.into(), again: true }
    }
}

/// A new folder for the skills in the system's temp folder, never the
/// project, so nothing lands in the developer's repo. Its name is random, so
/// no one can have put anything there first.
pub(super) fn new_dir() -> Result<PathBuf, CliError> {
    tempfile::Builder::new()
        .prefix("elevenlabs-skills-")
        .tempdir()
        .map(tempfile::TempDir::keep)
        .map_err(|e| CliError::Other(anyhow::anyhow!("create a folder for the ElevenLabs skills: {e}")))
}

/// Puts the skills in `dir`, one folder per skill, and returns their names,
/// or why it couldn't. Nothing is written unless the whole download arrived
/// and unpacked.
pub(super) fn fetch(ctx: &AppContext, dir: &Path) -> Result<Vec<String>, String> {
    let client = ctx.http_config().build_client().map_err(|e| e.to_string())?;
    let url = archive_url();
    // One more try covers a dropped connection. A second failure, or one that
    // another try can't change, is the answer.
    let files = match block_on(download(&client, &url, TRY_LIMIT)) {
        Err(Failed { again: true, .. }) => block_on(download(&client, &url, TRY_LIMIT)),
        first => first,
    }
    .map_err(|failed| failed.why)?;
    for (path, contents) in &files {
        let to = dir.join(path);
        to.parent()
            .map_or(Ok(()), std::fs::create_dir_all)
            .and_then(|()| std::fs::write(&to, contents))
            .map_err(|e| format!("{} couldn't be written: {e}", to.display()))?;
    }
    Ok(skills_in(&files))
}

/// `name`'s guide in a copy `fetch` made.
pub(super) fn guide(dir: &Path, name: &str) -> Result<PathBuf, String> {
    let guide = dir.join(name).join("SKILL.md");
    if guide.is_file() {
        Ok(guide)
    } else {
        Err(format!("the ElevenLabs skills have no `{name}` skill yet"))
    }
}

/// Only a copy on this machine may stand in for GitHub's.
fn archive_url() -> String {
    std::env::var(ARCHIVE_ENV)
        .ok()
        .filter(|url| is_local(url))
        .unwrap_or_else(|| ARCHIVE.to_string())
}

fn is_local(url: &str) -> bool {
    reqwest::Url::parse(url).is_ok_and(|url| {
        url.host_str().is_some_and(|host| {
            host.eq_ignore_ascii_case("localhost")
                || host
                    .trim_matches(|c| c == '[' || c == ']')
                    .parse::<std::net::IpAddr>()
                    .is_ok_and(|ip| ip.is_loopback())
        })
    })
}

/// One try: the archive, downloaded whole and unpacked in memory.
async fn download(client: &reqwest::Client, url: &str, limit: Duration) -> Result<Vec<File>, Failed> {
    let why = |e: reqwest::Error| {
        Failed::this_time(if e.is_timeout() {
            format!("it took longer than {} seconds", limit.as_secs())
        } else if e.is_connect() {
            "this machine couldn't reach GitHub".to_string()
        } else {
            e.without_url().to_string()
        })
    };
    let mut response = client
        .get(url)
        // GitHub needs no word about this run: the client's own names the command and the platform.
        .header(reqwest::header::USER_AGENT, "elevenlabs-cli")
        .timeout(limit)
        .send()
        .await
        .map_err(why)?;
    let status = response.status();
    if !status.is_success() {
        let answered = format!("GitHub answered {}", status.as_u16());
        return Err(if status.is_server_error() || status.as_u16() == 429 {
            Failed::this_time(answered)
        } else {
            Failed::for_good(answered)
        });
    }
    let mut packed = Vec::new();
    while let Some(chunk) = response.chunk().await.map_err(why)? {
        if packed.len() + chunk.len() > MAX_PACKED {
            return Err(Failed::for_good("the download is far larger than the skills"));
        }
        packed.extend_from_slice(&chunk);
    }
    unpack(&packed)
}

/// The files of a `.tar.gz` of the repo, without the one top folder GitHub
/// wraps it in. Only plain files count: a link could point outside the copy.
fn unpack(packed: &[u8]) -> Result<Vec<File>, Failed> {
    // A download cut short reads as a broken archive, and may arrive whole next time.
    let broken =
        |e: std::io::Error| Failed::this_time(format!("the download is incomplete or isn't the skills ({e})"));
    let mut files = Vec::new();
    let mut size = 0;
    let mut archive = tar::Archive::new(flate2::read::GzDecoder::new(packed));
    for entry in archive.entries().map_err(broken)? {
        let mut entry = entry.map_err(broken)?;
        if !entry.header().entry_type().is_file() {
            continue;
        }
        let path: PathBuf = entry.path().map_err(broken)?.components().skip(1).collect();
        if path.as_os_str().is_empty() {
            continue;
        }
        if !path.components().all(|c| matches!(c, Component::Normal(_))) {
            return Err(Failed::for_good(format!(
                "the download holds a file outside the skills folder ({})",
                path.display()
            )));
        }
        size += entry.size();
        if size > MAX_UNPACKED {
            return Err(Failed::for_good("the download is far larger than the skills"));
        }
        let mut contents = Vec::new();
        entry.read_to_end(&mut contents).map_err(broken)?;
        files.push((path, contents));
    }
    if skills_in(&files).is_empty() {
        return Err(Failed::for_good("the download holds no skills"));
    }
    Ok(files)
}

/// The skills among `files`: each top folder with a `SKILL.md` of its own.
fn skills_in(files: &[File]) -> Vec<String> {
    let mut names: Vec<String> = files
        .iter()
        .filter(|(path, _)| path.components().count() == 2 && path.ends_with("SKILL.md"))
        .filter_map(|(path, _)| Some(path.parent()?.to_str()?.to_string()))
        .collect();
    names.sort();
    names
}

#[cfg(test)]
mod tests {
    use super::*;
    use wiremock::matchers::method;
    use wiremock::{Mock, MockServer, ResponseTemplate};

    /// A `.tar.gz` holding `files`, each a path and its contents.
    fn packed(files: &[(&str, &str)]) -> Vec<u8> {
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(Vec::new(), Default::default()));
        for (path, contents) in files {
            let mut header = tar::Header::new_gnu();
            header.set_size(contents.len() as u64);
            header.set_mode(0o644);
            // `append_data` refuses `..`, which is the path one test needs.
            header.as_gnu_mut().unwrap().name[..path.len()].copy_from_slice(path.as_bytes());
            header.set_cksum();
            tar.append(&header, contents.as_bytes()).unwrap();
        }
        tar.into_inner().unwrap().finish().unwrap()
    }

    fn paths(files: &[File]) -> Vec<String> {
        files
            .iter()
            .map(|(path, _)| path.to_str().unwrap().replace('\\', "/"))
            .collect()
    }

    #[test]
    fn the_repo_unpacks_without_its_top_folder() {
        let files = unpack(&packed(&[
            ("skills-main/README.md", "readme"),
            ("skills-main/text-to-speech/SKILL.md", "tts"),
            ("skills-main/text-to-speech/references/streaming.md", "streaming"),
            ("skills-main/.agents/skills/update/SKILL.md", "not one of the skills"),
        ]))
        .unwrap();
        assert_eq!(
            paths(&files),
            [
                "README.md",
                "text-to-speech/SKILL.md",
                "text-to-speech/references/streaming.md",
                ".agents/skills/update/SKILL.md"
            ]
        );
        assert_eq!(files[1].1, b"tts");
        assert_eq!(skills_in(&files), ["text-to-speech"]);
    }

    #[test]
    fn a_file_outside_the_folder_stops_the_copy() {
        let failed = unpack(&packed(&[
            ("skills-main/text-to-speech/SKILL.md", "tts"),
            ("skills-main/../../evil", "x"),
        ]))
        .unwrap_err();
        assert!(failed.why.contains("outside the skills folder"), "{failed:?}");
        assert!(!failed.again, "another try brings the same archive");
    }

    #[test]
    fn links_are_left_out() {
        let mut tar = tar::Builder::new(flate2::write::GzEncoder::new(Vec::new(), Default::default()));
        let mut link = tar::Header::new_gnu();
        link.set_entry_type(tar::EntryType::Symlink);
        link.set_size(0);
        tar.append_link(&mut link, "skills-main/agents/SKILL.md", "/etc/passwd").unwrap();
        let mut file = tar::Header::new_gnu();
        file.set_size(3);
        file.set_mode(0o644);
        tar.append_data(&mut file, "skills-main/music/SKILL.md", &b"mus"[..]).unwrap();
        let files = unpack(&tar.into_inner().unwrap().finish().unwrap()).unwrap();
        assert_eq!(paths(&files), ["music/SKILL.md"]);
    }

    #[test]
    fn a_download_that_is_not_the_skills_is_refused() {
        let no_skills = unpack(&packed(&[("skills-main/README.md", "x")])).unwrap_err();
        assert!(no_skills.why.contains("no skills") && !no_skills.again, "{no_skills:?}");
        // A page in place of the archive, or a download cut short, may be whole on another try.
        let whole = packed(&[("skills-main/music/SKILL.md", &"m".repeat(4000))]);
        for bad in [&b"<html>Busy</html>"[..], &whole[..whole.len() / 2]] {
            let failed = unpack(bad).unwrap_err();
            assert!(failed.why.contains("incomplete") && failed.again, "{failed:?}");
        }
    }

    #[test]
    fn only_a_copy_on_this_machine_replaces_github() {
        assert!(is_local("http://127.0.0.1:8080/skills.tar.gz"));
        assert!(is_local("http://localhost:8080/skills.tar.gz"));
        assert!(is_local("http://[::1]:8080/skills.tar.gz"));
        assert!(!is_local("https://example.com/skills.tar.gz"));
        assert!(!is_local("https://127.0.0.1.example.com/skills.tar.gz"));
        assert!(!is_local("skills.tar.gz"));
    }

    #[tokio::test]
    async fn a_slow_download_ends_at_the_limit() {
        let server = MockServer::start().await;
        Mock::given(method("GET"))
            .respond_with(
                ResponseTemplate::new(200)
                    .set_body_bytes(packed(&[("skills-main/music/SKILL.md", "m")]))
                    .set_delay(Duration::from_secs(5)),
            )
            .mount(&server)
            .await;
        let started = std::time::Instant::now();
        let failed = download(&reqwest::Client::new(), &server.uri(), Duration::from_millis(300))
            .await
            .unwrap_err();
        assert!(failed.why.contains("took longer") && failed.again, "{failed:?}");
        assert!(started.elapsed() < Duration::from_secs(3));
    }

    #[tokio::test]
    async fn only_a_failure_another_try_could_change_is_tried_again() {
        for (status, again) in [(404, false), (403, false), (503, true), (429, true)] {
            let server = MockServer::start().await;
            Mock::given(method("GET"))
                .respond_with(ResponseTemplate::new(status).set_body_string("Not the skills"))
                .mount(&server)
                .await;
            let failed = download(&reqwest::Client::new(), &server.uri(), TRY_LIMIT)
                .await
                .unwrap_err();
            assert_eq!(failed.why, format!("GitHub answered {status}"));
            assert_eq!(failed.again, again, "{status}");
        }
    }
}
