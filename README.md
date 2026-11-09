# Rocket Ignite docs

The documentation site for [Ignite](https://github.com/lajennylove/Ignite), an admin panel for Rocket.rs. Built on Rocket and [Propel](https://github.com/lajennylove/Propel) views, with Tailwind and Vite for the assets.

Status: the panel, the resources, the derive and the dashboard, with its latest-records tables, are documented. The pages say what is not built yet, such as filters and bulk actions.

## Dependencies

- Ignite: the site documents commit `265cb1df1bccc7d8a343b5fa300027e28b8d011d`
- Rocket: git revision `3a54d079`
- Propel: git revision `71ecd1c0c9676db74ce0c90ea3578a12fc043f21` (`propel_rocket`)
- Diesel: git fork `lajennylove/diesel` at `a07c4b2c20a466ef34f50ca283790f4df20b0699`

## Run

```sh
npm install
npm run build  # once: Rocket will not start until public/build exists
npm run dev    # Vite, in one terminal
cargo run      # Rocket, in another
cargo build --release   # a release build compiles the views in build.rs
```
