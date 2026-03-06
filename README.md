# NomadLibre Website

**Libre software that moves with you, across every device you own.**

This is the source code for the official NomadLibre landing page, built with Rust, Dioxus, and Tailwind CSS. It is compiled to WebAssembly and hosted statically on GitHub Pages.

## Philosophy
* **Offline First, KISS:** Keep it simple, stupid. Tools should work without an internet connection whenever possible.
* **No Data Harvesting:** Zero unnecessary telemetry. Your data belongs to you.
* **Stable Experiences:** Built for longevity and stability, favoring official distribution channels (RPM/DEB/Flatpak).
* **Cross-Platform:** Write once in Rust, deploy to Windows, macOS, Linux, Android, iOS, FreeBSD and the Web.

## The Roadmap
NomadLibre is currently a solo operation laying the groundwork for a suite of cross-platform utilities. The future development pipeline includes:

1. **Personal Finance:** Local-first, secure financial tracking.
2. **MoveLivre:** An open-source, ad-free transit routing alternative.
3. **Board Games:** Open-source implementations of Indian Ludo and Snakes & Ladders.
4. **Mobile Terminal:** A robust terminal built natively for Android and iOS.
5. **Secure P2P Chat:** Decentralized messaging protocol utilizing SSH keys.
6. **The Meme App:** The ultimate ad simulator (proceed with caution).

## Local Development
To run this website locally, you will need the Rust toolchain and the Dioxus CLI installed.

`rustup target add wasm32-unknown-unknown`
`cargo install dioxus-cli`
`dx serve`

## Contributing and Team
NomadLibre is currently maintained by a solo developer. I am in a money crunch and absolutely do not expect free work from anyone (not even interns). If you want to support the project, starring the repositories or sharing the tools is the best way to help.

## License
This repository is licensed under the GNU Affero General Public License v3.0 (AGPLv3). See the `LICENSE` file for details.