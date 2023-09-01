//! Article metadata extracted from a specially formatted Path.

/// Date and time digits, presumed in big-endian order.
/// Precision unspecified (may be YYYYMMDD, or just YYYYMM, or YYYYMMDDHHMM, etc.).
use std::path::Path;

#[derive(Debug, PartialEq)]
pub struct DateTime(String);

#[derive(Debug, PartialEq)]
pub struct PathInfo {
    slug: String,
    datetime: DateTime,
    tags: Vec<String>,
    extension: String,
}

impl PathInfo {
    pub fn parse(path: &Path) -> Result<PathInfo, Error> {
        if path.file_name().is_none() {
            return Err(Error::MissingFileName);
        }
        let extension = path
            .extension()
            .unwrap_or_default()
            .to_str()
            .ok_or(Error::NotUTF8)?
            .to_string();

        let unwrap_normal_component = |c| match c {
            std::path::Component::Normal(s) => Some(s),
            _ => None,
        };
        let mut directory_names: Vec<String> = path
            .components()
            .rev()
            .skip(1) // skip filename
            .filter_map(unwrap_normal_component)
            .map(|s| s.to_str().map(String::from)) // OsStr -> Option<String>
            .collect::<Option<_>>() // Vec<Option<_>> -> Option<Vec<_>>
            .ok_or(Error::NotUTF8)?;

        let stem = path
            .file_stem()
            .unwrap_or_default()
            .to_str()
            .ok_or(Error::NotUTF8)?;
        let mut info = file_stem::parse_info(stem)?;
        info.tags.append(&mut directory_names);
        info.tags.sort();

        Ok(PathInfo { extension, ..info })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("file name is missing")]
    MissingFileName,
    #[error("path does not conform to UTF-8")]
    NotUTF8,
    #[error("bad file stem format")]
    BadStemFormat(#[from] peg::error::ParseError<peg::str::LineCol>),
}

// TODO: this is very ad-hoc, and not necessarily covers everything in a sensible way. Let's start
// with whatever and tweak if needed, see how it evolves with time.
peg::parser! { grammar file_stem() for str {
    /// Parse PathInfo from a filename stem.
    pub rule parse_info() -> PathInfo
        = when:datetime_prefix()? slug:slug_with_tags() extra:extra_tags()? {
            let (slug, mut tags) = slug;
            if let Some(mut extra) = extra {
                tags.append(&mut extra);
            }
            PathInfo {
                slug,
                tags,
                datetime: when.unwrap_or(DateTime(String::default())),
                extension: String::default(),
            }
        }

    // Helper sub-parsers.
    rule datetime_prefix() -> DateTime
        = digits:$( ['0'..='9']+ ) ['.'|'-'] { DateTime(digits.into()) }
    rule extra_tags() -> Vec<String>
        = "." tags:( tag() ** "." ) { tags }
    rule slug_with_tags() -> (String, Vec<String>)
        = words:( maybe_tag() ** "-" ) {
            let mut slug_words = Vec::new();
            let mut tags = Vec::new();
            for (is_tag, word) in words {
                if is_tag {
                    tags.push(word.clone())
                }
                slug_words.push(word);
            }
            (slug_words.join("-"), tags)
        }

    // Lowest-level "words" parsers.
    rule maybe_tag() -> (bool, String)
        = marker:"@"? w:word() { (marker.is_some(), w) }
    rule tag() -> String
        = "@" w:word() { w }
    rule word() -> String
        = slice:$( ['a'..='z']+ ) { slice.to_string() }
}}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn parse_draft_path_with_date() {
        let path = PathBuf::from("_drafts/2023090101-@foo-bar.@baz.md");
        let info = PathInfo::parse(&path).unwrap();
        assert_eq!(
            info,
            PathInfo {
                slug: "foo-bar".into(),
                datetime: DateTime("2023090101".to_string()),
                tags: ["_drafts", "baz", "foo"].map(String::from).to_vec(),
                extension: "md".to_string(),
            }
        );
    }
}
