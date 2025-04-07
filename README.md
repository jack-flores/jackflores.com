# Portfolio Website

## Tech stack:
- Frontend:
  - Rust with Yew Framework: https://yew.rs/
  - Tailwind CSS: https://tailwindcss.com/

- Backend:
  - Rust with Actix Web Framework: https://actix.rs/


## To run:

### Frontend
- #### Step 1: change to frontend dir
  - `cd frontend`

- #### Step 2: build tailwind locally
  - `npx tailwindcss -i ./styles/input.css -o ./styles/output.css`

- #### Step 3: start app
  - `trunk serve`


### Backend
- #### Step 1: change to frontend dir
  - `cd backend`

- #### Run test suite
  - `cargo test`

- #### Run dev server
  - `cargo build`
  - `cargo run`


## References
- https://github.com/wpcodevo/fullstack-rust-app
- 'Add Tailwind CSS to your Yew project': https://www.youtube.com/watch?v=DEWoizX96k8
