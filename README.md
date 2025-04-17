# Portfolio Website
https://jackflores.com

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
  - Option 1: just trunk
    - in a new shell: `trunk serve`
  - Option 2: docker
    - `docker build -t frontend`
    - `docker run -p 8080:8080 frontend`


### Backend
- #### Step 1: change to backend dir
  - `cd backend`

- #### Run test suite
  - `cargo test --test main`

- #### Run dev server
  - `cargo build`
  - `cargo run`
## Backend API -- In Development
I am building an API to manage records of my live performances. I will soon have a shows page that will fetch every show record from the database and display them on the webpage.
  - `POST /shows`: create a show record
    - Required fields:
      - `city`: nonempty string
      - `date`: string in YYYY-MM-DD format
      - `state`: valid state code (e.g. MA)
      - `venue`: nonempty string
    - Optional fields:
      - `poster`: pdf
      - `ticket_link`: url
    - Example: `curl -X POST http://localhost:8000/shows \-F "city=Somerville" \-F "date=2025-4-20" \-F "state=MA" \-F "ticket_link=https://google.com" \-F "venue=The Jungle"`
    - Returns: 
      - 201 Created if all fields pass validation and database record is created
      - 400 Bad Request if any fields are in bad format or any required fields are missing
      - 500 Internal Server Error if a database error occurs
  - `GET /shows`: get all existing shows
    - Example: `curl -X GET http://localhost:8000/shows`
    - Returns:
        - 200 Ok with a json vector of all existing show records, or
        - 500 Internal Server Error if a database error occurs



## References
- Rust docs: https://docs.rs/
- Yew docs: https://yew.rs/docs/getting-started/introduction
- Tailwind docs: https://tailwindcss.com/docs/installation
- Zero To Production In Rust: https://www.zero2prod.com/
- Fullstack rust app example: https://github.com/wpcodevo/fullstack-rust-app
- 'Add Tailwind CSS to your Yew project': https://www.youtube.com/watch?v=DEWoizX96k8
- Took inspiration for tailwind components from here: https://freefrontend.com/
- Icons from https://finnbear.github.io/yew_icons/
- Favicon from https://favicon.io/favicon-generator/