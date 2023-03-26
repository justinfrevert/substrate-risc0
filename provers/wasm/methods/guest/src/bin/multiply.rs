// Copyright 2023 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#![no_main]
#![no_std]

use risc0_zkvm::guest::env;
use wasmi::*;

pub extern crate externc_libm as libm;

risc0_zkvm::guest::entry!(main);

pub fn main() {
    // Load the number from the host
    let a: i32 = env::read();

    let engine = Engine::default();
    // Todo: think of where to use this
    // type HostState = u32;
    let mut linker = <Linker<()>>::new(&engine);
    let mut store = Store::new(&engine, ());

    let host_fn = Func::wrap(&mut store, |mut caller: Caller<()>, input: i32| -> i32 {
        let wasm_fn = caller
            .get_export("square")
            .and_then(Extern::into_func)
            .unwrap()
            .typed::<i32, i32>(&caller)
            .unwrap();
        wasm_fn.call(&mut caller, input).unwrap()
    });
    linker.define("env", "host_fn", host_fn).unwrap();
    let wasm = wat::parse_str(
        r#"
        (module
            (import "env" "host_fn" (func $host_fn (param i32) (result i32)))
            (func (export "wasm_fn") (param i32) (result i32)
                (call $host_fn (local.get 0))
            )
            (func (export "square") (param i32) (result i32)
                (i32.mul
                    (local.get 0)
                    (local.get 0)
                )
            )
        )
        "#,
    )
    .unwrap();
    let module = Module::new(store.engine(), &mut &wasm[..]).unwrap();
    let instance = linker
        .instantiate(&mut store, &module)
        .unwrap()
        .start(&mut store)
        .unwrap();
    let wasm_fn = instance
        .get_export(&store, "wasm_fn")
        .and_then(Extern::into_func)
        .unwrap()
        .typed::<i32, i32>(&store)
        .unwrap();
    let result = wasm_fn.call(&mut store, a).unwrap();
    // assert_eq!(result, expected);
    env::commit(&result);
}
