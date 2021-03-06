// SPDX-License-Identifier: Apache-2.0

use crt0stack::Reader;

fn main() {
    let reader = Reader::from_environ().prev().prev();
    assert_eq!(reader.count(), std::env::args().count());

    let mut reader = reader.done();
    for arg in &mut reader {
        println!("arg: {:?}", arg);
    }

    let mut reader = reader.done();
    for env in &mut reader {
        println!("env: {:?}", env);
    }

    let mut reader = reader.done();
    for aux in &mut reader {
        println!("aux: {:?}", aux);
    }
}
