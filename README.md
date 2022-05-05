# ai-meter

<details><summary>JSON objects</summary>

## JSON objects
### Device
``` json
{
  "type": "object",
  "properties": {
    "id": { "type": "number" },
    "name": { "type": "string" },
    "consumption": { "type": "number" },
    "duration": { "type": ["number", "null"] }
  },
  "additionalProperties": false
}
```

### Meter device
``` json
{
  "type": "object",
  "properties": {
    "id": { "type": "number" },
    "name": { "type": "string" },
    "consumption": { "type": "number" },
    "duration": { "type": ["number", "null"] },
    "on": { "type": "bool" }
  },
  "additionalProperties": false
}
```

### Datapoint
``` json
{
  "type": "object",
  "properties": {
    "day_consumption": { "type": "number" },
    "night_consumption": { "type": "number" },
    "current_consumption": { "type": "number" },
    "datetime": { "type": "number" }
  },
  "additionalProperties": false
}

```

### House
``` json
{
  "type": "object",
  "properties": {
    "occupants": { "type": "number" },
    "latitude": { "type": "number" },
    "longitude": { "type": "number" },
    "devices": {
      "type": "array", 
      "items": "meterdevice"
    }
  },
  "additionalProperties": false
}
```

### Meter
``` json
{
  "type": "object",
  "properties": {
    "id": { "type": "number" },
    "last_data_point": { "type": "datapoint" },
    "house": { "type": "house" }
  },
  "additionalProperties": false
}
```
</details>

<details><summary>Endpoints</summary>

## Endpoints
- /device
  - GET: list all devices  
    Returns: array of devices  
    ```sh
    curl --request GET --url http://localhost:3000/device 
    ```

  - POST: create a new device  
    name: String -> Name of the device  
    consumption: integer -> How much the device consumes in kWh  
    duration: null/integer -> how long the device runs for in seconds (null = forever)  

    Returns: the device created  
    ```sh
    curl --request POST \
        --url http://localhost:3000/device \
        --header 'Content-Type: application/json' \
        --data '{
            "name": "crypto farm",
            "consumption": 99999,
            "duration": null
        }'
    ```
    
    - /:device_id or /:device_name
      - GET: Return a specific device  
        Returns: the device  
        ```sh
        curl --request GET --url http://localhost:3000/device/1
        curl --request GET --url http://localhost:3000/device/device%201
        ```

      - PATCH: Update a device  
        Same keys as `POST /device` if a key is not present it will not be updated  

        Returns: updated device  
        NOTE: the value `null` is normally the same as leaving out the key  
        except for duration since it can have the value `null`  
        ```sh
        curl --request PATCH \
            --url http://localhost:3000/device%201/2 \
            --header 'Content-Type: application/json' \
            --data '{
                "name": "renamed device"
            }'
        ```
- /meter
  - GET: list all meters with their attached devices  
    Returns: array of meters
  - POST: Create a meter  
    occupants:integer -> how many people live in the house  
    day_consumption:integer(f32) -> day consumption to start with  
    night_consumption:integer(f32) -> night consumption to start with  
    
    Returns: created meter
    ```sh
      curl --request POST \
          --url http://localhost:3000/meter \
          --header 'Content-Type: application/json' \
          --data '{
              "occupants": 4,
              "day_consumption": 0,
              "night_consumption": 0
          }'
    ```
    - /:meter_id
      - GET: return a specific meter  
        Returns: specific meter  

      - PATCH: Update the meter  
        Same keys as `POST /meter` if a key is not present it will not be updated  
        
        Returns: updated meter
        ```sh
        curl --request PATCH \
            --url http://localhost:3000/meter/4 \
            --header 'Content-Type: application/json' \
            --data '{
                "occupants": 1,
                "latitude": 50.5039,
                "longitude": 4.4699
            }'
        ```
      - /update
          - GET: Same as `GET /meter/:meter_id` but this also update/recalculates the usage  
            Returns: meter with updated day/night consumption
      - /device
        - POST: Add a device to the current meter from the device listed in `GET /device`  
          device: number/string -> device identifier this can be the id or the name  
          on: bool -> Is the device on or off when added  

          Returns: created meterdevice
          ```sh
          curl --request POST \
              --url http://localhost:3000/meter/4/device \
              --header 'Content-Type: application/json' \
              --data '{
                  "device": 5,
                  "on": true
              }'
          ```
        - /:meterdevice_id
          - GET: Returns a specific meter devices attached to the current meter
            Returns: specific meterdevice
          - PATCH: Update a meter device
            on: bool -> turn the device on/off

            Returns: updated meterdevice
            ```sh
            curl --request PATCH \
                --url http://localhost:3000/meter/4/device/13 \
                --header 'Content-Type: application/json' \
                --data '{
                    "on": true
                }'
            ```
</details>

## Setup
### Native
1. Install the following:
- [rust](https://www.rust-lang.org/learn/get-started)
- [diesel cli](https://diesel.rs/guides/getting-started)
- [docker](https://docs.docker.com/get-docker/)

2. Setup the db
```sh
cp example.env .env
docker run --name postgres -e POSTGRES_PASSWORD=toor123 -p 5432:5432 -d postgres
# Wait for a few seconds for the db to start
diesel setup
```
- To start the docker db again
```sh
docker start postgress
```

3. Run the server
- Debug: `cargo run`
- Release: `cargo run --release`
- arguments: `cargo run -- --help`
- NOTE: Values in databse-url must be url encoded

### Docker
```bash
#docker run ghcr.io/tm-pe2/meter --database-url 'postgres://<user-name>:<password>@<server>:<port>/<database>'
docker run ghcr.io/tm-pe2/meter --database-url 'postgres://ai:Ai2022%25@10.97.0.10:5432/meter' --log-level debug
```

## Notes
- `insomnia-req-collection.json` is created with and can be imported by [insomnia](https://insomnia.rest/)
- `tables.drawio` is a [diagrams.net](https://www.diagrams.net/) file
