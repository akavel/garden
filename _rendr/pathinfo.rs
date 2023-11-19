// This file is part of Garden Rendr.
// Copyright (C) 2023  Mateusz Czapliński
//
// Garden Rendr is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as
// published by the Free Software Foundation, either version 3 of the
// License, or (at your option) any later version.
//
// Garden Rendr is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

//! Article metadata extracted from a specially formatted Path.

use std::path::Path;

/// Date and time digits, presumed in big-endian order.
/// Precision unspecified (may be YYYYMMDD, or just YYYYMM, or YYYYMMDDHHMM, etc.).
// #[derive(Debug, PartialEq)]
pub type DateTime = String;

#[derive(Debug, PartialEq)]
pub struct PathInfo {
    pub slug: String,
    pub datetime: DateTime,
    pub tags: Vec<String>,
    pub extension: String,
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

        let extract_normal_component = |c| match c {
            std::path::Component::Normal(s) => Some(s),
            _ => None,
        };
        let mut directory_names: Vec<String> = path
            .components()
            .rev()
            .skip(1) // skip filename
            .filter_map(extract_normal_component)
            .map(|o| o.to_str().map(|s| s.trim_start_matches('@').to_string())) // OsStr -> Option<String>
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
                datetime: when.unwrap_or(String::default()),
                extension: String::default(),
            }
        }

    // Helper sub-parsers.
    rule datetime_prefix() -> DateTime
        = digits:digits()+ { digits.join("") }
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
    rule digits() -> String
        = slice:$( ['0'..='9']+ ) ['.'|'-'] { slice.to_string() }
}}

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    fn parse_path(path: &str) -> PathInfo {
        let path = PathBuf::from(path);
        PathInfo::parse(&path).unwrap()
    }

    #[test]
    fn parse_draft_path_with_date() {
        assert_eq!(
            parse_path("_drafts/2023090101-@foo-bar.@baz.md"),
            PathInfo {
                slug: "foo-bar".into(),
                datetime: "2023090101".to_string(),
                tags: ["_drafts", "baz", "foo"].map(String::from).to_vec(),
                extension: "md".to_string(),
            }
        );
    }

    #[test]
    fn parse_nondraft_path_no_date() {
        assert_eq!(
            parse_path("foo-@bar.@baz.md"),
            PathInfo {
                slug: "foo-bar".into(),
                datetime: String::default(),
                tags: ["bar", "baz"].map(String::from).to_vec(),
                extension: "md".to_string(),
            }
        );
    }

    #[test]
    fn parse_datetime_with_dashes() {
        assert_eq!(
            parse_path("20211022-001-@go-loose.md"),
            PathInfo {
                slug: "go-loose".into(),
                datetime: "20211022001".into(),
                tags: ["go"].map(String::from).to_vec(),
                extension: "md".into(),
            },
        );
    }
}
