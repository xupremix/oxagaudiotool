pub mod error {
    use kira::manager::backend::cpal::Error;
    use kira::manager::error::PlaySoundError;
    use kira::sound::FromFileError;
    use std::fmt::{Display, Formatter};

    #[derive(Debug)]
    pub enum OxAgAudioToolError {
        AudioManagerError(Error),
        FileError(FromFileError),
        PlaySoundError(PlaySoundError<()>),
    }

    impl Display for OxAgAudioToolError {
        fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
            let error_string = match self {
                OxAgAudioToolError::AudioManagerError(e) => format!("{}", e),
                OxAgAudioToolError::FileError(e) => format!("{}", e),
                OxAgAudioToolError::PlaySoundError(e) => format!("{}", e),
            };

            write!(f, "{}", error_string)
        }
    }

    impl From<Error> for OxAgAudioToolError {
        fn from(value: Error) -> Self {
            OxAgAudioToolError::AudioManagerError(value)
        }
    }

    impl From<FromFileError> for OxAgAudioToolError {
        fn from(value: FromFileError) -> Self {
            OxAgAudioToolError::FileError(value)
        }
    }

    impl From<PlaySoundError<()>> for OxAgAudioToolError {
        fn from(value: PlaySoundError<()>) -> Self {
            OxAgAudioToolError::PlaySoundError(value)
        }
    }
}
