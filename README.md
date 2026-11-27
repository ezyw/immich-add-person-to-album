### License
Do anything with this, reference this, or not I'm ok

### Whats this

This talks to Immich server `serverUrl`, for each rule, 
use `apiKey` fetch images with `personId` and adds it to `albumId

## Build and run
First edit the config/config.yaml.example and save as config/config.yaml

```
make build
make run
```


### Example Docker Compose

```
services:
  immich-server:
    container_name: immich_server
    image: ghcr.io/immich-app/immich-server:${IMMICH_VERSION:-release}
    ... (Your immich config)

  immich-person-to-album:
    image: ghcr.io/ezyw/immich-person-to-album:latest
    volumes:
      - ./config:/app/config:Z
    restart: "no"
    
networks:
  default:
  proxy:
    external: true
```
Then ask chatgpt to do some systemd magic and call this when appropriate