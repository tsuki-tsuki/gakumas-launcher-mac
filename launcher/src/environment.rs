use std::{ffi::OsString, path::PathBuf};

pub fn host_path(path: PathBuf, windows: bool) -> PathBuf {
    let text = path.as_os_str().to_string_lossy();
    if windows && text.starts_with('/') && !text.starts_with("//") {
        PathBuf::from(format!("Z:{text}"))
    } else {
        path
    }
}

pub fn working_dir(notproton: Option<OsString>, cwd: PathBuf) -> PathBuf {
    notproton
        .filter(|path| !path.is_empty())
        .map(PathBuf::from)
        .unwrap_or(cwd)
}

pub fn config_home(
    home: Option<OsString>,
    profile: Option<OsString>,
    windows: bool,
) -> Option<PathBuf> {
    home.or(profile).map(|path| host_path(path.into(), windows))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn windows_can_access_crossover_host_paths() {
        assert_eq!(
            host_path("/Volumes/External Disk/Gakumas".into(), true),
            PathBuf::from("Z:/Volumes/External Disk/Gakumas")
        );
        assert_eq!(
            host_path("C:/Games/Gakumas".into(), true),
            PathBuf::from("C:/Games/Gakumas")
        );
        assert_eq!(host_path("game".into(), true), PathBuf::from("game"));
        assert_eq!(
            host_path("//server/share/game".into(), true),
            PathBuf::from("//server/share/game")
        );
        assert_eq!(
            host_path("/Volumes/Gakumas".into(), false),
            PathBuf::from("/Volumes/Gakumas")
        );
    }

    #[test]
    fn notproton_workdir_overrides_the_process_directory() {
        assert_eq!(
            working_dir(Some("/Volumes/Gakumas".into()), "/tmp".into()),
            PathBuf::from("/Volumes/Gakumas")
        );
        assert_eq!(working_dir(None, "/tmp".into()), PathBuf::from("/tmp"));
        assert_eq!(
            working_dir(Some("".into()), "/tmp".into()),
            PathBuf::from("/tmp")
        );
    }

    #[test]
    fn windows_reuses_the_native_launchers_session_directory() {
        assert_eq!(
            config_home(
                Some("/Users/player".into()),
                Some("C:/users/player".into()),
                true
            ),
            Some(PathBuf::from("Z:/Users/player"))
        );
        assert_eq!(
            config_home(None, Some("C:/users/player".into()), true),
            Some(PathBuf::from("C:/users/player"))
        );
    }
}
