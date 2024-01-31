<picture>
    <source srcset="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_Solid_White.svg" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/leptos-rs/leptos/main/docs/logos/Leptos_logo_RGB.svg" alt="Leptos Logo">
</picture>

# Leptos CSR Examples

You can [see the deployed examples here][deployed-examples].

To run the frontend examples locally, clone the git repo & `cd` into the directory.

Then to spin up the examples, run:

```sh
trunk serve --port 3000 --open
```

To build for deployment / release:

```sh
trunk build --release
```

To build the full example project locally, (including the backend API) you'll need the Spin CLI.

<br/>
To start up the project, run
```sh
spin watch
```

then go to `localhost:3000` to see the running examples, with the API server handling the contact form submission.

[deployed-examples]: https://csr-examples-hjh4tnot.fermyon.app
