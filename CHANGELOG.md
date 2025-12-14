## v0.9.3

- Crates updated:
    - serde (v1.0.228: features).
    - serde_json (v1.0.145: features).

## v0.9.2

- Renamed mysql module to mysql_async (mysql_async = "0.36.1")

## v0.9.1

- Crates updated:
    - cans (v1.1.0 -> v1.3.0).

## v0.9.0

- Crates updated:
    - wtime (v0.6.0 -> v0.7.0).
    - mathlab (v1.4.0 -> v1.5.0).
- Modules / Crates removed:
    - mongo / mongodb = "3.3.0" 
    - psql / tokio-postgres = "0.7.14"
    - redis / redis = "0.32.7"
- Modules / Crates added:
    - mysql / mysql_async = "0.36.1"

## v0.8.0

- Functions changed:
    - Started from version 0.8.0:
        - The order of parameters in the route function has been updated from (path, method, handler) to (method, path, handler).
        - This change improves code readability and consistency by placing the HTTP method first, aligning with common conventions and making it easier to group related methods together for clarity.
- Crates updated:
    - tokio (v1.46.1 -> v1.47.1).
    - url (v2.5.4 -> v2.5.7).
    - tokio-postgres (v0.7.13 -> v0.7.14).
    - redis (0.32.3 -> 0.32.7).
    - mongodb (3.2.4 -> 3.3.0).
    - serde (1.0.219 -> 1.0.228).
    - serde_json (1.0.140 -> 1.0.145).
    - mathlab (1.3.0 -> 1.4.0).
- Modified fluxor cli

## v0.7.1

- Added fluxor cli (cans-template-engine example).

## v0.7.0

- Added fluxor cli (dotenv example).
- Modified fluxor cli (all examples).

## v0.6.1

- Modified fluxor cli (assets example).
- Updated:
    - tokio (v1.45.1 -> v1.46.1).
    - redis (0.32.0 -> 0.32.3).
    - mongodb (3.2.3 -> 3.2.4).

## v0.6.0

- Added fluxor cli (assets example).
- Updated:
    - redis (0.31.0 -> 0.32.0).

## v0.5.0

- Added fluxor cli (routes-project example)

## v0.4.0

- Added fluxor cli (routes example)

## v0.3.0

- Removed main.rs.
- Updated:
    - tokio (v1.44.1 -> v1.45.1).
    - redis (0.29.1 -> 0.31.0).
    - mongodb (3.2.2 -> 3.2.3).

## v0.2.0

- Moved Fluxor-cli to its own crate.

## v0.1.1

- Added tokio crate (pub use) to Prelude Module.

## v0.1.0

- Initial version.