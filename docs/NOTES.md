# CLI Command Definitions

## Updater
1. Iterate through each chain in `config.toml`
    a. Fetches chain specs and metadata for each chain
    b. Generates a QR code for each chain spec
    c. If the metadata version is not equal to the current version
        i. Generates a new video QR code
        ii. Inserts metadata into QR code

## Collector
1. Fetches all spec and metadata QR codes
2. Iterates through each chain in `config.toml`
    a. Fetches the chain specs and metadata
    b. Gets active metadata version
    c. Checks if there is an existing metadata QR image file
    d. Checks if there is an existing chainspec QR image file
    e. Determines if a metadata QR file is of a later version than the metadata
        currently deployed
    f. Updates a symlink to point to the latest metadata QR imagefile
    g. Builds a map containing the chain name to a list of spec values
3. Writes the map to `public/data.json`

## Signer

## Verifier

## Cleaner

## Check Deployment
1. Check if deployment is up to date
