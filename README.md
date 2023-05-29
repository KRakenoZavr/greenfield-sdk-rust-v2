# greenfield-sdk-proto

[![Crate][crate-image]][crate-link]
[![Docs][docs-image]][docs-link]
[![Build Status][build-image]][build-link]
[![Apache 2.0 Licensed][license-image]][license-link]
![MSRV][rustc-image]

Rust crate for interacting with [Protobufs] defined by the [greenfield SDK].

The goal of this crate is to provide complete proto struct definitions for interacting
with a greenfield SDK blockchain.

Currently, this crate only provides a subset of the many total structs exported by
greenfield SDK proto files.

Pull requests to expand coverage are welcome.

[Documentation][docs-link]

## Minimum Supported Rust Version

This crate is supported on Rust **1.63** or newer.

[//]: # "badges"
[crate-image]: https://buildstats.info/crate/cosmos-sdk-proto
[crate-link]: https://crates.io/crates/cosmos-sdk-proto
[docs-image]: https://docs.rs/cosmos-sdk-proto/badge.svg
[docs-link]: https://docs.rs/cosmos-sdk-proto/
[build-image]: https://github.com/cosmos/cosmos-rust/workflows/cosmos-sdk-proto/badge.svg
[build-link]: https://github.com/cosmos/cosmos-rust/actions/workflows/cosmos-sdk-proto.yml
[license-image]: https://img.shields.io/badge/license-Apache2.0-blue.svg
[license-link]: https://github.com/cosmos/cosmos-rust/blob/master/LICENSE
[rustc-image]: https://img.shields.io/badge/rustc-1.63+-blue.svg

[//]: # "links"
[Protobufs]: (https://github.com/cosmos/cosmos-sdk/tree/master/proto/)
[greenfield SDK]: (https://github.com/bnb-chain/greenfield-go-sdk)

MIT License
THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE. Copyright (c) 2023
