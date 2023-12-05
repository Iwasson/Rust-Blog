# Rust-Blog By Ian Wasson
A project website written in Rust for making blog posts.


## Description
What is this?

This is a Rust website built on the Tokio and Axum framework to allow a user to view blogs written by other users, as well as make their own blogs. The basic premise of this site is that you can style your blog posts using a subset of markdown which will be interpreted by the server and styled appropriately.

### How to build
To build this project first you need to have docker installed. Follow the next steps:
1. Clone repo
2. Rename .env.example to be .env
3. Run `sudo docker-compose up`
4. CD into backend folder
5. Run `sqlx migrate run`
6. Run `cargo run`
7. In a new terminal, CD into the client folder
8. Run `cargo run`, this will create the a test account using the email: `test@test.com` and the password: `1234`.
8. Navigate to a web browser and type in `127.0.0.1:3000` into the search bar
9. Enjoy!

### Testing
The testing was primarily done with a combination of the client and manual testing on the web browser. The client side is able to send REST requests to the backend and get the response headers back. But to test whether or not things were being displayed correctly, I had to manually inspect the website while it was running and go through a set of test cases that I had constructed for myself. 

### How Well Did it Go?
There are still a couple of gaps that I would like to fix. The main issue is that I do not currently have user registration working. For now using the client side of the project to inject a user is the easiest way to test this project as it is still a prototype. 

The other thing that I would like to have is better markdown parsing. Right now its pretty simple and only matches on very specific rules, the bold, italics and strikethrough all only work if the line starts with and ends with their respective characters. I would like to figure out a regex method for matching these cases instead.

I am satisfied that the website runs and that a blog can be created and saved in the database. I think there are a lot of features that I could add if I had more time to invest into this. Additionally, if there was a more clear theme for this site, I think it would be more fun to build, for now this was just a proof of concept.

### License information
This project is licensed under the MIT license. More information can be found in the LICENSE file at the root of this project.