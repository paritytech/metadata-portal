# Frequency Metadata Portal

## Overview

The *Frequency Metadata Portal* is a web page that shows users the latest metadata for the *Frequency* blockchain for use with the *Parity Signer* mobile app.

The URL for accessing the portal is:

https://metadata.frequency.xyz

The portal is intended for use by:
- *Frequency* developers (to validate and sign metadata)
- Council members, Sudo key holders and *Frequency* users (to sign transactions with the Parity Signer app)

*Parity Signer* mobile app (air-gapped wallet)
https://www.parity.io/technologies/signer

## CLI Command Definitions

### Updater
1. Updates using "node" by default but can also update from "github"
2. Iterates through each chain in `config.toml`
    1. Fetches chain specs and metadata for each chain
    2. Generates a QR code for each chain spec
    3. If the metadata version is not equal to the current version
        1. Generates a new video QR code
        2. Inserts metadata into QR code

### Collector
1. Fetches all spec and metadata QR codes
2. Iterates through each chain in `config.toml`
    1. Fetches the chain specs and metadata
    2. Gets active metadata version
    3. Checks if there is an existing metadata QR image file
    4. Checks if there is an existing chainspec QR image file
    5. Determines if a metadata QR file is of a later version than the metadata currently deployed
    6. Updates a symlink to point to the latest metadata QR imagefile
    7. Builds a map containing the chain name to a list of spec values
3. Writes the map to `public/data.json`

### Signer
1. Obtains a list of unsigned QR image files in `public/qr`
2. Iterates through the list of unsigned QR image files
    1. Prompts user to select an unsigned QR image file to sign
    2. Displays preview of the selected file to scan with Parity Signer mobile app
        - On macOS, the Preview app is able to display the `.png` QR image file but is unable to display the `.apng` QR video file for the metadata - this can be opened with Google Chrome
    3. Opens the Camera to scan the signed chain spec or metadata produced by Parity Signer mobile app
    4. Deposits the signed chain spec or metadata as a QR image file in `public/qr`

### Verifier
1. Obtains a list of all QR barcode image files in `public/qr`
2. Iterates through the files
    - Returns an error and exit if the QR image file is unsigned
3. Iterates through the list of all QR image files (second round)
    - Determines if the QR image file is of content type metadata
        1. Determines if the signature of the metadata QR image file was produced by the private key holder of the public_key provided in `config.toml`
        2. Returns an error and exits if the signature doesn't match
### Cleaner
1. Obtains a list of of all QR image files in `public/qr`
2. Obtains a list of each metadata QR image file
3. Obtains a list of each chainspec QR image file
4. Fetches the chain specs
5. Instantiates a HashSet to store files to keep
6. Iterates through each chain in `config.toml`
    1. Determines the current metadata version from the chain specs
    2. Determines which metadata QR files are versioned equal to or greater than the current meta data version; these files are kept in the HashSet
    3. Stores Chainspec QR files are kept in the HashSet
7. Determines the difference of all files and kept files, these are the files to be removed
8. Iterates through each file to be removed and delete it

### Check Deployment
1. Generates the contents of a `data.json` and compares the generated file with the hosted `data.json` stored at the root of the homepage specified in `package.json`; for example: https://metadata.frequency.xyz/data.json
2. If generated vs hosted `data.json` mismatch, `exit(12)`

## Github Actions Workflows

### Check Updates Workflow
`.github/workflows/update.yml`

Runs daily at 00:00:00 UTC.
#### update job
This job uses a branch of the format: `sign-me-<year>-<month>-<day>`. The branch places unsigned metadata QR barcode image files into the its `public/qr` directory.
1. Determines if there is updated runtime metadata from:
    - The RPC node (current version)
    - GitHub latest WASM release assets (next version)
2. If metadata has been updated:
    - Commits added files to the branch
    - Creates a pull request
3. Notifies technical council or sudo key holder via a Matrix channel (if specified) that new metadata is available
4. A member of the technical council or a sudo key holder may then checkout the branch locally and:
    - Run `make signer` to sign the files
    - Run `make collector` to collect version information about the current chains
    - Run `make cleaner` to remove obsolete QR image files
    - Commit changes to the branch so that it may be reviewed by the technical committee
#### check-deployment job
The purpose of check-deployment is to keep the data.json up to date
1. Compares GitHub pages hosted https://metadata.frequency.xyz/data.json vs RPC fetched specs from the node
2. If the hosted vs fetched specs differ
    - Runs the collector to build a new `data.json`
    - Initiates the deploy workflow (`.github/workflows/deploy/action.yml`) to redeploy the GitHub pages hosted site with the updated `data.json`

### Deploy Main Workflow
`.github/workflows/deploy.yml`

Triggered by changes to `*.yml` files or the `config.toml` in the `main` branch.

#### deploy job
The purpose of deploy is to:
1. Run the verifier to make sure all QR image files are signed by Frequency
2. Run the collector to build a new `data.json`
3. Redeploy the GitHub pages site with the updated `data.json` (by use of the deploy action)

### CLI Workflow
`.github/workflows/cli-test.yml`

Runs on pull requests targeting the `main` branch with file changes in `cli/**`.
#### test job
The purpose of test is to make sure that formatting, linting and tests pass.
1. Runs `cargo fmt --all -- --check`
    - This currently fails on the stable channel of the rust toolkit
2. Runs `cargo clippy`
3. Runs `cargo test`
4. Runs `cargo check`

### Frontend Test Workflow
`.github/workflows/frontend-test.yml`

Runs on pull requests targeting the `main` branch.
### frontend-test job
The purpose of frontend-test is to run linting and React tests

### Verify Workflow
`.github/workflows/verify.yml`

Runs on pull requests targeting the `main` branch with file changes in `public/qr/**` or `config.toml`.
#### verify job
The purpose of verify is to ensure that all QR image files are signed by Frequency

### Verify Skip Workflow
`.github/workflows/verify-skip.yml`

Runs on pull requests targeting the `main` branch with file changes excluding `public/qr/**` or `config.toml`.
#### verify-skip job
The purpose of verify-skip is to serve as a negation of the verify workflow and job
- Displays "No verification required" to signify that the file changes did NOT require reverification
