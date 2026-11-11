# Rocket Ignite docs

The documentation site for [Ignite](https://github.com/lajennylove/Ignite), an admin panel for Rocket.rs. Built on Rocket and [Propel](https://github.com/lajennylove/Propel) views, with Tailwind and Vite for the assets.

The documentation for [Ignite](https://github.com/lajennylove/Ignite): the panel, resources, forms, tables, actions, notifications, widgets and views. The pages describe Ignite on its main branch, where the database layer is [Orbit](https://github.com/lajennylove/Orbit) and Diesel is gone. Each page says what is not built yet, such as filters and bulk actions.

## Dependencies

The site is a Rocket app with Propel views. It does not depend on the Ignite crates.

- Rocket: git revision `3a54d079`
- Propel: git revision `09c8a02ba0f3ed75b02c7f3d36b5a0e6f4d047b7` (`propel_rocket`, `propel_runtime`, `propel_macros`, and `propel_compiler` as a build dependency)

The install page for apps pins Ignite to a git revision, and Orbit, Rocket and Propel to their crates.io releases, the same ones Ignite resolves. This site's own pins are in `Cargo.toml`.

## Developing this site

These steps are for developing the documentation site. An app that uses Ignite does not need them. Node.js 22.12 or newer, or 20.19 or newer, is needed for Vite and Tailwind.

```sh
npm install
npm run build  # once: Rocket will not start until public/build exists
npm run dev    # Vite, in one terminal
cargo run      # Rocket, in another
cargo build --release   # a release build compiles the views in build.rs
```
