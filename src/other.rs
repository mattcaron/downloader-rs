// Copyright (c) 2021-2025 José Manuel Barroso Galindo <theypsilon@gmail.com>

// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

// You can download the latest version of this tool from:
// https://github.com/theypsilon/downloader-rs

//! A collection of utility functions.

use percent_encoding::{AsciiSet, NON_ALPHANUMERIC, percent_encode};

/// What characters should be percent encoded.
///
/// This dataset it designed to match the Python urllib implementation and
/// is therefore based on
/// https://docs.python.org/3/library/urllib.parse.html#url-quoting,
/// from urllib 3.13.3 which is, in turn, based on
/// https://datatracker.ietf.org/doc/html/rfc3986.html
///
/// In reality, this implementation primarily based of these two lines in the
/// doc description:
///
/// "Letters, digits, and the characters '_.-~' are never quoted."
///
/// "The optional safe parameter specifies additional ASCII characters that
/// should not be quoted — its default value is '/'"
///
/// Thus, the implementation is the NON_ALPHANUMERIC set with those specific
/// characters removed.
const PYTHON_ESCAPES: &AsciiSet = &NON_ALPHANUMERIC
    .remove(b'_')
    .remove(b'.')
    .remove(b'-')
    .remove(b'~')
    .remove(b'/');

/// Calculate the URL from given base URL and path.
///
/// Essentially, this concatenates `path` on to `base_files_url`, after
/// properly percent encoding `path`.
///
/// # Arguments
///
/// * `base_files_url` - Base URL to which `path` should be appended. No
///   manipulation of this URL is performed.
///
/// * `path` - Path to add to `base_files_url`. This is percent encoded.
///
/// # Returns
///
/// The concatenated value as a `String`, or `None` if an error occurred.
///
pub fn calculate_url(base_files_url: Option<&str>, path: &str) -> Option<String> {
    match base_files_url {
        Some(url) => {
            if url.trim().is_empty() {
                None
            } else {
                Some(url.to_owned() + &percent_encode(path.as_bytes(), PYTHON_ESCAPES).to_string())
            }
        }
        None => None,
    }
}

#[cfg(test)]
mod tests {
    use super::calculate_url;

    #[test]
    fn test_calculate_urls() {
        for (input_url, input_file_ctx, expected) in [
            (
                Some("https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/"),
                "Cheats/AtariLynx/A.P.B. (USA, Europe) [F6FB48FB].zip",
                Some(
                    "https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/Cheats/AtariLynx/A.P.B.%20%28USA%2C%20Europe%29%20%5BF6FB48FB%5D.zip",
                ),
            ),
            (
                Some("https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/"),
                "Cheats/NES/Lipstick #1 - Lolita Hen (Japan) (Unl) [30D9946C].zip",
                Some(
                    "https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/Cheats/NES/Lipstick%20%231%20-%20Lolita%20Hen%20%28Japan%29%20%28Unl%29%20%5B30D9946C%5D.zip",
                ),
            ),
            (
                Some("https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/"),
                "Cheats/SNES/Maerchen Adventure Cotton 100% (Japan) [5FB7A31D].zip",
                Some(
                    "https://raw.githubusercontent.com/MiSTer-devel/Distribution_MiSTer/main/Cheats/SNES/Maerchen%20Adventure%20Cotton%20100%25%20%28Japan%29%20%5B5FB7A31D%5D.zip",
                ),
            ),
            (
                Some("   "),
                "Cheats/SNES/Maerchen Adventure Cotton 100% (Japan) [5FB7A31D].zip",
                None,
            ),
            (
                None,
                "Cheats/SNES/Maerchen Adventure Cotton 100% (Japan) [5FB7A31D].zip",
                None,
            ),
        ] {
            assert_eq!(
                expected,
                calculate_url(input_url, input_file_ctx).as_deref()
            );
        }
    }
}
