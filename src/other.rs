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

pub fn calculate_url(_base_files_url: Option<&str>, _path: &str) -> Option<String> {
    None
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
