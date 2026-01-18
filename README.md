# docker_axum_microservices
Demo of containerizing rust axum microservices for running on a single host server (using docker compose for DNS).

# Setup
To run, simply:

* `docker-compose up --build` (make sure you have docker installed!)

To reach service A (without calling dependencies):

* Navigate to `localhost:3000` in browser

To call service A with all dependencies:
* Navigate to `localhost:3000/call-service-b` in browser

# Structure
This is a demo of a microservices system, where the services communicate via docker compose. The services are composed as follows:

<img width="2194" height="825" alt="Blank diagram" src="https://github.com/user-attachments/assets/251b4700-6375-4dfe-abda-8c850d08e763" />
