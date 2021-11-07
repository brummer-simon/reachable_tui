// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this
// file, You can obtain one at http://mozilla.org/MPL/2.0/.
//
// Author: Simon Brummer (simon.brummer@posteo.de)

use std::error::Error;
use std::path::Path;

pub mod config;
use config::{read_config_from_file, write_config_to_file, Config, Group, Host, Ui};

fn build_config() -> Config {
    let host1 = Host::new().set_fqhn("Host1");
    let host2 = Host::new().set_fqhn("Host2");
    let group1 = Group::new().add_host(host1).add_host(host2);

    let host3 = Host::new().set_fqhn("Host3");
    let host4 = Host::new().set_fqhn("Host4");
    let group2 = Group::new().set_name("Group1").add_host(host3).add_host(host4);

    let ui = Ui::new();
    Config::new().set_ui(ui).add_group(group1).add_group(group2)
}

fn main() -> Result<(), Box<dyn Error>> {
    write_config_to_file(&build_config(), &Path::new("/home/simon/.config/reachable_tui.yaml"))?;
    let config = read_config_from_file(&Path::new("/home/simon/.config/reachable_tui.yaml"))?;
    print!("{:?}", config);
    Ok(())
}
