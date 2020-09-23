// Copyright 2020 Farzad FARID <farzy@farzy.org>
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{fs, error};
use std::fs::{ File, DirBuilder };
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::io::Write;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[macro_use]
extern crate log;

const DATA_PREFIX: &str = "var galleriesBundleData = ";
const DATA_SUFFIX: &str = "</script>";

#[derive(Serialize, Deserialize, Debug)]
struct Photo {
    url: String,
    original_url: String,
    taken_at: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Data {
    photo_data: Vec<Photo>,
}

fn main() -> Result<(), Box<dyn Error>> {
    if std::env::args().len() != 2 {
        eprintln!("Usage: cheerz-dl URL");
        std::process::exit(1);
    }

    let url = std::env::args().skip(1).next().unwrap();

    let body = match read_from_url(&url) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Error downloading {}: {}", url, e);
            std::process::exit(2);
        }
    };

    let mut idx = body.find(DATA_PREFIX).unwrap();
    let mut content = &body[(idx + DATA_PREFIX.len())..];
    idx = content.find(DATA_SUFFIX).unwrap();
    content = &content[..idx];

    let data: Data = serde_json::from_str(content)?;

    DirBuilder::new().recursive(true).create("/tmp/cheerz")?;

    for photo in data.photo_data {
        println!("Photo: {:?}", photo);

        // let tmp_dir = Builder::new().prefix("cheerz").tempdir()?;
        let tmp_dir = std::path::Path::new("/tmp/cheerz");
        let target = photo.original_url;
        let response = reqwest::blocking::get(&target)?;

        let mut dest = {
            let fname = photo.taken_at;
            // Remove milliseconds
            let idx = fname.rfind(':').unwrap();
            let fname = format!("{}.jpg", &fname[..idx]);

            let fname = tmp_dir.join(fname);
            println!("will be located under: '{:?}'", fname);
            File::create(fname)?
        };
        let content = response.bytes()?;
        dest.write(&content)?;

        println!("{:?}", dest);
    }

    Ok(())
}

/// Read a text from from an URL and cache it in /var/tmp, return the body
///
///# Examples
///
/// ```
///
/// let body = read_from_url("https://httpbin.org/base64/SFRUUEJJTiBpcyBhd2Vzb21l").unwrap();
/// assert_eq!("HTTPBIN is awesome", body);
/// ```
fn read_from_url(url: &str) -> Result<String, Box<dyn error::Error>> {
    // Create filename for the file cache
    let mut hasher = DefaultHasher::new();
    url.hash(&mut hasher);
    let filename = format!("/var/tmp/cheerz-dl-{:x}.txt", hasher.finish());

    // Read file from the cache or Internet
    let body: String;
    if let Ok(body_from_file) = fs::read_to_string(&filename) {
        info!("Read text of {} from cache file {}", url, filename);
        body = body_from_file;
    } else {
        body = reqwest::blocking::get(url)?
            .text()?;
        info!("Write text from {} to cache file {}", url, filename);
        let mut f = fs::File::create(filename)?;
        f.write_all(body.as_bytes())?;
    }

    Ok(body)
}
