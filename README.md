# Rocket Ignite docs

The documentation site for [Ignite](https://github.com/lajennylove/Ignite), an admin panel for Rocket.rs. Built on Rocket and [Propel](https://github.com/lajennylove/propel) views, with Tailwind and Vite for the assets.

Status: the panel, the resources, the derive and the dashboard, with its latest-records tables, are documented. The pages say what is not built yet, such as filters and bulk actions.

## Dependencies

- Ignite: the site documents commit `c8591235e9d6939b5209b6daeba32566f21f9179`
- Rocket: git revision `3a54d079`
- Propel: git revision `81b97ce213bbedc3992823e846f0d6689eeb602d` (`propel_rocket`)
- Diesel: git fork `lajennylove/diesel` at `a07c4b2c20a466ef34f50ca283790f4df20b0699`

## Run

```sh
npm install
npm run build  # once: Rocket will not start until public/build exists
npm run dev    # Vite, in one terminal
cargo run      # Rocket, in another
cargo build --release   # a release build compiles the views in build.rs
```
