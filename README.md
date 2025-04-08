# Portfolio Website

## Tech stack:
- Frontend:
  - Rust with Yew Framework: https://yew.rs/
  - Tailwind CSS: https://tailwindcss.com/

- Backend:
  - Rust with Actix Web Framework: https://actix.rs/

I chose Rust over JavaScript for this project for many reasons. While I love JS and have a lot of experience with Node, Next, and React, I find Rust to be cleaner, more intuitive, and easier to scale (this project is small now but will continue to grow). I also love and appreciate Tailwind -- I found it easy to integrate Tailwind with the Yew framework.

## To run:

### Frontend
- #### Step 1: change to frontend dir
  - `cd frontend`

- #### Step 2: build tailwind locally
  - `npx tailwindcss -i ./styles/input.css -o ./styles/output.css`

- #### Step 3: start app
  - `trunk serve`


### Backend
- #### Step 1: change to backend dir
  - `cd backend`

- #### Run test suite
  - `cargo test`

- #### Run dev server
  - `cargo build`
  - `cargo run`


## References
- https://github.com/wpcodevo/fullstack-rust-app
- 'Add Tailwind CSS to your Yew project': https://www.youtube.com/watch?v=DEWoizX96k8
- https://freefrontend.com/
