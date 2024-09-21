# mash 🥔

**as simple as potatoes**

Welcome to **MASH** - a minimalist web development stack that combines the power of **m**aud, **a**xum, **s**ass and **h**tmx.

## Why mash? 🤔

- **Maud**: A blazing-fast Rust-based templating engine that lets you write HTML directly in Rust, ensuring type safety and avoiding common HTML errors.
- **Axum**: A powerful web framework built on Tokio that handles routing and request management with ease and speed.
- **Sass**: A preprocessor scripting language that is interpreted or compiled into CSS, allowing for more maintainable and modular CSS styling.
- **HTMX**: A JavaScript library that enables interaction between the client and server without writing JavaScript—using HTML attributes like `hx-post` and `hx-target`.

MASH allows for rapid prototyping with simple UI elements while maintaining performance and ease of development. By integrating **sass** from the [yree/mold](https://github.com/yree/mold) framework, you get clean, responsive layouts without needing to write custom CSS. The styling is minimal but customizable, ensuring that your projects look modern without bloat.

## Hello World! 🌍

In this [example](https://github.com/8hantanu/mash), we build a basic web application where the user inputs their name, and the server responds with a greeting. The app leverages HTMX to dynamically update the page without needing a full reload, Maud for HTML generation, Axum for request routing, and Sass for minimal styling.

<details>
<summary>See mash demo 🥔</summary>
<hr>
<iframe src="https://mash.fly.dev" title="A mash demo 🥔" height="200ch"></iframe>
</details>

The project includes:
- `handlers.rs`: Contains the logic for handling HTTP requests. In this example, it renders the form and handles form submissions.
- `main.rs`: The entry point for the Axum web server, routing requests to the handlers.
- `views.rs`: Defines the HTML templates using Maud for both the form and response.

To continuously build and run the app use `cargo-watch`:

```bash
cargo watch -x run
```

Access the app at `http://127.0.0.1:3000`.

### Breaking it down 🔍

#### **Axum** - The Web Framework 🌐
Axum is a web framework that uses an asynchronous model, ideal for handling web traffic efficiently. In this example, **Axum** provides:
- **Routing**: Handling `GET` and `POST` requests to serve the form and process the submission.
- **Extractors**: Handling form data extraction using Serde's `Form`.

```rust
.route("/", get(handlers::get_form))
.route("/submit", post(handlers::handle_submit))
```

#### **Maud** - HTML Templating in Rust 🦀
**Maud** allows you to write HTML using Rust's type-safe syntax. This eliminates any risk of invalid HTML while keeping your code minimal and clean. Maud renders the form and the dynamic greeting.

```rust
html! {
    div class="w" {
        h1 { "What's your name!" }
        form hx-post="/submit" hx-target="#response" {
            input type="text" id="name" name="name" required;
            button { "Submit" }
        }
    }
}
```

#### **HTMX** - Simple Dynamic Interactions ⚡
**HTMX** simplifies AJAX-like requests by using HTML attributes. In this example, `hx-post` sends a POST request when the form is submitted, and `hx-target` updates the content of a specific DOM element (in this case, the `#response` div) without needing custom JavaScript.

```html
<form hx-post="/submit" hx-target="#response">
```

#### **Sass** - Styling the Application 🎨
[Mold](https://yree.io/mold) is a pre-built sass framework that keeps the CSS minimal and responsive. I include Mold’s [SCSS](https://github.com/yree/mold/blob/master/_sass/mold.scss) for my styling needs, using classes like `.grid` and `.w` to manage layout and responsiveness with almost no additional effort. You can easily extend or override styles using Sass variables or mixins to ensure that your app's UI is both customizable and scalable.

## Why you'll love MASH ❤️

- **Minimalist**: The stack is designed to be lightweight and efficient. Maud ensures your HTML is clean and type-safe, while HTMX eliminates the need for complex JavaScript.

- **Rapid Prototyping**: Thanks to Axum’s easy routing and HTMX’s seamless client-server interaction, you can quickly prototype interactive web applications.

- **Simple Styling**: Using Sass from the [mold](https://yree.io/mold) framework provides responsive design with minimal effort, keeping UI/UX straightforward, clean, and adaptable.

- **Performance**: By using Rust and asynchronous programming via Tokio (used by Axum), the app remains highly performant even under load.

It's plain, versatile, and gets the job done effectively.

With the MASH stack, you get a straightforward, efficient, and minimalist approach to web development. Perfect for fast prototyping and small-scale applications, it's **as simple as potatoes**! 🥔
