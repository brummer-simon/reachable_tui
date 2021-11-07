// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

use serde::{Deserialize, Serialize};
use serde_yaml::{self};
use std::error::Error;
use std::fs::File;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    ui: Ui,
    groups: Vec<Group>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            ui: Ui::new(),
            groups: vec![],
        }
    }

    pub fn set_ui(mut self, ui: Ui) -> Self {
        self.ui = ui;
        self
    }

    pub fn add_group(mut self, group: Group) -> Self {
        self.groups.push(group);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Ui {}

impl Ui {
    pub fn new() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Group {
    name: Option<String>,
    hosts: Vec<Host>,
}

impl Group {
    pub fn new() -> Self {
        Self {
            name: None,
            hosts: vec![],
        }
    }

    pub fn set_name(mut self, name: &str) -> Self {
        self.name = Some(String::from(name));
        self
    }

    pub fn add_host(mut self, host: Host) -> Self {
        self.hosts.push(host);
        self
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Host {
    fqhn: String,
    alias: Option<String>,
}

impl Host {
    pub fn new() -> Self {
        Self {
            fqhn: String::from(""),
            alias: None,
        }
    }

    pub fn set_fqhn(mut self, fqhn: &str) -> Self {
        self.fqhn = String::from(fqhn);
        self
    }

    pub fn set_alias(mut self, alias: &str) -> Self {
        self.alias = Some(String::from(alias));
        self
    }
}

pub fn write_config_to_file(config: &Config, path: &Path) -> Result<(), Box<dyn Error>> {
    serde_yaml::to_writer(File::create(path)?, config)?;
    Ok(())
}

pub fn read_config_from_file(path: &Path) -> Result<Config, Box<dyn Error>> {
    let config = serde_yaml::from_reader(File::open(path)?)?;
    // let as_config: Config = serde_yaml::from_str(&as_string)?;
    // println!("{:?}", as_config);
    Ok(config)
}
