# What order of pickup
The aim of this project is to implement a pathfinding algorithm that efficiently relays the order which you should pick people up.

## Prerequisites
- Docker installed on your machine. You can download and install Docker from [here](https://docs.docker.com/get-docker/).

## Getting Started

1. **Clone the repository**:

    ```sh
    git clone https://github.com/your-username/your-repo.git
    cd your-repo
    ```

2. **Build the Docker image**:

    ```sh
    docker build -t friend_group_tsp .
    ```

3. **Run the Docker container**:

    ```sh
    docker run -it friend_group_tsp
    ```

### Developing

To make changes and develop the application further:

1. **Edit the source code**: Make changes to the files in the `src` directory.

2. **Rebuild the Docker image** after making changes:

    ```sh
    docker build -t friend_group_tsp .
    ```

3. **Run the container** to see your changes:

    ```sh
    docker run -it friend_group_tsp
    ```

### Troubleshooting

- **Docker Logs**: If you encounter any issues, check the container logs using:

    ```sh
    docker logs <container_id>
    ```

- **Common Issues**:
    - Ensure Docker is running.
    - Check for typos or errors in the `Dockerfile`.
    - Ensure the project directory structure matches the expected layout.