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
        use std::path::Component;
        let utf8_dir_tags: Option<Vec<String>> = path
            .components()
            .rev()
            .skip(1)
            .filter_map(|c| match c {
                Component::Normal(s) => Some(s),
                _ => None,
            })
            .map(|s| s.to_str().map(String::from))
            .collect();
        let Some(dir_tags) = utf8_dir_tags else {
            return Err(Error::NotUTF8);
        };
        let Some(stem) = path.file_stem().unwrap_or_default().to_str() else {
            return Err(Error::NotUTF8);
        };
        let slug = stem.to_string(); // FIXME
        let datetime = DateTime(String::default()); // FIXME
        let tags = dir_tags; // FIXME
        let Some(extension) = path.extension().unwrap_or_default().to_str() else {
            return Err(Error::NotUTF8);
        };
        let extension = extension.to_string();
        Ok(PathInfo {
            slug,
            datetime,
            tags,
            extension,
        })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("file name is missing")]
    MissingFileName,
    #[error("path does not conform to UTF-8")]
    NotUTF8,
    #[error("bad file stem format")]
    BadStemFormat(#[from] peg::ParseError),
}

// TODO: this is very ad-hoc, and not necessarily covers everything in a sensible way. Let's start
// with whatever and tweak if needed, see how it evolves with time.
peg::parser!( grammar file_stem() for str {
    pub rule file_stem() -> PathInfo
        = (when:datetime() ['.' | '-'])? slug:slug_with_tags() extra:('.' extra_tags())? {
            let (slug, tags) = slug;
            tags.append(extra);
            PathInfo {
                slug, tags,
                datetime: when.unwrap_or(Datetime(String::default())),
                extension: String::default(),
            }
        }
    // pub rule parse() -> PathInfo =
    rule datetime() -> DateTime
        = digits:$(['0'..='9']+) { DateTime(digits.into()) }
    rule slug_with_tags() -> (String, Vec<String>)
        = words:(maybe_tag() ** '-') {
            let slug = words.iter().map(|t| t.1).collect().join("-");
            let tags = words.iter().filter_map(|t| t.0.then_some(t.1)).collect();
            (slug, tags)
        }
    rule extra_tags() -> Vec<String>
        = tags:(tag() ** '.') { tags }
    rule maybe_tag() -> (bool, String)
        = is_tag:'@'? w:word() = { (is_tag.is_some(), w.to_string()) }
    rule tag() -> String
        = '@' w:word() { w }
    rule word() -> String
        = slice:$(['a'..='z']) { slice.to_string() }
})

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
                tags: ["@baz".into(), "@foo".into(), "_drafts".into()].into(),
                extension: "md".to_string(),
            }
        );
    }
}
