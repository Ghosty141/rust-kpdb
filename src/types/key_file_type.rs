// Copyright (c) 2016-2017 Martijn Rijkeboer <mrr@sru-systems.com>
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

/// The type of the key file.
#[derive(Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum KeyFileType {
    /// Binary key file.
    Binary,

    /// Hexadecimal key file.
    Hex,

    /// XML key file.
    Xml,
}
