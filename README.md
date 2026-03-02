[![Book][book-badge]][documentation-mdbook]
[![Documentation][rustdoc-badge]][documentation-dev-rustdoc]

## Project Overview

This fork adds the **NRF54L15DK** board by running `python patch.py`.
Currently working with [this](https://www.aliexpress.com/item/1005008847491389.html?spm=a2g0o.order_list.order_list_main.34.75061802ixCvKU) board
<br><br>
### Ariel OS

Ariel OS builds on top of existing projects from the Embedded Rust ecosystem, including [Embassy](https://github.com/embassy-rs/embassy), [esp-hal](https://github.com/esp-rs/esp-hal), [defmt](https://github.com/knurling-rs/defmt), [probe-rs](https://github.com/probe-rs/probe-rs), [sequential-storage](https://github.com/tweedegolf/sequential-storage), and [embedded-test](https://github.com/probe-rs/embedded-test).
While those provide high-quality building blocks for a wide range of embedded applications, such projects do not provide the high level of integration that developers know from contemporary C-based operating systems for microcontrollers, such as [RIOT](https://github.com/RIOT-OS/RIOT/) or [Zephyr](https://github.com/zephyrproject-rtos/zephyr) for instance.

Ariel OS thus follows an approach whereby it simultaneously integrates many heterogeneous pre-existing crates, and adds missing operating system functionalities such as a preemptive multicore scheduler, portable peripheral APIs, additional network security facilities, as well as [laze](https://github.com/kaspar030/laze), a meta-build system to bind it all together.
The result?
A powerful framework that allows to write portable embedded Rust applications with minimal boilerplate, providing a batteries-included experience.
Still have questions?
Check out our [FAQ](https://github.com/ariel-os/ariel-os/blob/main/FAQ.md) and/or take a look at the [Ariel OS book](https://ariel-os.github.io/ariel-os/dev/docs/book/).

### Minimum Supported Rust Version (MSRV) and Policy

Ariel OS compiles with stable Rust version 1.91 and up.
The MSRV can be increased in patch version updates.

[matrix-badge]: https://img.shields.io/badge/chat-Matrix-brightgreen.svg
[matrix-link]: https://matrix.to/#/#ariel-os:matrix.org
[book-badge]: https://img.shields.io/badge/Book-%F0%9F%93%94-blue
[rustdoc-badge]: https://img.shields.io/badge/Documentation-%F0%9F%93%94-blue
[documentation-mdbook]: https://ariel-os.github.io/ariel-os/dev/docs/book/
[documentation-dev-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/
[getting-started-mdbook]: https://ariel-os.github.io/ariel-os/dev/docs/book/getting-started.html
[hello-world-example]: https://github.com/ariel-os/ariel-os/tree/main/examples/hello-world
[openssf-badge]: https://www.bestpractices.dev/projects/10610/badge
[openssf-project-page]: https://www.bestpractices.dev/projects/10610
[mastodon-badge]: https://img.shields.io/badge/social-Mastodon-informational.svg
[mastodon-link]: https://floss.social/@ariel
