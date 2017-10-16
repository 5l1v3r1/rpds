/* This file is part of rpds.
 *
 * rpds is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Lesser General Public License as published
 * by the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * rpds is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Lesser General Public License for more details.
 *
 * You should have received a copy of the GNU Lesser General Public License
 * along with rpds.  If not, see <http://www.gnu.org/licenses/>.
 */

#![cfg_attr(feature = "fatal-warnings", deny(warnings))]

#[macro_use]
extern crate bencher;

mod utils;

use utils::BencherNoDrop;
use bencher::{Bencher, black_box};

fn rust_vec_push(bench: &mut Bencher) -> () {
    let limit = 100_000;

    bench.iter_no_drop(|| {
        let mut vector: Vec<isize> = Vec::new();

        for i in 0..limit {
            vector.push(i);
        }

        vector
    });
}

// TODO implement rust_vec_pop in the same style as the test of `Vector::drop_last()` once we can
// do per-iteration initialization.

fn rust_vec_get(bench: &mut Bencher) -> () {
    let limit = 100_000;
    let mut vector: Vec<isize> = Vec::new();

    for i in 0..limit {
        vector.push(i);
    }

    bench.iter(|| {
        for i in 0..limit {
            black_box(vector.get(i as usize));
        }
    });
}

fn rust_vec_iterate(bench: &mut Bencher) -> () {
    let limit = 100_000;
    let mut vector: Vec<isize> = Vec::new();

    for i in 0..limit {
        vector.push(i);
    }

    bench.iter(|| {
        for i in vector.iter() {
            black_box(i);
        }
    });
}

benchmark_group!(benches, rust_vec_push, rust_vec_get, rust_vec_iterate);
benchmark_main!(benches);
