hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo test --all --verbose
       Fresh version_check v0.9.5
       Fresh cfg-if v1.0.1
       Fresh typenum v1.18.0
       Fresh unicode-ident v1.0.18
       Fresh proc-macro2 v1.0.95
       Fresh generic-array v0.14.7
       Fresh libc v0.2.174
       Fresh quote v1.0.40
       Fresh crypto-common v0.1.6
       Fresh autocfg v1.5.0
       Fresh either v1.15.0
       Fresh block-buffer v0.10.4
       Fresh syn v2.0.104
       Fresh getrandom v0.2.16
       Fresh half v2.6.0
       Fresh anstyle v1.0.11
       Fresh regex-syntax v0.8.5
       Fresh memchr v2.7.5
       Fresh serde_derive v1.0.219
       Fresh crossbeam-utils v0.8.21
       Fresh zerocopy v0.8.26
       Fresh rand_core v0.6.4
       Fresh plotters-backend v0.3.7
       Fresh itoa v1.0.15
       Fresh ryu v1.0.20
       Fresh clap_lex v0.7.5
       Fresh ciborium-io v0.2.2
       Fresh regex-automata v0.4.9
       Fresh itertools v0.10.5
       Fresh serde v1.0.219
       Fresh crossbeam-epoch v0.9.18
       Fresh ppv-lite86 v0.2.21
       Fresh plotters-svg v0.3.7
       Fresh ciborium-ll v0.2.2
       Fresh clap_builder v4.5.43
       Fresh num-traits v0.2.19
       Fresh digest v0.10.7
       Fresh inout v0.1.4
       Fresh keccak v0.1.5
       Fresh same-file v1.0.6
       Fresh cast v0.3.0
       Fresh crossbeam-deque v0.8.6
       Fresh serde_json v1.0.142
       Fresh rand_chacha v0.3.1
       Fresh plotters v0.3.7
       Fresh walkdir v2.5.0
       Fresh sha3 v0.10.8
       Fresh criterion-plot v0.5.0
       Fresh cipher v0.4.4
       Fresh clap v4.5.43
       Fresh ciborium v0.2.2
       Fresh regex v1.11.1
       Fresh is-terminal v0.4.16
       Fresh rayon-core v1.12.1
       Fresh rand v0.8.5
       Fresh tinytemplate v1.2.1
       Fresh anes v0.1.6
       Fresh oorandom v11.1.5
       Fresh subtle v2.6.1
       Fresh once_cell v1.21.3
       Fresh cpufeatures v0.2.17
       Fresh hex v0.4.3
       Fresh rayon v1.10.0
       Fresh aes v0.8.4
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
       Fresh criterion v0.5.1
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name ntt_mul --edition=2021 tests/ntt_mul.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=0edc388491d2e2e9 -C extra-filename=-854b212d767152e6 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name ntt_poly --edition=2021 tests/ntt_poly.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=8c93e853123526dc -C extra-filename=-34d17e66a8564d35 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name api_std --edition=2021 tests/api_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=14178045b8ae0845 -C extra-filename=-7d684b4bb372ebb7 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name ctr_drbg --edition=2021 tests/ctr_drbg.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=f30c40d3225fb209 -C extra-filename=-11ee18d32a18faf0 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
warning: `rusty-kyber` (lib test) generated 6 warnings (6 duplicates)
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=b730cc46395d909f -C extra-filename=-a68764d5a2384914 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name xof_matrix --edition=2021 tests/xof_matrix.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=084d40c4230f2255 -C extra-filename=-56265ca8cc78659d --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
error: `#[panic_handler]` function required, but not found

error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

error: could not compile `rusty-kyber` (example "no_std") due to 2 previous errors

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="default"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=b730cc46395d909f -C extra-filename=-a68764d5a2384914 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-fd5133082f9dea63.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
warning: variable does not need to be mutable
  --> tests/ntt_poly.rs:45:13
   |
45 |         let mut a = rand_poly([s; 32]);
   |             ----^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `rusty-kyber` (test "ntt_poly") generated 1 warning (run `cargo fix --test "ntt_poly"` to apply 1 suggestion)

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo test --no-default-features --features "std,kyber768" --verbose
       Fresh version_check v0.9.5
       Fresh cfg-if v1.0.1
       Fresh typenum v1.18.0
       Fresh unicode-ident v1.0.18
       Fresh autocfg v1.5.0
       Fresh either v1.15.0
       Fresh generic-array v0.14.7
       Fresh proc-macro2 v1.0.95
       Fresh half v2.6.0
       Fresh clap_lex v0.7.5
       Fresh ciborium-io v0.2.2
       Fresh quote v1.0.40
       Fresh libc v0.2.174
       Fresh crypto-common v0.1.6
       Fresh crossbeam-utils v0.8.21
       Fresh zerocopy v0.8.26
       Fresh block-buffer v0.10.4
       Fresh ryu v1.0.20
       Fresh plotters-backend v0.3.7
       Fresh memchr v2.7.5
       Fresh regex-syntax v0.8.5
       Fresh anstyle v1.0.11
       Fresh syn v2.0.104
       Fresh crossbeam-epoch v0.9.18
       Fresh getrandom v0.2.16
       Fresh ppv-lite86 v0.2.21
       Fresh itoa v1.0.15
       Fresh digest v0.10.7
       Fresh regex-automata v0.4.9
       Fresh num-traits v0.2.19
       Fresh clap_builder v4.5.43
       Fresh plotters-svg v0.3.7
       Fresh inout v0.1.4
       Fresh ciborium-ll v0.2.2
       Fresh serde_derive v1.0.219
       Fresh rand_core v0.6.4
       Fresh crossbeam-deque v0.8.6
       Fresh itertools v0.10.5
       Fresh same-file v1.0.6
       Fresh keccak v0.1.5
       Fresh cast v0.3.0
       Fresh clap v4.5.43
       Fresh cipher v0.4.4
       Fresh regex v1.11.1
       Fresh plotters v0.3.7
       Fresh is-terminal v0.4.16
       Fresh serde v1.0.219
       Fresh rand_chacha v0.3.1
       Fresh rayon-core v1.12.1
       Fresh walkdir v2.5.0
       Fresh sha3 v0.10.8
       Fresh criterion-plot v0.5.0
       Fresh once_cell v1.21.3
       Fresh oorandom v11.1.5
       Fresh cpufeatures v0.2.17
       Fresh anes v0.1.6
       Fresh subtle v2.6.1
       Fresh hex v0.4.3
       Fresh serde_json v1.0.142
       Fresh rayon v1.10.0
       Fresh ciborium v0.2.2
       Fresh rand v0.8.5
       Fresh aes v0.8.4
       Fresh tinytemplate v1.2.1
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
       Fresh criterion v0.5.1
warning: variable does not need to be mutable
  --> tests/ntt_poly.rs:45:13
   |
45 |         let mut a = rand_poly([s; 32]);
   |             ----^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: `rusty-kyber` (test "ntt_poly") generated 1 warning (run `cargo fix --test "ntt_poly"` to apply 1 suggestion)
warning: `rusty-kyber` (lib test) generated 6 warnings (6 duplicates)
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name utils_hash --edition=2021 tests/utils_hash.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=c3dcc6b40f8c5be4 -C extra-filename=-f6f829b8766e4e13 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name xof_matrix --edition=2021 tests/xof_matrix.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=71ded72eeff46b14 -C extra-filename=-77b9829cef222258 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=85b897aaab2ad0d1 -C extra-filename=-dc9155e47612890b --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name hardening --edition=2021 tests/hardening.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=66192658a4f1da7e -C extra-filename=-ffb5147b5f30560d --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name cbd --edition=2021 tests/cbd.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=f0893c4f8c610236 -C extra-filename=-0e4b6fc9f26ddf2c --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name params_consistency --edition=2021 tests/params_consistency.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=c6969ae60c326e9e -C extra-filename=-b27c70c18123ad2b --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
error: `#[panic_handler]` function required, but not found

error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

error: could not compile `rusty-kyber` (example "no_std") due to 2 previous errors

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber768"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=85b897aaab2ad0d1 -C extra-filename=-dc9155e47612890b --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-3e8309ffbd43ccb7.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib` (exit status: 1)
warning: build failed, waiting for other jobs to finish...

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo test --no-default-features --features "std,kyber1024" --verbose
       Fresh version_check v0.9.5
       Fresh cfg-if v1.0.1
       Fresh unicode-ident v1.0.18
       Fresh autocfg v1.5.0
       Fresh either v1.15.0
       Fresh typenum v1.18.0
       Fresh half v2.6.0
       Fresh ryu v1.0.20
       Fresh ciborium-io v0.2.2
       Fresh proc-macro2 v1.0.95
       Fresh generic-array v0.14.7
       Fresh libc v0.2.174
       Fresh crossbeam-utils v0.8.21
       Fresh zerocopy v0.8.26
       Fresh memchr v2.7.5
       Fresh regex-syntax v0.8.5
       Fresh plotters-backend v0.3.7
       Fresh itoa v1.0.15
       Fresh clap_lex v0.7.5
       Fresh quote v1.0.40
       Fresh crypto-common v0.1.6
       Fresh crossbeam-epoch v0.9.18
       Fresh getrandom v0.2.16
       Fresh block-buffer v0.10.4
       Fresh ppv-lite86 v0.2.21
       Fresh anstyle v1.0.11
       Fresh regex-automata v0.4.9
       Fresh plotters-svg v0.3.7
       Fresh num-traits v0.2.19
       Fresh inout v0.1.4
       Fresh ciborium-ll v0.2.2
       Fresh syn v2.0.104
       Fresh crossbeam-deque v0.8.6
       Fresh rand_core v0.6.4
       Fresh clap_builder v4.5.43
       Fresh digest v0.10.7
       Fresh itertools v0.10.5
       Fresh cast v0.3.0
       Fresh same-file v1.0.6
       Fresh keccak v0.1.5
       Fresh plotters v0.3.7
       Fresh regex v1.11.1
       Fresh cipher v0.4.4
       Fresh serde_derive v1.0.219
       Fresh rand_chacha v0.3.1
       Fresh rayon-core v1.12.1
       Fresh clap v4.5.43
       Fresh walkdir v2.5.0
       Fresh sha3 v0.10.8
       Fresh criterion-plot v0.5.0
       Fresh is-terminal v0.4.16
       Fresh cpufeatures v0.2.17
       Fresh anes v0.1.6
       Fresh oorandom v11.1.5
       Fresh subtle v2.6.1
       Fresh serde v1.0.219
       Fresh rayon v1.10.0
       Fresh rand v0.8.5
       Fresh once_cell v1.21.3
       Fresh aes v0.8.4
       Fresh hex v0.4.3
       Fresh serde_json v1.0.142
       Fresh ciborium v0.2.2
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
       Fresh tinytemplate v1.2.1
       Fresh criterion v0.5.1
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: variable does not need to be mutable
  --> tests/ntt_poly.rs:45:13
   |
45 |         let mut a = rand_poly([s; 32]);
   |             ----^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: `rusty-kyber` (test "ntt_poly") generated 1 warning (run `cargo fix --test "ntt_poly"` to apply 1 suggestion)
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber1024"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=941e211f2d367349 -C extra-filename=-30a1e6c07c46fe9e --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-99d01c39ac072ef6.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib`
warning: `rusty-kyber` (lib test) generated 6 warnings (6 duplicates)
error: `#[panic_handler]` function required, but not found

error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

error: could not compile `rusty-kyber` (example "no_std") due to 2 previous errors

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber1024"' --cfg 'feature="std"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=941e211f2d367349 -C extra-filename=-30a1e6c07c46fe9e --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-99d01c39ac072ef6.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib` (exit status: 1)


---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo test --no-default-features --features "std,kyber512,serde,zeroize" --verbose
       Fresh version_check v0.9.5
       Fresh unicode-ident v1.0.18
       Fresh cfg-if v1.0.1
       Fresh proc-macro2 v1.0.95
       Fresh typenum v1.18.0
       Fresh autocfg v1.5.0
       Fresh either v1.15.0
       Fresh half v2.6.0
       Fresh generic-array v0.14.7
       Fresh quote v1.0.40
       Fresh libc v0.2.174
       Fresh ryu v1.0.20
       Fresh regex-syntax v0.8.5
       Fresh plotters-backend v0.3.7
       Fresh syn v2.0.104
       Fresh crossbeam-utils v0.8.21
       Fresh crypto-common v0.1.6
       Fresh zerocopy v0.8.26
       Fresh getrandom v0.2.16
       Fresh block-buffer v0.10.4
       Fresh ciborium-io v0.2.2
       Fresh anstyle v1.0.11
       Fresh itoa v1.0.15
       Fresh memchr v2.7.5
       Fresh clap_lex v0.7.5
       Fresh serde_derive v1.0.219
       Fresh crossbeam-epoch v0.9.18
       Fresh rand_core v0.6.4
       Fresh ppv-lite86 v0.2.21
       Fresh clap_builder v4.5.43
       Fresh ciborium-ll v0.2.2
       Fresh num-traits v0.2.19
       Fresh digest v0.10.7
       Fresh regex-automata v0.4.9
       Fresh inout v0.1.4
       Fresh plotters-svg v0.3.7
       Fresh itertools v0.10.5
       Fresh serde v1.0.219
       Fresh crossbeam-deque v0.8.6
       Fresh rand_chacha v0.3.1
       Fresh keccak v0.1.5
       Fresh same-file v1.0.6
       Fresh cast v0.3.0
       Fresh cipher v0.4.4
       Fresh clap v4.5.43
       Fresh regex v1.11.1
       Fresh plotters v0.3.7
       Fresh is-terminal v0.4.16
       Fresh subtle v2.6.1
       Fresh serde_json v1.0.142
       Fresh rayon-core v1.12.1
       Fresh sha3 v0.10.8
       Fresh criterion-plot v0.5.0
       Fresh ciborium v0.2.2
       Fresh rand v0.8.5
       Fresh walkdir v2.5.0
       Fresh cpufeatures v0.2.17
       Fresh oorandom v11.1.5
       Fresh zeroize v1.8.1
       Fresh once_cell v1.21.3
       Fresh anes v0.1.6
       Fresh tinytemplate v1.2.1
       Fresh rayon v1.10.0
       Fresh aes v0.8.4
       Fresh hex v0.4.3
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
       Fresh criterion v0.5.1
warning: variable does not need to be mutable
  --> tests/ntt_poly.rs:45:13
   |
45 |         let mut a = rand_poly([s; 32]);
   |             ----^
   |             |
   |             help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: `rusty-kyber` (test "ntt_poly") generated 1 warning (run `cargo fix --test "ntt_poly"` to apply 1 suggestion)
warning: `rusty-kyber` (lib test) generated 6 warnings (6 duplicates)
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=03a3401c473efc1f -C extra-filename=-8f5908c6be129c91 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name utils_msg --edition=2021 tests/utils_msg.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=3f2eebe627975005 -C extra-filename=-1d78ca65407e660e --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name kat --edition=2021 tests/kat.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=5724b644ce7b94da -C extra-filename=-2344d3ef9a0a8e35 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name ntt_mul --edition=2021 tests/ntt_mul.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=5edfd47fd2e4e893 -C extra-filename=-eea27208e199a569 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name hardening_ok --edition=2021 tests/hardening_ok.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=4decacbadbe85865 -C extra-filename=-a1621edeb06ec058 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name ctr_drbg --edition=2021 tests/ctr_drbg.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=64841a38e16d1c35 -C extra-filename=-e8c5e1a038780cfa --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
error: `#[panic_handler]` function required, but not found

error: unwinding panics are not supported without std
  |
  = help: using nightly cargo, use -Zbuild-std with panic="abort" to avoid unwinding
  = note: since the core library is usually precompiled with panic="unwind", rebuilding your crate with panic="abort" may not be enough to fix the problem

error: could not compile `rusty-kyber` (example "no_std") due to 2 previous errors

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name no_std --edition=2021 examples/no_std.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type bin --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="kyber512"' --cfg 'feature="serde"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=03a3401c473efc1f -C extra-filename=-8f5908c6be129c91 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/examples -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern rusty_kyber=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librusty_kyber-c7d6508a8fa0095b.rlib --extern serde=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde-8eb2796a3ce4ce30.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib` (exit status: 1)
warning: build failed, waiting for other jobs to finish...

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo test --no-default-features --features "std,kyber512,hardening" --verbose
       Fresh version_check v0.9.5
       Fresh cfg-if v1.0.1
       Fresh typenum v1.18.0
       Fresh unicode-ident v1.0.18
       Fresh autocfg v1.5.0
       Fresh either v1.15.0
       Fresh half v2.6.0
       Fresh proc-macro2 v1.0.95
       Fresh generic-array v0.14.7
       Fresh clap_lex v0.7.5
       Fresh memchr v2.7.5
       Fresh anstyle v1.0.11
       Fresh libc v0.2.174
       Fresh quote v1.0.40
       Fresh crypto-common v0.1.6
       Fresh crossbeam-utils v0.8.21
       Fresh zerocopy v0.8.26
       Fresh block-buffer v0.10.4
       Fresh ryu v1.0.20
       Fresh regex-syntax v0.8.5
       Fresh ciborium-io v0.2.2
       Fresh plotters-backend v0.3.7
       Fresh itoa v1.0.15
       Fresh syn v2.0.104
       Fresh getrandom v0.2.16
       Fresh crossbeam-epoch v0.9.18
       Fresh ppv-lite86 v0.2.21
       Fresh digest v0.10.7
       Fresh ciborium-ll v0.2.2
       Fresh plotters-svg v0.3.7
       Fresh regex-automata v0.4.9
       Fresh num-traits v0.2.19
       Fresh clap_builder v4.5.43
       Fresh inout v0.1.4
       Fresh itertools v0.10.5
       Fresh serde_derive v1.0.219
       Fresh rand_core v0.6.4
       Fresh crossbeam-deque v0.8.6
       Fresh keccak v0.1.5
       Fresh cast v0.3.0
       Fresh same-file v1.0.6
       Fresh cipher v0.4.4
       Fresh plotters v0.3.7
       Fresh clap v4.5.43
       Fresh regex v1.11.1
       Fresh is-terminal v0.4.16
       Fresh cpufeatures v0.2.17
       Fresh serde v1.0.219
       Fresh rayon-core v1.12.1
       Fresh rand_chacha v0.3.1
       Fresh walkdir v2.5.0
       Fresh sha3 v0.10.8
       Fresh criterion-plot v0.5.0
       Fresh anes v0.1.6
       Fresh subtle v2.6.1
       Fresh once_cell v1.21.3
       Fresh oorandom v11.1.5
       Fresh zeroize v1.8.1
       Fresh aes v0.8.4
       Fresh serde_json v1.0.142
       Fresh rayon v1.10.0
       Fresh ciborium v0.2.2
       Fresh rand v0.8.5
       Fresh hex v0.4.3
       Fresh tinytemplate v1.2.1
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
       Fresh criterion v0.5.1
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name rusty_kyber --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="hardening"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=4366e1989fa4066c -C extra-filename=-19574be2e94d194c --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rmeta --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rmeta --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rmeta --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rmeta --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rmeta`
     Running `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name rusty_kyber --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="hardening"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=19420b824a0ea1ca -C extra-filename=-fe419d8536cdf4c5 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib`
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

error[E0599]: no function or associated item named `conditional_select` found for type `u8` in the current scope
  --> src/hardening.rs:41:22
   |
41 |         out[i] = u8::conditional_select(&0u8, &ss1[i], equal);
   |                      ^^^^^^^^^^^^^^^^^^ function or associated item not found in `u8`
   |
   = help: items from traits can only be used if the trait is in scope
help: trait `ConditionallySelectable` which provides `conditional_select` is implemented but not in scope; perhaps you want to import it
   |
13 + use subtle::ConditionallySelectable;
   |
help: there is an associated function `conditional_swap` with a similar name
   |
41 -         out[i] = u8::conditional_select(&0u8, &ss1[i], equal);
41 +         out[i] = u8::conditional_swap(&0u8, &ss1[i], equal);
   |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

For more information about this error, try `rustc --explain E0599`.
warning: `rusty-kyber` (lib) generated 5 warnings
error: could not compile `rusty-kyber` (lib) due to 1 previous error; 5 warnings emitted

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name rusty_kyber --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --crate-type lib --emit=dep-info,metadata,link -C embed-bitcode=no -C debuginfo=2 --cfg 'feature="hardening"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=4366e1989fa4066c -C extra-filename=-19574be2e94d194c --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rmeta --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rmeta --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rmeta --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rmeta --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rmeta` (exit status: 1)
warning: build failed, waiting for other jobs to finish...
warning: `rusty-kyber` (lib test) generated 5 warnings (5 duplicates)
error: could not compile `rusty-kyber` (lib test) due to 1 previous error; 5 warnings emitted

Caused by:
  process didn't exit successfully: `/home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rustc --crate-name rusty_kyber --edition=2021 src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --diagnostic-width=199 --emit=dep-info,link -C embed-bitcode=no -C debuginfo=2 --test --cfg 'feature="hardening"' --cfg 'feature="kyber512"' --cfg 'feature="std"' --cfg 'feature="zeroize"' --check-cfg 'cfg(docsrs,test)' --check-cfg 'cfg(feature, values("default", "hardening", "kyber1024", "kyber512", "kyber768", "serde", "std", "zeroize"))' -C metadata=19420b824a0ea1ca -C extra-filename=-fe419d8536cdf4c5 --out-dir /home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps -C incremental=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/incremental -L dependency=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps --extern aes=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libaes-03ca09e1b59c6ffa.rlib --extern criterion=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libcriterion-fae703067a1b5239.rlib --extern hex=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libhex-20cdb627b9823e3e.rlib --extern rand=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand-28753af5a01ca4b3.rlib --extern rand_chacha=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_chacha-fd3228acab85f90c.rlib --extern rand_core=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/librand_core-ee41072387d09e54.rlib --extern serde_json=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libserde_json-12ea5b80ecda7678.rlib --extern sha3=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsha3-1c511d61627593c8.rlib --extern subtle=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libsubtle-90457403dad6ef76.rlib --extern zeroize=/home/hootzluh/Desktop/rust/rusty-kyber/target/debug/deps/libzeroize-172f0b1b1ddf513d.rlib` (exit status: 1)

  ---

  hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo install cargo-tarpaulin
cargo tarpaulin --out Lcov --timeout 600 --verbose --fail-under 90
    Updating crates.io index
  Downloaded cargo-tarpaulin v0.32.8
  Downloaded 1 crate (179.3KiB) in 0.47s
  Installing cargo-tarpaulin v0.32.8
    Updating crates.io index
     Locking 265 packages to latest compatible versions
      Adding cargo_metadata v0.20.0 (available: v0.21.0)
      Adding indexmap v1.8.2 (available: v1.9.3)
      Adding quick-xml v0.37.5 (available: v0.38.1)
      Adding toml v0.8.23 (available: v0.9.5)
  Downloaded camino v1.1.11
  Downloaded adler32 v1.2.0
  Downloaded cargo-platform v0.2.0
  Downloaded erased-serde v0.4.6
  Downloaded cargo_metadata v0.20.0
  Downloaded cargo-util-schemas v0.2.0
  Downloaded coveralls-api v0.7.0
  Downloaded typeid v1.0.3
  Downloaded serde-value v0.7.0
  Downloaded slab v0.4.11
  Downloaded humantime-serde v1.1.1
  Downloaded humantime v2.2.0
  Downloaded ordered-float v2.10.1
  Downloaded gzip-header v1.0.0
  Downloaded serde-untagged v0.1.7
  Downloaded leb128 v0.2.5
  Downloaded mime_guess v2.0.5
  Downloaded llvm_profparser v0.8.3
  Downloaded fallible-iterator v0.3.0
  Downloaded indexmap v1.8.2
  Downloaded procfs-core v0.17.0
  Downloaded deflate v1.0.0
  Downloaded procfs v0.17.0
  Downloaded ruzstd v0.8.1
  Downloaded zerovec v0.11.4
  Downloaded hashbrown v0.15.5
  Downloaded cc v1.2.32
  Downloaded quick-xml v0.37.5
  Downloaded object v0.26.2
  Downloaded git2 v0.20.2
  Downloaded gimli v0.32.0
  Downloaded object v0.37.2
  Downloaded libssh2-sys v0.3.1
  Downloaded libgit2-sys v0.18.2+1.9.1
  Downloaded 34 crates (4.6MiB) in 0.49s (largest was `libgit2-sys` at 1.8MiB)
   Compiling proc-macro2 v1.0.95
   Compiling unicode-ident v1.0.18
   Compiling libc v0.2.174
   Compiling serde v1.0.219
   Compiling shlex v1.3.0
   Compiling pkg-config v0.3.32
   Compiling memchr v2.7.5
   Compiling vcpkg v0.2.15
   Compiling stable_deref_trait v1.2.0
   Compiling cfg-if v1.0.1
   Compiling autocfg v1.5.0
   Compiling pin-project-lite v0.2.16
   Compiling smallvec v1.15.1
   Compiling writeable v0.6.1
   Compiling itoa v1.0.15
   Compiling once_cell v1.21.3
   Compiling bitflags v2.9.1
   Compiling litemap v0.8.0
   Compiling bytes v1.10.1
   Compiling icu_normalizer_data v2.0.0
   Compiling icu_properties_data v2.0.1
   Compiling futures-core v0.3.31
   Compiling num-traits v0.2.19
   Compiling crc32fast v1.5.0
   Compiling percent-encoding v2.3.1
   Compiling semver v1.0.26
   Compiling fnv v1.0.7
   Compiling futures-sink v0.3.31
   Compiling tracing-core v0.1.34
   Compiling futures-io v0.3.31
   Compiling pin-utils v0.1.0
   Compiling futures-task v0.3.31
   Compiling log v0.4.27
   Compiling slab v0.4.11
   Compiling form_urlencoded v1.2.1
   Compiling quote v1.0.40
   Compiling http v1.3.1
   Compiling futures-util v0.3.31
   Compiling foreign-types-shared v0.1.1
   Compiling utf8_iter v1.0.4
   Compiling openssl v0.10.73
   Compiling httparse v1.10.1
   Compiling foreign-types v0.3.2
   Compiling syn v2.0.104
   Compiling try-lock v0.2.5
   Compiling typeid v1.0.3
   Compiling openssl-probe v0.1.6
   Compiling native-tls v0.2.14
   Compiling equivalent v1.0.2
   Compiling tower-service v0.3.3
   Compiling adler2 v2.0.1
   Compiling hashbrown v0.15.5
   Compiling want v0.3.1
   Compiling futures-channel v0.3.31
   Compiling miniz_oxide v0.8.9
   Compiling thiserror v1.0.69
   Compiling unicase v2.8.1
   Compiling serde_json v1.0.142
   Compiling ryu v1.0.20
   Compiling mime_guess v2.0.5
   Compiling sync_wrapper v1.0.2
   Compiling thiserror v2.0.12
   Compiling tower-layer v0.3.3
   Compiling http-body v1.0.1
   Compiling utf8parse v0.2.2
   Compiling ipnet v2.11.0
   Compiling flate2 v1.1.2
   Compiling base64 v0.22.1
   Compiling indexmap v2.10.0
   Compiling toml_write v0.1.2
   Compiling jobserver v0.1.33
   Compiling mio v1.0.4
   Compiling socket2 v0.6.0
   Compiling cc v1.2.32
   Compiling winnow v0.7.12
   Compiling iana-time-zone v0.1.63
   Compiling http-body-util v0.1.3
   Compiling anstyle-parse v0.2.7
   Compiling ordered-float v2.10.1
   Compiling tokio v1.47.1
   Compiling chrono v0.4.41
   Compiling aho-corasick v1.1.3
   Compiling indexmap v1.8.2
   Compiling anstyle-query v1.1.4
   Compiling cfg_aliases v0.2.1
   Compiling rustix v0.38.44
   Compiling is_terminal_polyfill v1.70.1
   Compiling zeroize v1.8.1
   Compiling anstyle v1.0.11
   Compiling mime v0.3.17
   Compiling anyhow v1.0.98
   Compiling regex-syntax v0.8.5
   Compiling colorchoice v1.0.4
   Compiling regex-syntax v0.6.29
   Compiling camino v1.1.11
   Compiling iri-string v0.7.8
   Compiling anstream v0.6.20
   Compiling rustls-pki-types v1.12.0
   Compiling nix v0.30.1
   Compiling openssl-sys v0.9.109
   Compiling libz-sys v1.1.22
   Compiling libssh2-sys v0.3.1
   Compiling libgit2-sys v0.18.2+1.9.1
warning: openssl-sys@0.9.109: In file included from build/expando.c:1:
warning: openssl-sys@0.9.109: In file included from /usr/include/openssl/opensslv.h:109:
warning: openssl-sys@0.9.109: /usr/include/openssl/macros.h:14:10: fatal error: 'openssl/opensslconf.h' file not found
warning: openssl-sys@0.9.109:    14 | #include <openssl/opensslconf.h>
warning: openssl-sys@0.9.109:       |          ^~~~~~~~~~~~~~~~~~~~~~~
warning: openssl-sys@0.9.109: 1 error generated.
error: failed to run custom build command for `openssl-sys v0.9.109`

Caused by:
  process didn't exit successfully: `/tmp/cargo-installKNTDeZ/release/build/openssl-sys-30111ed088d8f8ed/build-script-main` (exit status: 101)
  --- stdout
  cargo:rustc-check-cfg=cfg(osslconf, values("OPENSSL_NO_OCB", "OPENSSL_NO_SM4", "OPENSSL_NO_SEED", "OPENSSL_NO_CHACHA", "OPENSSL_NO_CAST", "OPENSSL_NO_IDEA", "OPENSSL_NO_CAMELLIA", "OPENSSL_NO_RC4", "OPENSSL_NO_BF", "OPENSSL_NO_PSK", "OPENSSL_NO_DEPRECATED_3_0", "OPENSSL_NO_SCRYPT", "OPENSSL_NO_SM3", "OPENSSL_NO_RMD160", "OPENSSL_NO_EC2M", "OPENSSL_NO_OCSP", "OPENSSL_NO_CMS", "OPENSSL_NO_COMP", "OPENSSL_NO_SOCK", "OPENSSL_NO_STDIO", "OPENSSL_NO_EC", "OPENSSL_NO_SSL3_METHOD", "OPENSSL_NO_KRB5", "OPENSSL_NO_TLSEXT", "OPENSSL_NO_SRP", "OPENSSL_NO_RFC3779", "OPENSSL_NO_SHA", "OPENSSL_NO_NEXTPROTONEG", "OPENSSL_NO_ENGINE", "OPENSSL_NO_BUF_FREELISTS", "OPENSSL_NO_RC2"))
  cargo:rustc-check-cfg=cfg(openssl)
  cargo:rustc-check-cfg=cfg(libressl)
  cargo:rustc-check-cfg=cfg(boringssl)
  cargo:rustc-check-cfg=cfg(awslc)
  cargo:rustc-check-cfg=cfg(libressl250)
  cargo:rustc-check-cfg=cfg(libressl251)
  cargo:rustc-check-cfg=cfg(libressl252)
  cargo:rustc-check-cfg=cfg(libressl261)
  cargo:rustc-check-cfg=cfg(libressl270)
  cargo:rustc-check-cfg=cfg(libressl271)
  cargo:rustc-check-cfg=cfg(libressl273)
  cargo:rustc-check-cfg=cfg(libressl280)
  cargo:rustc-check-cfg=cfg(libressl281)
  cargo:rustc-check-cfg=cfg(libressl291)
  cargo:rustc-check-cfg=cfg(libressl310)
  cargo:rustc-check-cfg=cfg(libressl321)
  cargo:rustc-check-cfg=cfg(libressl332)
  cargo:rustc-check-cfg=cfg(libressl340)
  cargo:rustc-check-cfg=cfg(libressl350)
  cargo:rustc-check-cfg=cfg(libressl360)
  cargo:rustc-check-cfg=cfg(libressl361)
  cargo:rustc-check-cfg=cfg(libressl370)
  cargo:rustc-check-cfg=cfg(libressl380)
  cargo:rustc-check-cfg=cfg(libressl381)
  cargo:rustc-check-cfg=cfg(libressl382)
  cargo:rustc-check-cfg=cfg(libressl390)
  cargo:rustc-check-cfg=cfg(libressl400)
  cargo:rustc-check-cfg=cfg(libressl410)
  cargo:rustc-check-cfg=cfg(ossl101)
  cargo:rustc-check-cfg=cfg(ossl102)
  cargo:rustc-check-cfg=cfg(ossl102f)
  cargo:rustc-check-cfg=cfg(ossl102h)
  cargo:rustc-check-cfg=cfg(ossl110)
  cargo:rustc-check-cfg=cfg(ossl110f)
  cargo:rustc-check-cfg=cfg(ossl110g)
  cargo:rustc-check-cfg=cfg(ossl110h)
  cargo:rustc-check-cfg=cfg(ossl111)
  cargo:rustc-check-cfg=cfg(ossl111b)
  cargo:rustc-check-cfg=cfg(ossl111c)
  cargo:rustc-check-cfg=cfg(ossl111d)
  cargo:rustc-check-cfg=cfg(ossl300)
  cargo:rustc-check-cfg=cfg(ossl310)
  cargo:rustc-check-cfg=cfg(ossl320)
  cargo:rustc-check-cfg=cfg(ossl330)
  cargo:rustc-check-cfg=cfg(ossl340)
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_LIB_DIR
  OPENSSL_LIB_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_INCLUDE_DIR
  OPENSSL_INCLUDE_DIR unset
  cargo:rerun-if-env-changed=X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR
  X86_64_UNKNOWN_LINUX_GNU_OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_DIR
  OPENSSL_DIR unset
  cargo:rerun-if-env-changed=OPENSSL_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=OPENSSL_STATIC
  cargo:rerun-if-env-changed=OPENSSL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=SYSROOT
  cargo:rerun-if-env-changed=OPENSSL_STATIC
  cargo:rerun-if-env-changed=OPENSSL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rustc-link-lib=ssl
  cargo:rustc-link-lib=crypto
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=OPENSSL_STATIC
  cargo:rerun-if-env-changed=OPENSSL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-changed=build/expando.c
  OPT_LEVEL = Some(3)
  OUT_DIR = Some(/tmp/cargo-installKNTDeZ/release/build/openssl-sys-f5347dae32b10329/out)
  TARGET = Some(x86_64-unknown-linux-gnu)
  HOST = Some(x86_64-unknown-linux-gnu)
  cargo:rerun-if-env-changed=CC_x86_64-unknown-linux-gnu
  CC_x86_64-unknown-linux-gnu = None
  cargo:rerun-if-env-changed=CC_x86_64_unknown_linux_gnu
  CC_x86_64_unknown_linux_gnu = None
  cargo:rerun-if-env-changed=HOST_CC
  HOST_CC = None
  cargo:rerun-if-env-changed=CC
  CC = Some(/home/hootzluh/wasi-sdk-22.0/bin/clang)
  RUSTC_WRAPPER = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some(false)
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = Some(--target=wasm32-wasip1 --sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot)
  cargo:rerun-if-env-changed=CC_SHELL_ESCAPED_FLAGS
  CC_SHELL_ESCAPED_FLAGS = None
  cargo:rerun-if-env-changed=HOST_CFLAGS
  HOST_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_unknown_linux_gnu
  CFLAGS_x86_64_unknown_linux_gnu = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64-unknown-linux-gnu
  CFLAGS_x86_64-unknown-linux-gnu = None
  CARGO_ENCODED_RUSTFLAGS = Some()
  cargo:warning=In file included from build/expando.c:1:
  cargo:warning=In file included from /usr/include/openssl/opensslv.h:109:
  cargo:warning=/usr/include/openssl/macros.h:14:10: fatal error: 'openssl/opensslconf.h' file not found
  cargo:warning=   14 | #include <openssl/opensslconf.h>
  cargo:warning=      |          ^~~~~~~~~~~~~~~~~~~~~~~
  cargo:warning=1 error generated.

  --- stderr

  thread 'main' panicked at /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/openssl-sys-0.9.109/build/main.rs:315:13:

  Header expansion error:
  Error { kind: ToolExecError, message: "command did not execute successfully (status code exit status: 1): LC_ALL=\"C\" \"/home/hootzluh/wasi-sdk-22.0/bin/clang\" \"-O3\" \"-ffunction-sections\" \"-fdata-sections\" \"-fPIC\" \"-m64\" \"--target=x86_64-unknown-linux-gnu\" \"-I\" \"/usr/include\" \"--target=wasm32-wasip1\" \"--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot\" \"-E\" \"build/expando.c\"" }

  Failed to find OpenSSL development headers.

  You can try fixing this setting the `OPENSSL_DIR` environment variable
  pointing to your OpenSSL installation or installing OpenSSL headers package
  specific to your distribution:

      # On Ubuntu
      sudo apt-get install pkg-config libssl-dev
      # On Arch Linux
      sudo pacman -S pkgconf openssl
      # On Fedora
      sudo dnf install pkgconf perl-FindBin perl-IPC-Cmd openssl-devel
      # On Alpine Linux
      apk add pkgconf openssl-dev

  See rust-openssl documentation for more information:

      https://docs.rs/openssl

  note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
warning: build failed, waiting for other jobs to finish...
warning: libz-sys@1.1.22: In file included from src/zlib/adler32.c:8:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-adler32.o" "-c" "src/zlib/adler32.c"cargo:warning=In file included from src/zlib/compress.c:9:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-compress.o" "-c" "src/zlib/compress.c"cargo:warning=In file included from src/zlib/crc32.c:30:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-crc32.o" "-c" "src/zlib/crc32.c"cargo:warning=In file included from src/zlib/deflate.c:52:
warning: libz-sys@1.1.22: In file included from src/zlib/deflate.h:16:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-deflate.o" "-c" "src/zlib/deflate.c"cargo:warning=In file included from src/zlib/infback.c:13:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-infback.o" "-c" "src/zlib/infback.c"cargo:warning=In file included from src/zlib/inffast.c:6:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inffast.o" "-c" "src/zlib/inffast.c"cargo:warning=In file included from src/zlib/inflate.c:83:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inflate.o" "-c" "src/zlib/inflate.c"cargo:warning=In file included from src/zlib/inftrees.c:6:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inftrees.o" "-c" "src/zlib/inftrees.c"cargo:warning=In file included from src/zlib/trees.c:37:
warning: libz-sys@1.1.22: In file included from src/zlib/deflate.h:16:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-trees.o" "-c" "src/zlib/trees.c"cargo:warning=In file included from src/zlib/uncompr.c:9:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-uncompr.o" "-c" "src/zlib/uncompr.c"cargo:warning=In file included from src/zlib/zutil.c:8:
warning: libz-sys@1.1.22: In file included from src/zlib/zutil.h:22:
warning: libz-sys@1.1.22: In file included from src/zlib/zlib.h:34:
warning: libz-sys@1.1.22: src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
warning: libz-sys@1.1.22:   446 | #    include <sys/types.h>      /* for off_t */
warning: libz-sys@1.1.22:       |              ^~~~~~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-zutil.o" "-c" "src/zlib/zutil.c"cargo:warning=In file included from src/zlib/gzclose.c:6:
warning: libz-sys@1.1.22: src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
warning: libz-sys@1.1.22:    20 | #include <stdio.h>
warning: libz-sys@1.1.22:       |          ^~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzclose.o" "-c" "src/zlib/gzclose.c"cargo:warning=In file included from src/zlib/gzlib.c:6:
warning: libz-sys@1.1.22: src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
warning: libz-sys@1.1.22:    20 | #include <stdio.h>
warning: libz-sys@1.1.22:       |          ^~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzlib.o" "-c" "src/zlib/gzlib.c"cargo:warning=In file included from src/zlib/gzread.c:6:
warning: libz-sys@1.1.22: src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
warning: libz-sys@1.1.22:    20 | #include <stdio.h>
warning: libz-sys@1.1.22:       |          ^~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzread.o" "-c" "src/zlib/gzread.c"cargo:warning=In file included from src/zlib/gzwrite.c:6:
warning: libz-sys@1.1.22: src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
warning: libz-sys@1.1.22:    20 | #include <stdio.h>
warning: libz-sys@1.1.22:       |          ^~~~~~~~~
warning: libz-sys@1.1.22: 1 error generated.
warning: libz-sys@1.1.22: ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzwrite.o" "-c" "src/zlib/gzwrite.c"
error: failed to run custom build command for `libz-sys v1.1.22`

Caused by:
  process didn't exit successfully: `/tmp/cargo-installKNTDeZ/release/build/libz-sys-98c1a447e8fa7322/build-script-build` (exit status: 1)
  --- stdout
  cargo:rerun-if-env-changed=LIBZ_SYS_STATIC
  cargo:rerun-if-changed=build.rs
  cargo:rerun-if-changed=zng/cmake.rs
  cargo:rerun-if-changed=zng/cc.rs
  cargo:rerun-if-env-changed=ZLIB_NO_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=ZLIB_STATIC
  cargo:rerun-if-env-changed=ZLIB_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=SYSROOT
  cargo:rerun-if-env-changed=ZLIB_STATIC
  cargo:rerun-if-env-changed=ZLIB_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rustc-link-lib=z
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG
  cargo:rerun-if-env-changed=PKG_CONFIG
  cargo:rerun-if-env-changed=ZLIB_STATIC
  cargo:rerun-if-env-changed=ZLIB_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_STATIC
  cargo:rerun-if-env-changed=PKG_CONFIG_ALL_DYNAMIC
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_PATH
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_LIBDIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64-unknown-linux-gnu
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR_x86_64_unknown_linux_gnu
  cargo:rerun-if-env-changed=HOST_PKG_CONFIG_SYSROOT_DIR
  cargo:rerun-if-env-changed=PKG_CONFIG_SYSROOT_DIR
  cargo:include=/usr/include
  OPT_LEVEL = Some(3)
  OUT_DIR = Some(/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out)
  TARGET = Some(x86_64-unknown-linux-gnu)
  HOST = Some(x86_64-unknown-linux-gnu)
  cargo:rerun-if-env-changed=CC_x86_64-unknown-linux-gnu
  CC_x86_64-unknown-linux-gnu = None
  cargo:rerun-if-env-changed=CC_x86_64_unknown_linux_gnu
  CC_x86_64_unknown_linux_gnu = None
  cargo:rerun-if-env-changed=HOST_CC
  HOST_CC = None
  cargo:rerun-if-env-changed=CC
  CC = Some(/home/hootzluh/wasi-sdk-22.0/bin/clang)
  RUSTC_WRAPPER = None
  cargo:rerun-if-env-changed=CC_ENABLE_DEBUG_OUTPUT
  cargo:rerun-if-env-changed=CRATE_CC_NO_DEFAULTS
  CRATE_CC_NO_DEFAULTS = None
  DEBUG = Some(false)
  cargo:rerun-if-env-changed=CFLAGS
  CFLAGS = Some(--target=wasm32-wasip1 --sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot)
  cargo:rerun-if-env-changed=CC_SHELL_ESCAPED_FLAGS
  CC_SHELL_ESCAPED_FLAGS = None
  cargo:rerun-if-env-changed=HOST_CFLAGS
  HOST_CFLAGS = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64_unknown_linux_gnu
  CFLAGS_x86_64_unknown_linux_gnu = None
  cargo:rerun-if-env-changed=CFLAGS_x86_64-unknown-linux-gnu
  CFLAGS_x86_64-unknown-linux-gnu = None
  CARGO_ENCODED_RUSTFLAGS = Some()
  running LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "src/smoke.c" "-g0" "-o" "/dev/null" "-lz"
  cargo:warning=In file included from src/zlib/adler32.c:8:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.
  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-adler32.o" "-c" "src/zlib/adler32.c"cargo:warning=In file included from src/zlib/compress.c:9:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-compress.o" "-c" "src/zlib/compress.c"cargo:warning=In file included from src/zlib/crc32.c:30:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-crc32.o" "-c" "src/zlib/crc32.c"cargo:warning=In file included from src/zlib/deflate.c:52:
  cargo:warning=In file included from src/zlib/deflate.h:16:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-deflate.o" "-c" "src/zlib/deflate.c"cargo:warning=In file included from src/zlib/infback.c:13:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-infback.o" "-c" "src/zlib/infback.c"cargo:warning=In file included from src/zlib/inffast.c:6:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inffast.o" "-c" "src/zlib/inffast.c"cargo:warning=In file included from src/zlib/inflate.c:83:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inflate.o" "-c" "src/zlib/inflate.c"cargo:warning=In file included from src/zlib/inftrees.c:6:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-inftrees.o" "-c" "src/zlib/inftrees.c"cargo:warning=In file included from src/zlib/trees.c:37:
  cargo:warning=In file included from src/zlib/deflate.h:16:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-trees.o" "-c" "src/zlib/trees.c"cargo:warning=In file included from src/zlib/uncompr.c:9:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-uncompr.o" "-c" "src/zlib/uncompr.c"cargo:warning=In file included from src/zlib/zutil.c:8:
  cargo:warning=In file included from src/zlib/zutil.h:22:
  cargo:warning=In file included from src/zlib/zlib.h:34:
  cargo:warning=src/zlib/zconf.h:446:14: fatal error: 'sys/types.h' file not found
  cargo:warning=  446 | #    include <sys/types.h>      /* for off_t */
  cargo:warning=      |              ^~~~~~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-zutil.o" "-c" "src/zlib/zutil.c"cargo:warning=In file included from src/zlib/gzclose.c:6:
  cargo:warning=src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
  cargo:warning=   20 | #include <stdio.h>
  cargo:warning=      |          ^~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzclose.o" "-c" "src/zlib/gzclose.c"cargo:warning=In file included from src/zlib/gzlib.c:6:
  cargo:warning=src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
  cargo:warning=   20 | #include <stdio.h>
  cargo:warning=      |          ^~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzlib.o" "-c" "src/zlib/gzlib.c"cargo:warning=In file included from src/zlib/gzread.c:6:
  cargo:warning=src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
  cargo:warning=   20 | #include <stdio.h>
  cargo:warning=      |          ^~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzread.o" "-c" "src/zlib/gzread.c"cargo:warning=In file included from src/zlib/gzwrite.c:6:
  cargo:warning=src/zlib/gzguts.h:20:10: fatal error: 'stdio.h' file not found
  cargo:warning=   20 | #include <stdio.h>
  cargo:warning=      |          ^~~~~~~~~
  cargo:warning=1 error generated.

  exit status: 1
  cargo:warning=ToolExecError: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzwrite.o" "-c" "src/zlib/gzwrite.c"

  --- stderr
  src/smoke.c:1:10: fatal error: 'zlib.h' file not found
      1 | #include <zlib.h>
        |          ^~~~~~~~
  1 error generated.


  error occurred in cc-rs: command did not execute successfully (status code exit status: 1): LC_ALL="C" "/home/hootzluh/wasi-sdk-22.0/bin/clang" "-O3" "-ffunction-sections" "-fdata-sections" "-fPIC" "-m64" "--target=x86_64-unknown-linux-gnu" "-I" "src/zlib" "-fvisibility=hidden" "-DSTDC" "-D_LARGEFILE64_SOURCE" "-D_POSIX_SOURCE" "--target=wasm32-wasip1" "--sysroot=/home/hootzluh/wasi-sdk-22.0/share/wasi-sysroot" "-o" "/tmp/cargo-installKNTDeZ/release/build/libz-sys-a6816aa495b27910/out/lib/f0389296f42960e9-gzwrite.o" "-c" "src/zlib/gzwrite.c"


error: failed to compile `cargo-tarpaulin v0.32.8`, intermediate artifacts can be found at `/tmp/cargo-installKNTDeZ`.
To reuse those artifacts with a future compilation, set the environment variable `CARGO_TARGET_DIR` to that path.
error: no such command: `tarpaulin`

help: view all installed commands with `cargo --list`
help: find a package to install `tarpaulin` with `cargo search cargo-tarpaulin`
hootzluh@synergy:~/Desktop/rust/rusty-kyber$

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ rustup toolchain install nightly
cargo +nightly install cargo-fuzz

cargo +nightly fuzz run pack_unpack -- -max_total_time=60
cargo +nightly fuzz run kem_roundtrip -- -max_total_time=60
warn: downloading with complete profile isn't recommended unless you are a developer of the rust language
info: syncing channel updates for 'nightly-x86_64-unknown-linux-gnu'
info: latest update on 2025-08-09, rust version 1.91.0-nightly (de3efa79f 2025-08-08)
info: downloading component 'rust-src'
info: downloading component 'rust-std' for 'wasm32-wasip1'
 21.4 MiB /  21.4 MiB (100 %)   9.8 MiB/s in  2s
info: downloading component 'rust-std' for 'wasm32-unknown-unknown'
info: downloading component 'cargo'
info: downloading component 'clippy'
info: downloading component 'rust-docs'
info: downloading component 'rust-std'
 29.9 MiB /  29.9 MiB (100 %)  29.0 MiB/s in  1s
info: downloading component 'rustc'
 79.6 MiB /  79.6 MiB (100 %)  42.7 MiB/s in  1s
info: downloading component 'rustfmt'
info: removing previous version of component 'rust-src'
info: removing previous version of component 'rust-std' for 'wasm32-wasip1'
info: removing previous version of component 'rust-std' for 'wasm32-unknown-unknown'
info: removing previous version of component 'cargo'
info: removing previous version of component 'clippy'
info: removing previous version of component 'rust-docs'
info: removing previous version of component 'rust-std'
info: removing previous version of component 'rustc'
info: removing previous version of component 'rustfmt'
info: installing component 'rust-src'
info: installing component 'rust-std' for 'wasm32-wasip1'
 21.4 MiB /  21.4 MiB (100 %)  19.6 MiB/s in  1s
info: installing component 'rust-std' for 'wasm32-unknown-unknown'
 20.2 MiB /  20.2 MiB (100 %)  19.6 MiB/s in  1s
info: installing component 'cargo'
info: installing component 'clippy'
info: installing component 'rust-docs'
 20.5 MiB /  20.5 MiB (100 %)  10.9 MiB/s in  1s
info: installing component 'rust-std'
 29.9 MiB /  29.9 MiB (100 %)  18.7 MiB/s in  1s
info: installing component 'rustc'
 79.6 MiB /  79.6 MiB (100 %)  19.4 MiB/s in  4s
info: installing component 'rustfmt'

  nightly-x86_64-unknown-linux-gnu updated - rustc 1.91.0-nightly (de3efa79f 2025-08-08) (from rustc 1.91.0-nightly (f34ba774c 2025-08-03))

info: checking for self-update
    Updating crates.io index
  Downloaded cargo-fuzz v0.13.1
  Downloaded 1 crate (39.0KiB) in 0.21s
  Installing cargo-fuzz v0.13.1
    Updating crates.io index
     Locking 69 packages to latest compatible versions
      Adding cargo_metadata v0.18.1 (available: v0.21.0)
      Adding toml v0.5.11 (available: v0.9.5)
  Downloaded current_platform v0.2.0
  Downloaded 1 crate (6.5KiB) in 0.14s
   Compiling proc-macro2 v1.0.95
   Compiling unicode-ident v1.0.18
   Compiling serde v1.0.219
   Compiling utf8parse v0.2.2
   Compiling libc v0.2.174
   Compiling semver v1.0.26
   Compiling getrandom v0.3.3
   Compiling colorchoice v1.0.4
   Compiling is_terminal_polyfill v1.70.1
   Compiling camino v1.1.11
   Compiling rustix v1.0.8
   Compiling anstyle v1.0.11
   Compiling anstyle-parse v0.2.7
   Compiling serde_json v1.0.142
   Compiling thiserror v1.0.69
   Compiling anstyle-query v1.1.4
   Compiling memchr v2.7.5
   Compiling current_platform v0.2.0
   Compiling strsim v0.11.1
   Compiling anstream v0.6.20
   Compiling clap_lex v0.7.5
   Compiling bitflags v2.9.1
   Compiling heck v0.5.0
   Compiling ryu v1.0.20
   Compiling anyhow v1.0.98
   Compiling linux-raw-sys v0.9.4
   Compiling cfg-if v1.0.1
   Compiling itoa v1.0.15
   Compiling fastrand v2.3.0
   Compiling clap_builder v4.5.43
   Compiling once_cell v1.21.3
   Compiling quote v1.0.40
   Compiling syn v2.0.104
   Compiling tempfile v3.20.0
   Compiling serde_derive v1.0.219
   Compiling thiserror-impl v1.0.69
   Compiling clap_derive v4.5.41
   Compiling clap v4.5.43
   Compiling cargo-platform v0.1.9
   Compiling toml v0.5.11
   Compiling rustc_version v0.4.1
   Compiling cargo_metadata v0.18.1
   Compiling cargo-fuzz v0.13.1
    Finished `release` profile [optimized] target(s) in 14.47s
  Installing /home/hootzluh/.cargo/bin/cargo-fuzz
   Installed package `cargo-fuzz v0.13.1` (executable `cargo-fuzz`)
    Updating crates.io index
     Locking 38 packages to latest compatible versions
      Adding rand v0.8.5 (available: v0.9.2)
      Adding rand_chacha v0.3.1 (available: v0.9.0)
   Compiling version_check v0.9.5
   Compiling typenum v1.18.0
   Compiling libc v0.2.174
   Compiling zerocopy v0.8.26
   Compiling cfg-if v1.0.1
   Compiling shlex v1.3.0
   Compiling keccak v0.1.5
   Compiling subtle v2.6.1
   Compiling arbitrary v1.4.1
   Compiling generic-array v0.14.7
   Compiling getrandom v0.2.16
   Compiling jobserver v0.1.33
   Compiling rand_core v0.6.4
   Compiling cc v1.2.32
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling digest v0.10.7
   Compiling sha3 v0.10.8
   Compiling ppv-lite86 v0.2.21
   Compiling libfuzzer-sys v0.4.10
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
   Compiling rusty-kyber-fuzz v0.0.1 (/home/hootzluh/Desktop/rust/rusty-kyber/fuzz)
    Finished `release` profile [optimized + debuginfo] target(s) in 19.66s
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.03s
     Running `fuzz/target/x86_64-unknown-linux-gnu/release/pack_unpack -artifact_prefix=/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/ -max_total_time=60 /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/corpus/pack_unpack`
INFO: Running with entropic power schedule (0xFF, 100).
INFO: Seed: 1301145052
INFO: Loaded 1 modules   (546 inline 8-bit counters): 546 [0x60131001f2f0, 0x60131001f512),
INFO: Loaded 1 PC tables (546 PCs): 546 [0x60131001f518,0x601310021738),
INFO:        0 files found in /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/corpus/pack_unpack
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes

thread '<unnamed>' (44589) panicked at /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:163:9:
index out of bounds: the len is 384 but the index is 384
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
==44589== ERROR: libFuzzer: deadly signal
    #0 0x60130ff510d1 in __sanitizer_print_stack_trace /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_stack.cpp:87:3
    #1 0x60130ffd669d in fuzzer::PrintStackTrace() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerUtil.cpp:210:38
    #2 0x60130ffba579 in fuzzer::Fuzzer::CrashCallback() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:231:18
    #3 0x60130ffba579 in fuzzer::Fuzzer::CrashCallback() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:226:6
    #4 0x73094424532f  (/lib/x86_64-linux-gnu/libc.so.6+0x4532f) (BuildId: 282c2c16e7b6600b0b22ea0c99010d2795752b5f)
    #5 0x73094429eb2b in __pthread_kill_implementation nptl/pthread_kill.c:43:17
    #6 0x73094429eb2b in __pthread_kill_internal nptl/pthread_kill.c:78:10
    #7 0x73094429eb2b in pthread_kill nptl/pthread_kill.c:89:10
    #8 0x73094424527d in raise signal/../sysdeps/posix/raise.c:26:13
    #9 0x7309442288fe in abort stdlib/abort.c:79:7
    #10 0x60130fff7c59 in std::sys::pal::unix::abort_internal::h9a0f3564407b4864 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/sys/pal/unix/mod.rs:366:14
    #11 0x60130fff5108 in std::process::abort::h66b558a4824612ff /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/process.rs:2499:5
    #12 0x60130ff7da84 in libfuzzer_sys::initialize::_$u7b$$u7b$closure$u7d$$u7d$::h9ad23e0e786d09e8 /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:94:9
    #13 0x60130fff71bd in _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..Fn$LT$Args$GT$$GT$::call::h0466b704ebca9b03 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/alloc/src/boxed.rs:1985:9
    #14 0x60130fff71bd in std::panicking::panic_with_hook::heae2ed06d2446c5d /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:842:13
    #15 0x60130fff6f89 in std::panicking::panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::he8f50a4285b5d007 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:707:13
    #16 0x60130fff5b68 in std::sys::backtrace::__rust_end_short_backtrace::heb2e8724623d343f /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/sys/backtrace.rs:174:18
    #17 0x60130fff6c1c in __rustc::rust_begin_unwind /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:698:5
    #18 0x60131000e8df in core::panicking::panic_fmt::hbf8b46eb2d03ea4d /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/core/src/panicking.rs:75:14
    #19 0x60131000ea60 in core::panicking::panic_bounds_check::h3529d13a4cd4366f /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/core/src/panicking.rs:280:5
    #20 0x60130ff7b638 in poly_to_bytes /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:163:9
    #21 0x60130ff7b638 in pack_unpack::_::__libfuzzer_sys_run::h745f540ce1b0cb34 /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/fuzz_targets/pack_unpack.rs:29:5
    #22 0x60130ff7af59 in rust_fuzzer_test_input /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:276:60
    #23 0x60130ff7c095 in {closure#0} /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:62:9
    #24 0x60130ff7c095 in std::panicking::catch_unwind::do_call::hf75e1e465f694b2f /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:590:40
    #25 0x60130ff7dae8 in __rust_try libfuzzer_sys.94ac673e9cbb1ab3-cgu.0
    #26 0x60130ff7d44d in catch_unwind<i32, libfuzzer_sys::test_input_wrap::{closure_env#0}> /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:553:19
    #27 0x60130ff7d44d in catch_unwind<libfuzzer_sys::test_input_wrap::{closure_env#0}, i32> /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:357:14
    #28 0x60130ff7d44d in LLVMFuzzerTestOneInput /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:60:22
    #29 0x60130ffbaad8 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:619:15
    #30 0x60130ffc2da2 in fuzzer::Fuzzer::ReadAndExecuteSeedCorpora(std::vector<fuzzer::SizedFile, std::allocator<fuzzer::SizedFile>>&) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:812:18
    #31 0x60130ffc34a9 in fuzzer::Fuzzer::Loop(std::vector<fuzzer::SizedFile, std::allocator<fuzzer::SizedFile>>&) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:872:28
    #32 0x60130ff905b1 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerDriver.cpp:915:10
    #33 0x60130ff7db16 in main /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerMain.cpp:20:30
    #34 0x73094422a1c9 in __libc_start_call_main csu/../sysdeps/nptl/libc_start_call_main.h:58:16
    #35 0x73094422a28a in __libc_start_main csu/../csu/libc-start.c:360:3
    #36 0x60130febcd44 in _start (/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/target/x86_64-unknown-linux-gnu/release/pack_unpack+0x5dd44) (BuildId: 512699a196dff17bb8b79072bbafd05ad757eb34)

NOTE: libFuzzer has rudimentary signal handlers.
      Combine libFuzzer with AddressSanitizer or similar for better crash reports.
SUMMARY: libFuzzer: deadly signal
MS: 0 ; base unit: 0000000000000000000000000000000000000000


artifact_prefix='/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/'; Test unit written to /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709
Base64:

────────────────────────────────────────────────────────────────────────────────

Failing input:

        fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

Output of `std::fmt::Debug`:

        []

Reproduce with:

        cargo fuzz run pack_unpack fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

Minimize test case with:

        cargo fuzz tmin pack_unpack fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

────────────────────────────────────────────────────────────────────────────────

Error: Fuzz target exited with exit status: 77
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
   Compiling rusty-kyber-fuzz v0.0.1 (/home/hootzluh/Desktop/rust/rusty-kyber/fuzz)
warning: unused import: `RngCore`
 --> fuzz_targets/kem_roundtrip.rs:4:12
  |
4 | use rand::{RngCore, SeedableRng};
  |            ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0277]: the trait bound `Ciphertext: From<[u8; 768]>` is not satisfied
  --> fuzz_targets/kem_roundtrip.rs:33:42
   |
33 |         let _ = decapsulate(&sk, &ct_bad.into());
   |                                          ^^^^ the trait `From<[u8; 768]>` is not implemented for `Ciphertext`
   |
   = note: required for `[u8; 768]` to implement `Into<Ciphertext>`

For more information about this error, try `rustc --explain E0277`.
warning: `rusty-kyber-fuzz` (bin "kem_roundtrip") generated 1 warning
error: could not compile `rusty-kyber-fuzz` (bin "kem_roundtrip") due to 1 previous error; 1 warning emitted
Error: failed to build fuzz script: ASAN_OPTIONS="detect_odr_violation=0" RUSTFLAGS=" -Cpasses=sancov-module -Cllvm-args=-sanitizer-coverage-level=4 -Cllvm-args=-sanitizer-coverage-inline-8bit-counters -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-trace-compares --cfg fuzzing -Cllvm-args=-simplifycfg-branch-fold-threshold=0 -Zsanitizer=address -Cllvm-args=-sanitizer-coverage-stack-depth -Cdebug-assertions -Ccodegen-units=1" "cargo" "build" "--manifest-path" "/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/Cargo.toml" "--target" "x86_64-unknown-linux-gnu" "--release" "--config" "profile.release.debug=\"line-tables-only\"" "--bin" "kem_roundtrip"

---


hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo +nightly fuzz run pack_unpack -- -max_total_time=1800
cargo +nightly fuzz run kem_roundtrip -- -max_total_time=1800
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.01s
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `release` profile [optimized + debuginfo] target(s) in 0.01s
     Running `fuzz/target/x86_64-unknown-linux-gnu/release/pack_unpack -artifact_prefix=/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/ -max_total_time=1800 /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/corpus/pack_unpack`
INFO: Running with entropic power schedule (0xFF, 100).
INFO: Seed: 3958292565
INFO: Loaded 1 modules   (546 inline 8-bit counters): 546 [0x59b7aa62e2f0, 0x59b7aa62e512),
INFO: Loaded 1 PC tables (546 PCs): 546 [0x59b7aa62e518,0x59b7aa630738),
INFO:        0 files found in /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/corpus/pack_unpack
INFO: -max_len is not provided; libFuzzer will not generate inputs larger than 4096 bytes

thread '<unnamed>' (45267) panicked at /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:163:9:
index out of bounds: the len is 384 but the index is 384
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
==45267== ERROR: libFuzzer: deadly signal
    #0 0x59b7aa5600d1 in __sanitizer_print_stack_trace /rustc/llvm/src/llvm-project/compiler-rt/lib/asan/asan_stack.cpp:87:3
    #1 0x59b7aa5e569d in fuzzer::PrintStackTrace() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerUtil.cpp:210:38
    #2 0x59b7aa5c9579 in fuzzer::Fuzzer::CrashCallback() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:231:18
    #3 0x59b7aa5c9579 in fuzzer::Fuzzer::CrashCallback() /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:226:6
    #4 0x7df74324532f  (/lib/x86_64-linux-gnu/libc.so.6+0x4532f) (BuildId: 282c2c16e7b6600b0b22ea0c99010d2795752b5f)
    #5 0x7df74329eb2b in __pthread_kill_implementation nptl/pthread_kill.c:43:17
    #6 0x7df74329eb2b in __pthread_kill_internal nptl/pthread_kill.c:78:10
    #7 0x7df74329eb2b in pthread_kill nptl/pthread_kill.c:89:10
    #8 0x7df74324527d in raise signal/../sysdeps/posix/raise.c:26:13
    #9 0x7df7432288fe in abort stdlib/abort.c:79:7
    #10 0x59b7aa606c59 in std::sys::pal::unix::abort_internal::h9a0f3564407b4864 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/sys/pal/unix/mod.rs:366:14
    #11 0x59b7aa604108 in std::process::abort::h66b558a4824612ff /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/process.rs:2499:5
    #12 0x59b7aa58ca84 in libfuzzer_sys::initialize::_$u7b$$u7b$closure$u7d$$u7d$::h9ad23e0e786d09e8 /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:94:9
    #13 0x59b7aa6061bd in _$LT$alloc..boxed..Box$LT$F$C$A$GT$$u20$as$u20$core..ops..function..Fn$LT$Args$GT$$GT$::call::h0466b704ebca9b03 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/alloc/src/boxed.rs:1985:9
    #14 0x59b7aa6061bd in std::panicking::panic_with_hook::heae2ed06d2446c5d /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:842:13
    #15 0x59b7aa605f89 in std::panicking::panic_handler::_$u7b$$u7b$closure$u7d$$u7d$::he8f50a4285b5d007 /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:707:13
    #16 0x59b7aa604b68 in std::sys::backtrace::__rust_end_short_backtrace::heb2e8724623d343f /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/sys/backtrace.rs:174:18
    #17 0x59b7aa605c1c in __rustc::rust_begin_unwind /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/std/src/panicking.rs:698:5
    #18 0x59b7aa61d8df in core::panicking::panic_fmt::hbf8b46eb2d03ea4d /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/core/src/panicking.rs:75:14
    #19 0x59b7aa61da60 in core::panicking::panic_bounds_check::h3529d13a4cd4366f /rustc/de3efa79f95852c7427587f1d535bfea7c0d6779/library/core/src/panicking.rs:280:5
    #20 0x59b7aa58a638 in poly_to_bytes /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:163:9
    #21 0x59b7aa58a638 in pack_unpack::_::__libfuzzer_sys_run::h745f540ce1b0cb34 /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/fuzz_targets/pack_unpack.rs:29:5
    #22 0x59b7aa589f59 in rust_fuzzer_test_input /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:276:60
    #23 0x59b7aa58b095 in {closure#0} /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:62:9
    #24 0x59b7aa58b095 in std::panicking::catch_unwind::do_call::hf75e1e465f694b2f /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:590:40
    #25 0x59b7aa58cae8 in __rust_try libfuzzer_sys.94ac673e9cbb1ab3-cgu.0
    #26 0x59b7aa58c44d in catch_unwind<i32, libfuzzer_sys::test_input_wrap::{closure_env#0}> /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panicking.rs:553:19
    #27 0x59b7aa58c44d in catch_unwind<libfuzzer_sys::test_input_wrap::{closure_env#0}, i32> /home/hootzluh/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/lib/rustlib/src/rust/library/std/src/panic.rs:357:14
    #28 0x59b7aa58c44d in LLVMFuzzerTestOneInput /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/src/lib.rs:60:22
    #29 0x59b7aa5c9ad8 in fuzzer::Fuzzer::ExecuteCallback(unsigned char const*, unsigned long) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:619:15
    #30 0x59b7aa5d1da2 in fuzzer::Fuzzer::ReadAndExecuteSeedCorpora(std::vector<fuzzer::SizedFile, std::allocator<fuzzer::SizedFile>>&) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:812:18
    #31 0x59b7aa5d24a9 in fuzzer::Fuzzer::Loop(std::vector<fuzzer::SizedFile, std::allocator<fuzzer::SizedFile>>&) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerLoop.cpp:872:28
    #32 0x59b7aa59f5b1 in fuzzer::FuzzerDriver(int*, char***, int (*)(unsigned char const*, unsigned long)) /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerDriver.cpp:915:10
    #33 0x59b7aa58cb16 in main /home/hootzluh/.cargo/registry/src/index.crates.io-1949cf8c6b5b557f/libfuzzer-sys-0.4.10/libfuzzer/FuzzerMain.cpp:20:30
    #34 0x7df74322a1c9 in __libc_start_call_main csu/../sysdeps/nptl/libc_start_call_main.h:58:16
    #35 0x7df74322a28a in __libc_start_main csu/../csu/libc-start.c:360:3
    #36 0x59b7aa4cbd44 in _start (/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/target/x86_64-unknown-linux-gnu/release/pack_unpack+0x5dd44) (BuildId: 512699a196dff17bb8b79072bbafd05ad757eb34)

NOTE: libFuzzer has rudimentary signal handlers.
      Combine libFuzzer with AddressSanitizer or similar for better crash reports.
SUMMARY: libFuzzer: deadly signal
MS: 0 ; base unit: 0000000000000000000000000000000000000000


artifact_prefix='/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/'; Test unit written to /home/hootzluh/Desktop/rust/rusty-kyber/fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709
Base64:

────────────────────────────────────────────────────────────────────────────────

Failing input:

        fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

Output of `std::fmt::Debug`:

        []

Reproduce with:

        cargo fuzz run pack_unpack fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

Minimize test case with:

        cargo fuzz tmin pack_unpack fuzz/artifacts/pack_unpack/crash-da39a3ee5e6b4b0d3255bfef95601890afd80709

────────────────────────────────────────────────────────────────────────────────

Error: Fuzz target exited with exit status: 77
warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
   Compiling rusty-kyber-fuzz v0.0.1 (/home/hootzluh/Desktop/rust/rusty-kyber/fuzz)
warning: unused import: `RngCore`
 --> fuzz_targets/kem_roundtrip.rs:4:12
  |
4 | use rand::{RngCore, SeedableRng};
  |            ^^^^^^^
  |
  = note: `#[warn(unused_imports)]` on by default

error[E0277]: the trait bound `Ciphertext: From<[u8; 768]>` is not satisfied
  --> fuzz_targets/kem_roundtrip.rs:33:42
   |
33 |         let _ = decapsulate(&sk, &ct_bad.into());
   |                                          ^^^^ the trait `From<[u8; 768]>` is not implemented for `Ciphertext`
   |
   = note: required for `[u8; 768]` to implement `Into<Ciphertext>`

For more information about this error, try `rustc --explain E0277`.
warning: `rusty-kyber-fuzz` (bin "kem_roundtrip") generated 1 warning
error: could not compile `rusty-kyber-fuzz` (bin "kem_roundtrip") due to 1 previous error; 1 warning emitted
Error: failed to build fuzz script: ASAN_OPTIONS="detect_odr_violation=0" RUSTFLAGS=" -Cpasses=sancov-module -Cllvm-args=-sanitizer-coverage-level=4 -Cllvm-args=-sanitizer-coverage-inline-8bit-counters -Cllvm-args=-sanitizer-coverage-pc-table -Cllvm-args=-sanitizer-coverage-trace-compares --cfg fuzzing -Cllvm-args=-simplifycfg-branch-fold-threshold=0 -Zsanitizer=address -Cllvm-args=-sanitizer-coverage-stack-depth -Cdebug-assertions -Ccodegen-units=1" "cargo" "build" "--manifest-path" "/home/hootzluh/Desktop/rust/rusty-kyber/fuzz/Cargo.toml" "--target" "x86_64-unknown-linux-gnu" "--release" "--config" "profile.release.debug=\"line-tables-only\"" "--bin" "kem_roundtrip"

---


hootzluh@synergy:~/Desktop/rust/rusty-kyber$ # no_std check for kyber512
cargo check --no-default-features --features kyber512

# wasm32 build (no_std)
rustup target add wasm32-unknown-unknown
cargo build --target wasm32-unknown-unknown --no-default-features --features kyber512

# ARM64 cross build + runtime smoke (requires Docker)
cargo install cross --git https://github.com/cross-rs/cross
cross build --target aarch64-unknown-linux-gnu
cross run   --target aarch64-unknown-linux-gnu --release --example arm64_smoke --features "std,kyber512"
    Checking rand_core v0.6.4
    Checking rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.26s
info: component 'rust-std' for target 'wasm32-unknown-unknown' is up to date
   Compiling version_check v0.9.5
   Compiling typenum v1.18.0
   Compiling keccak v0.1.5
   Compiling subtle v2.6.1
   Compiling rand_core v0.6.4
   Compiling generic-array v0.14.7
   Compiling crypto-common v0.1.6
   Compiling block-buffer v0.10.4
   Compiling digest v0.10.7
   Compiling sha3 v0.10.8
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 1.62s
    Updating git repository `https://github.com/cross-rs/cross`
    Updating git submodule `https://github.com/cross-rs/cross-toolchains.git`
  Installing cross v0.2.5 (https://github.com/cross-rs/cross#e281947c)
    Updating crates.io index
     Locking 112 packages to latest compatible versions
      Adding const-sha1 v0.2.0 (available: v0.3.0)
      Adding directories v4.0.1 (available: v6.0.0)
      Adding nix v0.26.4 (available: v0.30.1)
      Adding owo-colors v3.5.0 (available: v4.2.2)
      Adding thiserror v1.0.69 (available: v2.0.12)
      Adding toml v0.7.8 (available: v0.9.5)
      Adding which v4.4.2 (available: v8.0.0)
  Downloaded owo-colors v3.5.0
  Downloaded const-sha1 v0.2.0
  Downloaded is_ci v1.2.0
  Downloaded supports-color v1.3.1
  Downloaded indenter v0.3.4
  Downloaded signal-hook v0.3.18
  Downloaded directories v4.0.1
  Downloaded nix v0.26.4
  Downloaded 8 crates (401.1KiB) in 0.46s
   Compiling proc-macro2 v1.0.95
   Compiling libc v0.2.174
   Compiling unicode-ident v1.0.18
   Compiling serde v1.0.219
   Compiling cfg-if v1.0.1
   Compiling utf8parse v0.2.2
   Compiling memchr v2.7.5
   Compiling object v0.36.7
   Compiling bitflags v2.9.1
   Compiling colorchoice v1.0.4
   Compiling equivalent v1.0.2
   Compiling is_terminal_polyfill v1.70.1
   Compiling rustix v0.38.44
   Compiling anstyle-parse v0.2.7
   Compiling semver v1.0.26
   Compiling anstyle-query v1.1.4
   Compiling once_cell v1.21.3
   Compiling hashbrown v0.15.5
   Compiling gimli v0.31.1
   Compiling eyre v0.6.12
   Compiling getrandom v0.3.3
   Compiling anstyle v1.0.11
   Compiling rustix v1.0.8
   Compiling adler2 v2.0.1
   Compiling owo-colors v4.2.2
   Compiling miniz_oxide v0.8.9
   Compiling linux-raw-sys v0.4.15
   Compiling anstream v0.6.20
   Compiling serde_json v1.0.142
   Compiling is_ci v1.2.0
   Compiling thiserror v1.0.69
   Compiling clap_lex v0.7.5
   Compiling indenter v0.3.4
   Compiling quote v1.0.40
   Compiling strsim v0.11.1
   Compiling heck v0.5.0
   Compiling indexmap v2.10.0
   Compiling winnow v0.5.40
   Compiling linux-raw-sys v0.9.4
   Compiling signal-hook v0.3.18
   Compiling rustc-demangle v0.1.26
   Compiling syn v2.0.104
   Compiling clap_builder v4.5.43
   Compiling ryu v1.0.20
   Compiling itoa v1.0.15
   Compiling either v1.15.0
   Compiling cross v0.2.5 (/home/hootzluh/.cargo/git/checkouts/cross-f0189a1dc141e2d9/e281947)
   Compiling fastrand v2.3.0
   Compiling atty v0.2.14
   Compiling supports-color v1.3.1
   Compiling dirs-sys v0.3.7
   Compiling signal-hook-registry v1.4.6
   Compiling home v0.5.11
   Compiling bitflags v1.3.2
   Compiling nix v0.26.4
   Compiling directories v4.0.1
   Compiling owo-colors v3.5.0
   Compiling addr2line v0.24.2
   Compiling is-terminal v0.4.16
   Compiling rustc_version v0.4.1
   Compiling const-sha1 v0.2.0
   Compiling shell-escape v0.1.5
   Compiling which v4.4.2
   Compiling shell-words v1.1.0
   Compiling tempfile v3.20.0
   Compiling serde_derive v1.0.219
   Compiling thiserror-impl v1.0.69
   Compiling clap_derive v4.5.41
   Compiling backtrace v0.3.75
   Compiling color-eyre v0.6.5
   Compiling clap v4.5.43
   Compiling toml_datetime v0.6.11
   Compiling serde_spanned v0.6.9
   Compiling serde_ignored v0.1.12
   Compiling toml_edit v0.19.15
   Compiling toml v0.7.8
warning: creating a shared reference to mutable static
   --> src/docker/local.rs:168:34
    |
168 |     let is_terminated = unsafe { crate::errors::TERMINATED.load(Ordering::SeqCst) };
    |                                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives
    = note: `#[warn(static_mut_refs)]` on by default

warning: creating a shared reference to mutable static
   --> src/docker/shared.rs:573:17
    |
573 |             if !CHILD_CONTAINER.exists.swap(true, Ordering::SeqCst) {
    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a shared reference to mutable static
   --> src/docker/shared.rs:597:18
    |
597 |         unsafe { CHILD_CONTAINER.exists() }
    |                  ^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a mutable reference to mutable static
   --> src/docker/shared.rs:609:13
    |
609 |             CHILD_CONTAINER.exit();
    |             ^^^^^^^^^^^^^^^^^^^^^^ mutable reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
   --> src/docker/shared.rs:635:13
    |
635 |             CHILD_CONTAINER.finish(is_tty, msg_info);
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a shared reference to mutable static
  --> src/errors.rs:27:9
   |
27 |     if !TERMINATED.swap(true, Ordering::SeqCst) && temp::has_tempfiles() {
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a mutable reference to mutable static
   --> src/errors.rs:105:5
    |
105 |     docker::CHILD_CONTAINER.terminate();
    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ mutable reference to mutable static
    |
    = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
    = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a shared reference to mutable static
  --> src/temp.rs:22:15
   |
22 |     unsafe { !FILES.is_empty() || !DIRS.is_empty() }
   |               ^^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a shared reference to mutable static
  --> src/temp.rs:22:36
   |
22 |     unsafe { !FILES.is_empty() || !DIRS.is_empty() }
   |                                    ^^^^^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:31:5
   |
31 |     FILES.clear();
   |     ^^^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:32:5
   |
32 |     DIRS.clear();
   |     ^^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:41:5
   |
41 |     FILES.push(file);
   |     ^^^^^^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:42:8
   |
42 |     Ok(FILES.last_mut().expect("file list should not be empty"))
   |        ^^^^^^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:48:5
   |
48 |     FILES.pop()
   |     ^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:90:5
   |
90 |     DIRS.push(dir);
   |     ^^^^^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: creating a shared reference to mutable static
  --> src/temp.rs:91:8
   |
91 |     Ok(DIRS.last().expect("should not be empty").path())
   |        ^^^^^^^^^^^ shared reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: shared references to mutable statics are dangerous; it's undefined behavior if the static is mutated or if a mutable reference is created for it while the shared reference lives

warning: creating a mutable reference to mutable static
  --> src/temp.rs:97:5
   |
97 |     DIRS.pop()
   |     ^^^^^^^^^^ mutable reference to mutable static
   |
   = note: for more information, see <https://doc.rust-lang.org/edition-guide/rust-2024/static-mut-references.html>
   = note: mutable references to mutable statics are dangerous; it's undefined behavior if any other pointer to the static is used or if any other reference is created for the static while the mutable reference lives

warning: `cross` (lib) generated 17 warnings
    Finished `release` profile [optimized] target(s) in 39.82s
  Installing /home/hootzluh/.cargo/bin/cross
  Installing /home/hootzluh/.cargo/bin/cross-util
   Installed package `cross v0.2.5 (https://github.com/cross-rs/cross#e281947c)` (executables `cross`, `cross-util`)
Error:
   0: no container engine found

Location:
   src/docker/engine.rs:82

Suggestion: is docker or podman installed?
Error:
   0: no container engine found

Location:
   src/docker/engine.rs:82

Suggestion: is docker or podman installed?

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo fmt --all -- --check
cargo clippy --all-targets -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
cargo package --allow-dirty # verify packaging works (uses Cargo.toml exclude list)
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/benches/benchmark.rs:1:
+
+
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/benches/ntt.rs:56:
     group.bench_function("inv_ntt_single_loop", |b| {
         let base = {
             let mut v = make_polys();
-            for p in v.iter_mut() { p.ntt(); }
+            for p in v.iter_mut() {
+                p.ntt();
+            }
             v
         };
         b.iter_batched(
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/benches/ntt.rs:75:
     group.bench_function("inv_ntt_batched", |b| {
         let base = {
             let mut v = make_polys();
-            for p in v.iter_mut() { p.ntt(); }
+            for p in v.iter_mut() {
+                p.ntt();
+            }
             v
         };
         b.iter_batched(
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/api.rs:20:
 #![allow(clippy::needless_pass_by_value)]

 use crate::kem;
-use crate::params::{
-    CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES,
-};
+use crate::params::{CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES};

 use core::fmt;

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/decaps.rs:1:
-use crate::params::{K, SECRET_KEY_BYTES, CIPHERTEXT_BYTES, POLY_BYTES, DU, DV};
+use crate::params::{CIPHERTEXT_BYTES, DU, DV, K, POLY_BYTES, SECRET_KEY_BYTES};
 use crate::poly::Poly;
-use crate::utils::{poly_from_bytes, poly_decompress_u, poly_decompress_v, poly_to_msg};
+use crate::utils::{poly_decompress_u, poly_decompress_v, poly_from_bytes, poly_to_msg};

-pub fn indcpa_dec(
-    sk: &[u8; SECRET_KEY_BYTES],
-    ct: &[u8; CIPHERTEXT_BYTES],
-    msg: &mut [u8; 32],
-) {
+pub fn indcpa_dec(sk: &[u8; SECRET_KEY_BYTES], ct: &[u8; CIPHERTEXT_BYTES], msg: &mut [u8; 32]) {
     // Unpack indcpa secret vector s (first K*POLY_BYTES of sk)
     let mut s = [Poly::new(); K];
     for i in 0..K {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/encaps.rs:1:
-use crate::params::{
-    K, N, PUBLIC_KEY_BYTES, CIPHERTEXT_BYTES, POLY_BYTES, DU, DV, ETA1, ETA2,
-};
+use crate::params::{CIPHERTEXT_BYTES, DU, DV, ETA1, ETA2, K, N, POLY_BYTES, PUBLIC_KEY_BYTES};
 use crate::poly::Poly;
 use crate::utils::{
-    xof_matrix, prf, cbd_eta, rej_uniform, poly_from_bytes, poly_from_msg,
-    poly_compress_u, poly_compress_v,
+    cbd_eta, poly_compress_u, poly_compress_v, poly_from_bytes, poly_from_msg, prf, rej_uniform,
+    xof_matrix,
 };

 pub fn indcpa_enc(
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/hardening.rs:10:
 //! - Double-execution defends against fault/glitch injection that flips a bit
 //!   during decapsulation. Mismatch yields an all-zero shared secret.

-use subtle::{ConstantTimeEq, Choice};
+use subtle::{Choice, ConstantTimeEq};
 #[cfg(feature = "zeroize")]
 use zeroize::Zeroizing;

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/kem.rs:1:
-use crate::encaps::indcpa_enc;
 use crate::decaps::indcpa_dec;
+use crate::encaps::indcpa_enc;
 use crate::keygen::indcpa_keypair;
 use crate::params::{
     CIPHERTEXT_BYTES, K, POLY_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES,
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/keygen.rs:1:
-use crate::params::{K, N, ETA1, POLY_BYTES, PUBLIC_KEY_BYTES};
+use crate::params::{ETA1, K, N, POLY_BYTES, PUBLIC_KEY_BYTES};
 use crate::poly::Poly;
-use crate::utils::{g, prf, cbd_eta, rej_uniform, xof_matrix, poly_to_bytes};
+use crate::utils::{cbd_eta, g, poly_to_bytes, prf, rej_uniform, xof_matrix};

 use rand_core::{CryptoRng, RngCore};

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/lib.rs:12:
 //! - **Optional**: `serde` (serialize public types), `zeroize` (zeroize `SecretKey` on drop),
 //!   `hardening` (redundant constant-time checks and buffer scrubbing)

-pub mod params;
+pub mod api;
+pub mod decaps;
+pub mod encaps;
+pub mod kem;
+pub mod keygen;
 pub mod ntt;
+pub mod params;
 pub mod poly;
 pub mod utils;
-pub mod keygen;
-pub mod encaps;
-pub mod decaps;
-pub mod kem;
-pub mod api;

 #[cfg(feature = "hardening")]
 #[cfg_attr(docsrs, doc(cfg(feature = "hardening")))]
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/ntt.rs:82:

 /// Kyber zetas (twiddle factors) for the NTT.
 pub const ZETAS: [i16; 128] = [
-    -1044, -758,  -359, -1517, 1493, 1422, 287,  202,  -171, 622,  1577, 182,  962,  -1202, -1474,
-    1468,  573,   -1325, 264,  383,  -829, 1458, -1602, -130, -681, 1017, 732,  608,  -1542, 411,
-    -205,  -1571, 1223, 652,  -552, 1015, -1293, 1491, -282, -1544, 516,  -8,   -320, -666,  -1618,
-    -1162, 126,   1469, -853, -90,  -271, 830,  107,  -1421, -247, -951, -398, 961,  -1508, -725,
-    448,   -1065, 677,  -1275, -1103, 430,  555,  843,  -1251, 871,  1550, 105,  422,  587,   177,
-    -235,  -291,  -460, 1574, 1653, -246,  778,  1159, -147,  -777, 1483, -602, 1119, -1590, 644,
-    -872,  349,   418,  329,  -156, -75,   817,  1097, 603,  610,  1322, -1285, -1465, 384,   -1215,
-    -136,  1218,  -1335, -874, 220,  -1187, -1659, -1185, -1530, -1278, 794,  -1510, -854,  -870,
-    478,   -108,  -308, 996,  991,  958,  -1460, 1522, 1628,
+    -1044, -758, -359, -1517, 1493, 1422, 287, 202, -171, 622, 1577, 182, 962, -1202, -1474, 1468,
+    573, -1325, 264, 383, -829, 1458, -1602, -130, -681, 1017, 732, 608, -1542, 411, -205, -1571,
+    1223, 652, -552, 1015, -1293, 1491, -282, -1544, 516, -8, -320, -666, -1618, -1162, 126, 1469,
+    -853, -90, -271, 830, 107, -1421, -247, -951, -398, 961, -1508, -725, 448, -1065, 677, -1275,
+    -1103, 430, 555, 843, -1251, 871, 1550, 105, 422, 587, 177, -235, -291, -460, 1574, 1653, -246,
+    778, 1159, -147, -777, 1483, -602, 1119, -1590, 644, -872, 349, 418, 329, -156, -75, 817, 1097,
+    603, 610, 1322, -1285, -1465, 384, -1215, -136, 1218, -1335, -874, 220, -1187, -1659, -1185,
+    -1530, -1278, 794, -1510, -854, -870, 478, -108, -308, 996, 991, 958, -1460, 1522, 1628,
 ];

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:6:
 //! - Rejection sampling and CBD (η = 2/3) are implemented per spec.
 //! - Generic `d`-bit (de)compression underlies DU/DV packing.

-use crate::params::{N, Q, DU, DV};
+use crate::params::{DU, DV, N, Q};
 use crate::poly::Poly;
-use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
 use sha3::digest::{ExtendableOutput, XofReader};
+use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};

 #[inline(always)]
 fn montgomery_reduce(a: i32) -> i16 {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:36:
     if eta == 2 {
         let mut j = 0usize;
         for i in 0..N / 8 {
-            let t = u32::from_le_bytes([buf[4*i], buf[4*i+1], buf[4*i+2], buf[4*i+3]]);
+            let t =
+                u32::from_le_bytes([buf[4 * i], buf[4 * i + 1], buf[4 * i + 2], buf[4 * i + 3]]);
             let d = (t & 0x5555_5555) + ((t >> 1) & 0x5555_5555);
             for k in 0..8 {
-                let a = ((d >> (4*k)) & 0x3) as i16;
-                let b = ((d >> (4*k + 2)) & 0x3) as i16;
+                let a = ((d >> (4 * k)) & 0x3) as i16;
+                let b = ((d >> (4 * k + 2)) & 0x3) as i16;
                 r.coeffs[j] = a - b;
                 j += 1;
             }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:49:
         // eta == 3
         let mut j = 0usize;
         for i in 0..N / 4 {
-            let t = u32::from_le_bytes([buf[3*i], buf[3*i+1], buf[3*i+2], 0]);
+            let t = u32::from_le_bytes([buf[3 * i], buf[3 * i + 1], buf[3 * i + 2], 0]);
             let d = (t & 0x2492_4924) + ((t >> 1) & 0x2492_4924) + ((t >> 2) & 0x2492_4924);
             for k in 0..4 {
-                let a = ((d >> (6*k)) & 0x7) as i16;
-                let b = ((d >> (6*k + 3)) & 0x7) as i16;
+                let a = ((d >> (6 * k)) & 0x7) as i16;
+                let b = ((d >> (6 * k + 3)) & 0x7) as i16;
                 r.coeffs[j] = a - b;
                 j += 1;
             }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:63:

 /// Backward-compatible alias with `eta=2`.
 #[inline]
-pub fn cbd(buf: &[u8], r: &mut Poly) { cbd_eta(buf, 2, r) }
+pub fn cbd(buf: &[u8], r: &mut Poly) {
+    cbd_eta(buf, 2, r)
+}

 /// `H`: SHA3-256(m) -> 32 bytes.
 #[inline(always)]
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:118:
     let mut ctr = 0usize;
     let mut pos = 0usize;
     while ctr < N {
-        if pos + 3 > buf.len() { break; }
-        let val0 = (buf[pos] as u16) | (((buf[pos+1] as u16) & 0x0F) << 8);
-        let val1 = ((buf[pos+1] as u16) >> 4) | ((buf[pos+2] as u16) << 4);
+        if pos + 3 > buf.len() {
+            break;
+        }
+        let val0 = (buf[pos] as u16) | (((buf[pos + 1] as u16) & 0x0F) << 8);
+        let val1 = ((buf[pos + 1] as u16) >> 4) | ((buf[pos + 2] as u16) << 4);
         pos += 3;
         if val0 < Q as u16 {
             r.coeffs[ctr] = val0 as i16;
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:136:
 /// Serialize a polynomial into 12-bit packed form (384 bytes).
 #[inline]
 pub fn poly_to_bytes(a: &Poly, out: &mut [u8; 384]) {
-    for i in 0..N/8 {
+    for i in 0..N / 8 {
         let mut t = [0i16; 8];
-        for j in 0..8 { t[j] = a.coeffs[8*i + j]; }
         for j in 0..8 {
+            t[j] = a.coeffs[8 * i + j];
+        }
+        for j in 0..8 {
             t[j] = barrett_reduce(t[j]);
             // Canonicalize to [0, q)
             let mut v = t[j] as i32;
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:150:
             v -= ge * Q;
             t[j] = v as i16;
         }
-        let p0 = t[0] as u16; let p1 = t[1] as u16; let p2 = t[2] as u16; let p3 = t[3] as u16;
-        let p4 = t[4] as u16; let p5 = t[5] as u16; let p6 = t[6] as u16; let p7 = t[7] as u16;
+        let p0 = t[0] as u16;
+        let p1 = t[1] as u16;
+        let p2 = t[2] as u16;
+        let p3 = t[3] as u16;
+        let p4 = t[4] as u16;
+        let p5 = t[5] as u16;
+        let p6 = t[6] as u16;
+        let p7 = t[7] as u16;

-        out[13*i+0]  = (p0 & 0xFF) as u8;
-        out[13*i+1]  = ((p0 >> 8) | ((p1 & 0x1F) << 5)) as u8;
-        out[13*i+2]  = ((p1 >> 3) & 0xFF) as u8;
-        out[13*i+3]  = ((p1 >> 11) | ((p2 & 0x03) << 2) | ((p3 & 0x07) << 7)) as u8;
-        out[13*i+4]  = ((p2 >> 6) & 0xFF) as u8;
-        out[13*i+5]  = ((p2 >> 14) | ((p3 & 0xFF) << 1)) as u8;
-        out[13*i+6]  = ((p3 >> 7) | ((p4 & 0x0F) << 4)) as u8;
-        out[13*i+7]  = ((p4 >> 4) & 0xFF) as u8;
-        out[13*i+8]  = ((p4 >> 12) | ((p5 & 0x7F) << 1)) as u8;
-        out[13*i+9]  = ((p5 >> 7) | ((p6 & 0x3F) << 6)) as u8;
-        out[13*i+10] = ((p6 >> 2) & 0xFF) as u8;
-        out[13*i+11] = ((p6 >> 10) | ((p7 & 0x1F) << 3)) as u8;
-        out[13*i+12] = ((p7 >> 5) & 0xFF) as u8;
+        out[13 * i + 0] = (p0 & 0xFF) as u8;
+        out[13 * i + 1] = ((p0 >> 8) | ((p1 & 0x1F) << 5)) as u8;
+        out[13 * i + 2] = ((p1 >> 3) & 0xFF) as u8;
+        out[13 * i + 3] = ((p1 >> 11) | ((p2 & 0x03) << 2) | ((p3 & 0x07) << 7)) as u8;
+        out[13 * i + 4] = ((p2 >> 6) & 0xFF) as u8;
+        out[13 * i + 5] = ((p2 >> 14) | ((p3 & 0xFF) << 1)) as u8;
+        out[13 * i + 6] = ((p3 >> 7) | ((p4 & 0x0F) << 4)) as u8;
+        out[13 * i + 7] = ((p4 >> 4) & 0xFF) as u8;
+        out[13 * i + 8] = ((p4 >> 12) | ((p5 & 0x7F) << 1)) as u8;
+        out[13 * i + 9] = ((p5 >> 7) | ((p6 & 0x3F) << 6)) as u8;
+        out[13 * i + 10] = ((p6 >> 2) & 0xFF) as u8;
+        out[13 * i + 11] = ((p6 >> 10) | ((p7 & 0x1F) << 3)) as u8;
+        out[13 * i + 12] = ((p7 >> 5) & 0xFF) as u8;
     }
 }

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:172:
 /// Deserialize a polynomial from 12-bit packed form (384 bytes).
 #[inline]
 pub fn poly_from_bytes(inp: &[u8; 384], r: &mut Poly) {
-    for i in 0..N/8 {
-        let b = &inp[13*i..13*i+13];
+    for i in 0..N / 8 {
+        let b = &inp[13 * i..13 * i + 13];
         let t0 = (b[0] as u16) | (((b[1] as u16) & 0x1F) << 8);
         let t1 = ((b[1] as u16) >> 5) | ((b[2] as u16) << 3) | (((b[3] as u16) & 0x03) << 11);
         let t2 = ((b[3] as u16) >> 2) | (((b[4] as u16) & 0x7F) << 6);
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:183:
         let t6 = ((b[9] as u16) >> 6) | ((b[10] as u16) << 2) | (((b[11] as u16) & 0x07) << 10);
         let t7 = ((b[11] as u16) >> 3) | ((b[12] as u16) << 5);

-        r.coeffs[8*i+0] = (t0 as i32 % Q) as i16;
-        r.coeffs[8*i+1] = (t1 as i32 % Q) as i16;
-        r.coeffs[8*i+2] = (t2 as i32 % Q) as i16;
-        r.coeffs[8*i+3] = (t3 as i32 % Q) as i16;
-        r.coeffs[8*i+4] = (t4 as i32 % Q) as i16;
-        r.coeffs[8*i+5] = (t5 as i32 % Q) as i16;
-        r.coeffs[8*i+6] = (t6 as i32 % Q) as i16;
-        r.coeffs[8*i+7] = (t7 as i32 % Q) as i16;
+        r.coeffs[8 * i + 0] = (t0 as i32 % Q) as i16;
+        r.coeffs[8 * i + 1] = (t1 as i32 % Q) as i16;
+        r.coeffs[8 * i + 2] = (t2 as i32 % Q) as i16;
+        r.coeffs[8 * i + 3] = (t3 as i32 % Q) as i16;
+        r.coeffs[8 * i + 4] = (t4 as i32 % Q) as i16;
+        r.coeffs[8 * i + 5] = (t5 as i32 % Q) as i16;
+        r.coeffs[8 * i + 6] = (t6 as i32 % Q) as i16;
+        r.coeffs[8 * i + 7] = (t7 as i32 % Q) as i16;
     }
 }

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:252:
 }

 /// DU (u-vector) compressor wrapper.
-#[inline(always)] pub fn poly_compress_u(a: &Poly, out: &mut [u8]) { poly_compress_d(a, DU, out) }
+#[inline(always)]
+pub fn poly_compress_u(a: &Poly, out: &mut [u8]) {
+    poly_compress_d(a, DU, out)
+}
 /// DU (u-vector) decompressor wrapper.
-#[inline(always)] pub fn poly_decompress_u(inp: &[u8], r: &mut Poly) { poly_decompress_d(inp, DU, r) }
+#[inline(always)]
+pub fn poly_decompress_u(inp: &[u8], r: &mut Poly) {
+    poly_decompress_d(inp, DU, r)
+}
 /// DV (v-vector) compressor wrapper.
-#[inline(always)] pub fn poly_compress_v(a: &Poly, out: &mut [u8]) { poly_compress_d(a, DV, out) }
+#[inline(always)]
+pub fn poly_compress_v(a: &Poly, out: &mut [u8]) {
+    poly_compress_d(a, DV, out)
+}
 /// DV (v-vector) decompressor wrapper.
-#[inline(always)] pub fn poly_decompress_v(inp: &[u8], r: &mut Poly) { poly_decompress_d(inp, DV, r) }
+#[inline(always)]
+pub fn poly_decompress_v(inp: &[u8], r: &mut Poly) {
+    poly_decompress_d(inp, DV, r)
+}

 /// Map a 32-byte message to a polynomial with coefficients in `{0, (q+1)/2}`.
 #[inline]
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/src/utils.rs:273:
 /// Map a polynomial back to a 32-byte message (threshold at `q/2`).
 #[inline]
 pub fn poly_to_msg(a: &Poly, msg: &mut [u8; 32]) {
-    for b in msg.iter_mut() { *b = 0; }
+    for b in msg.iter_mut() {
+        *b = 0;
+    }
     for i in 0..N {
         let mut v = a.coeffs[i] as i32;
         v += (Q & (v >> 31));
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/arm64_smoke.rs:1:
 use rand::rngs::OsRng;
-use rusty_kyber::{keypair, encapsulate, decapsulate};
+use rusty_kyber::{decapsulate, encapsulate, keypair};

 fn main() {
     // Tiny runtime smoke (used under cross + qemu-aarch64 in CI)
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/arm64_smoke.rs:7:
     let (ct, ss_sender) = encapsulate(&mut OsRng, &pk);
     let ss_receiver = decapsulate(&sk, &ct);
     assert_eq!(ss_sender.as_bytes(), ss_receiver.as_bytes());
-    println!("arm64 smoke ok, ss[0..4]={:02x?}", &ss_receiver.as_bytes()[0..4]);
+    println!(
+        "arm64 smoke ok, ss[0..4]={:02x?}",
+        &ss_receiver.as_bytes()[0..4]
+    );
 }

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/ctr_drbg.rs:16:
     /// `seed` must be exactly 48 bytes.
     pub fn new(seed: &[u8]) -> Result<Self, String> {
         if seed.len() != 48 {
-            return Err(format!("CTR_DRBG seed must be 48 bytes, got {}", seed.len()));
+            return Err(format!(
+                "CTR_DRBG seed must be 48 bytes, got {}",
+                seed.len()
+            ));
         }
-        let mut drbg = Self { key: [0u8; 32], v: [0u8; 16] };
+        let mut drbg = Self {
+            key: [0u8; 32],
+            v: [0u8; 16],
+        };
         drbg.update(Some(seed));
         Ok(drbg)
     }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/hardening.rs:1:
 #![cfg(feature = "hardening")]

-use subtle::{Choice, ConditionallySelectable};
-use rand::{SeedableRng};
+use rand::SeedableRng;
 use rand_chacha::ChaCha20Rng;
+use subtle::{Choice, ConditionallySelectable};

-use rusty_kyber::{keypair};
 use rusty_kyber::api::Ciphertext;
-use rusty_kyber::hardening::decapsulate_hardened;
 use rusty_kyber::encapsulate;
+use rusty_kyber::hardening::decapsulate_hardened;
+use rusty_kyber::keypair;

 #[test]
 fn hardening_returns_zero_on_fault() {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/hardening_ok.rs:3:
 use rand::SeedableRng;
 use rand_chacha::ChaCha20Rng;

-use rusty_kyber::{keypair, encapsulate, decapsulate};
 use rusty_kyber::hardening::decapsulate_hardened;
+use rusty_kyber::{decapsulate, encapsulate, keypair};

 #[test]
 fn hardened_equals_normal_on_valid_ciphertext() {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/ctr_drbg.rs:16:
     /// `seed` must be exactly 48 bytes.
     pub fn new(seed: &[u8]) -> Result<Self, String> {
         if seed.len() != 48 {
-            return Err(format!("CTR_DRBG seed must be 48 bytes, got {}", seed.len()));
+            return Err(format!(
+                "CTR_DRBG seed must be 48 bytes, got {}",
+                seed.len()
+            ));
         }
-        let mut drbg = Self { key: [0u8; 32], v: [0u8; 16] };
+        let mut drbg = Self {
+            key: [0u8; 32],
+            v: [0u8; 16],
+        };
         drbg.update(Some(seed));
         Ok(drbg)
     }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:65:
         }
         // Push when a full record is formed (seed + pk + sk + ct + ss)
         let mut push_if_ready = || {
-            if cur.seed.is_some() && cur.pk.is_some() && cur.sk.is_some()
-                && cur.ct.is_some() && cur.ss.is_some()
+            if cur.seed.is_some()
+                && cur.pk.is_some()
+                && cur.sk.is_some()
+                && cur.ct.is_some()
+                && cur.ss.is_some()
             {
                 out.push(KatEntry {
                     seed: cur.seed.take(),
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:96:
         }
     }
     // Final record may not be followed by a blank line.
-    if cur.seed.is_some() && cur.pk.is_some() && cur.sk.is_some()
-        && cur.ct.is_some() && cur.ss.is_some()
+    if cur.seed.is_some()
+        && cur.pk.is_some()
+        && cur.sk.is_some()
+        && cur.ct.is_some()
+        && cur.ss.is_some()
     {
         out.push(cur);
     }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:116:
     let path = if Path::new(candidate).exists() {
         Some(candidate)
     } else {
-        ["tests/kat_vectors/kyber512_clean.rsp",
-         "tests/kat_vectors/kyber768_clean.rsp",
-         "tests/kat_vectors/kyber1024_clean.rsp"]
+        [
+            "tests/kat_vectors/kyber512_clean.rsp",
+            "tests/kat_vectors/kyber768_clean.rsp",
+            "tests/kat_vectors/kyber1024_clean.rsp",
+        ]
         .iter()
         .find(|p| Path::new(p).exists())
         .copied()
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:167:
         let (got_ct, got_ss) = encapsulate(&mut drbg, &got_pk);

         // Expect exact byte-for-byte matches
-        assert_eq!(got_pk.as_bytes(), &pk[..], "pk mismatch at entry {} from {}", i, path);
-        assert_eq!(got_sk.as_bytes(), &sk[..], "sk mismatch at entry {} from {}", i, path);
-        assert_eq!(got_ct.as_bytes(), &ct[..], "ct mismatch at entry {} from {}", i, path);
-        assert_eq!(got_ss.as_bytes(), &ss[..], "ss mismatch at entry {} from {}", i, path);
+        assert_eq!(
+            got_pk.as_bytes(),
+            &pk[..],
+            "pk mismatch at entry {} from {}",
+            i,
+            path
+        );
+        assert_eq!(
+            got_sk.as_bytes(),
+            &sk[..],
+            "sk mismatch at entry {} from {}",
+            i,
+            path
+        );
+        assert_eq!(
+            got_ct.as_bytes(),
+            &ct[..],
+            "ct mismatch at entry {} from {}",
+            i,
+            path
+        );
+        assert_eq!(
+            got_ss.as_bytes(),
+            &ss[..],
+            "ss mismatch at entry {} from {}",
+            i,
+            path
+        );

         // And decapsulation roundtrip
         let dec_ss = decapsulate(&got_sk, &got_ct);
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:177:
-        assert_eq!(dec_ss.as_bytes(), got_ss.as_bytes(), "decaps ss mismatch at entry {} from {}", i, path);
+        assert_eq!(
+            dec_ss.as_bytes(),
+            got_ss.as_bytes(),
+            "decaps ss mismatch at entry {} from {}",
+            i,
+            path
+        );

         ran += 1;
     }
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/kat.rs:181:
-    assert!(ran > 0, "no matching KAT entries for this parameter set in {}", path);
+    assert!(
+        ran > 0,
+        "no matching KAT entries for this parameter set in {}",
+        path
+    );
 }

 #[cfg(all(test, feature = "std"))]
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/pack.rs:5:
 use rusty_kyber::params::{CIPHERTEXT_BYTES, DU, DV, N, Q};
 use rusty_kyber::poly::Poly;
 use rusty_kyber::utils::{
-    poly_compress_d, poly_decompress_d, poly_compress_u, poly_decompress_u, poly_compress_v,
+    poly_compress_d, poly_compress_u, poly_compress_v, poly_decompress_d, poly_decompress_u,
     poly_decompress_v, poly_from_bytes, poly_to_bytes,
 };

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/serde.rs:1:
 #![cfg(feature = "serde")]

-use serde_json::{to_vec, from_slice};
+use serde_json::{from_slice, to_vec};

-use rusty_kyber::api::{PublicKey, SecretKey, Ciphertext, SharedSecret};
-use rusty_kyber::params::{PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, CIPHERTEXT_BYTES, SHARED_SECRET_BYTES};
+use rusty_kyber::api::{Ciphertext, PublicKey, SecretKey, SharedSecret};
+use rusty_kyber::params::{
+    CIPHERTEXT_BYTES, PUBLIC_KEY_BYTES, SECRET_KEY_BYTES, SHARED_SECRET_BYTES,
+};

 #[test]
 fn serde_roundtrip_public_types() {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/utils_hash.rs:1:
-use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};
 use sha3::digest::{ExtendableOutput, XofReader};
+use sha3::{Digest, Sha3_256, Sha3_512, Shake128, Shake256};

 use rusty_kyber::params::{N, Q};
 use rusty_kyber::poly::Poly;
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/utils_msg.rs:4:

 fn canon_q(x: i16) -> i32 {
     let mut v = x as i32 % Q;
-    if v < 0 { v += Q; }
+    if v < 0 {
+        v += Q;
+    }
     v
 }

Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/utils_msg.rs:16:
         [0xFF; 32],
         [0xAA; 32],
         [0x55; 32],
-        [0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
-               16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,31],
+        [
+            0x00, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
+            23, 24, 25, 26, 27, 28, 29, 30, 31,
+        ],
     ];

     for m in vectors {
Diff in /home/hootzluh/Desktop/rust/rusty-kyber/tests/utils_msg.rs:28:
         let half = ((Q as i16) + 1) / 2;
         for i in 0..N {
             let v = canon_q(p.coeffs[i]) as i16;
-            assert!(v == 0 || v == half, "coeff {} has unexpected value {}", i, v);
+            assert!(
+                v == 0 || v == half,
+                "coeff {} has unexpected value {}",
+                i,
+                v
+            );
         }

         let mut out = [0u8; 32];
    Checking rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
error: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_parens)]`
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

error: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `-D unused-mut` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(unused_mut)]`

error: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `-D dead-code` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(dead_code)]`

error: casting to the same type is unnecessary (`u32` -> `u32`)
  --> src/ntt.rs:26:15
   |
26 |     let t = ((v as u32 * (a as u32)) >> 26) as i16;
   |               ^^^^^^^^ help: try: `v`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast
   = note: `-D clippy::unnecessary-cast` implied by `-D warnings`
   = help: to override `-D warnings` add `#[allow(clippy::unnecessary_cast)]`

error: casting to the same type is unnecessary (`u32` -> `u32`)
  --> src/utils.rs:26:15
   |
26 |     let t = ((v as u32 * (a as u32)) >> 26) as i16;
   |               ^^^^^^^^ help: try: `v`
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#unnecessary_cast

error: the loop variable `j` is used to index `t`
   --> src/utils.rs:141:18
    |
141 |         for j in 0..8 { t[j] = a.coeffs[8*i + j]; }
    |                  ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
    = note: `-D clippy::needless-range-loop` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::needless_range_loop)]`
help: consider using an iterator and enumerate()
    |
141 -         for j in 0..8 { t[j] = a.coeffs[8*i + j]; }
141 +         for (j, <item>) in t.iter_mut().enumerate() { t[j] = a.coeffs[8*i + j]; }
    |

error: the loop variable `j` is only used to index `t`
   --> src/utils.rs:142:18
    |
142 |         for j in 0..8 {
    |                  ^^^^
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
    |
142 -         for j in 0..8 {
142 +         for <item> in &mut t {
    |

error: this operation has no effect
   --> src/utils.rs:156:13
    |
156 |         out[13*i+0]  = (p0 & 0xFF) as u8;
    |             ^^^^^^ help: consider reducing it to: `(13*i)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#identity_op
    = note: `-D clippy::identity-op` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(clippy::identity_op)]`

error: this operation has no effect
   --> src/utils.rs:186:18
    |
186 |         r.coeffs[8*i+0] = (t0 as i32 % Q) as i16;
    |                  ^^^^^ help: consider reducing it to: `(8*i)`
    |
    = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#identity_op

error: the loop variable `i` is only used to index `s`
  --> src/keygen.rs:28:14
   |
28 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
28 -     for i in 0..K {
28 +     for <item> in s.iter_mut().take(K) {
   |

error: the loop variable `i` is only used to index `e`
  --> src/keygen.rs:34:14
   |
34 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
34 -     for i in 0..K {
34 +     for <item> in e.iter_mut().take(K) {
   |

error: the loop variable `i` is only used to index `s`
  --> src/keygen.rs:42:14
   |
42 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
42 -     for i in 0..K {
42 +     for <item> in s.iter_mut().take(K) {
   |

error: the loop variable `j` is used to index `s`
  --> src/keygen.rs:49:18
   |
49 |         for j in 0..K {
   |                  ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
   |
49 -         for j in 0..K {
49 +         for (j, <item>) in s.iter().enumerate().take(K) {
   |

error: the loop variable `i` is only used to index `rvec`
  --> src/encaps.rs:29:14
   |
29 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
29 -     for i in 0..K {
29 +     for <item> in rvec.iter_mut().take(K) {
   |

error: the loop variable `j` is used to index `rvec`
  --> src/encaps.rs:44:18
   |
44 |         for j in 0..K {
   |                  ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator and enumerate()
   |
44 -         for j in 0..K {
44 +         for (j, <item>) in rvec.iter().enumerate().take(K) {
   |

error: the loop variable `i` is only used to index `u`
  --> src/encaps.rs:77:14
   |
77 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
77 -     for i in 0..K {
77 +     for <item> in u.iter().take(K) {
   |

error: the loop variable `i` is only used to index `u`
  --> src/decaps.rs:22:14
   |
22 |     for i in 0..K {
   |              ^^^^
   |
   = help: for further information visit https://rust-lang.github.io/rust-clippy/master/index.html#needless_range_loop
help: consider using an iterator
   |
22 -     for i in 0..K {
22 +     for <item> in u.iter_mut().take(K) {
   |

error: could not compile `rusty-kyber` (lib test) due to 20 previous errors
warning: build failed, waiting for other jobs to finish...
error: could not compile `rusty-kyber` (lib) due to 20 previous errors
   Compiling libc v0.2.174
   Compiling serde v1.0.219
    Checking zeroize v1.8.1
    Checking getrandom v0.2.16
    Checking rand_core v0.6.4
    Checking rand_chacha v0.3.1
    Checking rand v0.8.5
 Documenting rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
error: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `-D unused-parens` implied by `-D warnings`
    = help: to override `-D warnings` add `#[allow(unused_parens)]`
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

error: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

error: could not document `rusty-kyber`
   Packaging rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
    Updating crates.io index
    Packaged 3168 files, 24.0MiB (3.4MiB compressed)
   Verifying rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber)
   Compiling typenum v1.18.0
   Compiling version_check v0.9.5
   Compiling libc v0.2.174
   Compiling zerocopy v0.8.26
   Compiling cfg-if v1.0.1
   Compiling keccak v0.1.5
   Compiling subtle v2.6.1
   Compiling generic-array v0.14.7
   Compiling getrandom v0.2.16
   Compiling rand_core v0.6.4
   Compiling block-buffer v0.10.4
   Compiling crypto-common v0.1.6
   Compiling digest v0.10.7
   Compiling sha3 v0.10.8
   Compiling ppv-lite86 v0.2.21
   Compiling rand_chacha v0.3.1
   Compiling rand v0.8.5
   Compiling rusty-kyber v0.1.0 (/home/hootzluh/Desktop/rust/rusty-kyber/target/package/rusty-kyber-0.1.0)
warning: unnecessary parentheses around assigned value
   --> src/utils.rs:147:18
    |
147 |             v += (Q & (v >> 31));
    |                  ^             ^
    |
    = note: `#[warn(unused_parens)]` on by default
help: remove these parentheses
    |
147 -             v += (Q & (v >> 31));
147 +             v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:203:17
    |
203 |     let scale = ((1u32 << d) as u64);
    |                 ^                  ^
    |
help: remove these parentheses
    |
203 -     let scale = ((1u32 << d) as u64);
203 +     let scale = (1u32 << d) as u64;
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:209:14
    |
209 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
209 -         v += (Q & (v >> 31));
209 +         v += Q & (v >> 31);
    |

warning: unnecessary parentheses around assigned value
   --> src/utils.rs:279:14
    |
279 |         v += (Q & (v >> 31));
    |              ^             ^
    |
help: remove these parentheses
    |
279 -         v += (Q & (v >> 31));
279 +         v += Q & (v >> 31);
    |

warning: variable does not need to be mutable
  --> src/encaps.rs:28:9
   |
28 |     let mut e1 = [Poly::new(); K];
   |         ----^^
   |         |
   |         help: remove this `mut`
   |
   = note: `#[warn(unused_mut)]` on by default

warning: function `montgomery_reduce` is never used
  --> src/utils.rs:15:4
   |
15 | fn montgomery_reduce(a: i32) -> i16 {
   |    ^^^^^^^^^^^^^^^^^
   |
   = note: `#[warn(dead_code)]` on by default

warning: `rusty-kyber` (lib) generated 6 warnings (run `cargo fix --lib -p rusty-kyber` to apply 5 suggestions)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 4.61s

---

hootzluh@synergy:~/Desktop/rust/rusty-kyber$ cargo audit         # CVEs in transitive deps
cargo +nightly miri test  # UB and undefined-behavior checks in tests (safe Rust should be clean)
    Fetching advisory database from `https://github.com/RustSec/advisory-db.git`
      Loaded 792 security advisories (from /home/hootzluh/.cargo/advisory-db)
    Updating crates.io index
    Scanning Cargo.lock for vulnerabilities (93 crate dependencies)
error: 'cargo-miri' is not installed for the toolchain 'nightly-x86_64-unknown-linux-gnu'.
To install, run `rustup component add miri`

---

