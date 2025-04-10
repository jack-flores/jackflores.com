# Portfolio Website

## Tech stack:
- Frontend:
  - Rust with Yew Framework: https://yew.rs/
  - Tailwind CSS: https://tailwindcss.com/

- Backend:
  - Rust with Actix Web Framework: https://actix.rs/

I chose Rust over JavaScript for this project for many reasons. While I love JS and have a lot of experience with Node, Next, and React, I find Rust to be cleaner, more intuitive, and easier to scale (this project is small now but will continue to grow). I also love and appreciate Tailwind -- I found it easy to integrate Tailwind with the Yew framework.

## To run:

### Setup
- You'll need to install Rust and Tailwind:
  - https://www.rust-lang.org/tools/install
  - https://tailwindcss.com/docs/installation
- Then, follow the steps on this page to compile Rust to WebAssembly and install Trunk
  - https://yew.rs/docs/getting-started/introduction


### Frontend
- #### Step 1: change to frontend dir
  - `cd frontend`

- #### Step 2: build tailwind locally
  - `npx tailwindcss -i ./styles/input.css -o ./styles/output.css --watch`

- #### Step 3: start app
  - in a new shell: `trunk serve`


### Backend
- #### Step 1: change to backend dir
  - `cd backend`

- #### Run test suite
  - `cargo test`

- #### Run dev server
  - `cargo build`
  - `cargo run`


## References
- Rust docs: https://docs.rs/
- Yew docs: https://yew.rs/docs/getting-started/introduction
- Tailwind docs: https://tailwindcss.com/docs/installation
- Fullstack rust app example: https://github.com/wpcodevo/fullstack-rust-app
- 'Add Tailwind CSS to your Yew project': https://www.youtube.com/watch?v=DEWoizX96k8
- Took inspiration for tailwind components from here: https://freefrontend.com/
