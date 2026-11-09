# Rocket Ignite docs

The documentation site for [Ignite](https://github.com/lajennylove/Ignite), an admin panel for Rocket.rs. Built on Rocket and [Propel](https://github.com/lajennylove/propel) views, with Tailwind and Vite for the assets.

Status: the panel, the resources, the derive and the dashboard are documented. The pages say what is not built yet, such as CSRF protection and a table of the latest records.

## Dependencies

- Rocket: git revision `3a54d079`
- Propel: git revision `1853048` (`propel_rocket`)

## Run

```sh
npm install
npm run build  # once: Rocket will not start until public/build exists
npm run dev    # Vite, in one terminal
cargo run      # Rocket, in another
```
