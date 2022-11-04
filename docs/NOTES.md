# CLI Command Definitions

## Updater
1. Updates using "node" by default but can also update from "github"
2. Iterates through each chain in `config.toml`
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
    e. Determines if a metadata QR file is of a later version than the metadata currently deployed
    f. Updates a symlink to point to the latest metadata QR imagefile
    g. Builds a map containing the chain name to a list of spec values
3. Writes the map to `public/data.json`

## Signer
1. Obtains a list of unsigned QR image files in `public/qr`
2. Iterates through the list of unsigned QR image files
    a. Prompts user to select an unsigned QR image file to sign
    b. Displays preview of the selected file to scan with Parity Signer mobile app
        i. On macOS, the Preview app is able to display the `.png` QR image file but is unable to display the `.apng` QR video file for the metadata - this can be opened with Google Chrome
    c. Opens the Camera to scan the signed chain spec or metadata produced by Parity Signer mobile app
    d. Deposits the signed chain spec or metadata as a QR image file in `public/qr`

## Verifier
1. Obtains a list of all QR image files in `public/qr`
2. Iterates through the list of all QR image files
    a. Return an error and exit if the QR image file is unsigned
3. Iterates through the list of all QR image files (second round)
    a. Determines if the QR image file is of content type metadata
        i. Determines if the signature of the metadata QR image file was produced by the private key holder of the public_key provided in `config.toml`
        ii. Returns an error and exits if the signature doesn't match
## Cleaner
1. Obtains a list of of all QR image files in `public/qr`
2. Obtains a list of each metadata QR image file
3. Obtains a list of each chainspec QR image file
4. Fetches the chain specs
5. Instantiates a HashSet to store files to keep
6. Iterates through each chain in `config.toml`
    a. Determines the current metadata version from the chain specs
    b. Determines which metadata QR files are versioned equal to or greater than the current meta data version; these files are kept in the HashSet
    c. Stores Chainspec QR files are kept in the HashSet
7. Determines the difference of all files and kept files, these are the files to be removed
8. Iterates through each file to be removed and delete it

## Check Deployment
1. Generates the contents of a `data.json` and compares the generated file with the hosted `data.json` stored at the root of the homepage specified in `package.json`; for example: https://metadata.frequency.xyz/data.json
2. If generated vs hosted `data.json` mismatch, `exit(12)`

# Github Actions workflows

## Update.yml
Runs daily at 00:00 UTC.
### Jobs
#### update
- Checks for an existing branch prepended with "sign-me" and checks it out if it exists. Otherwise, creates a new branch named "sign-me-<year>-<month>-<day>"
- Determines if there is an updated runtime metadata by querying the RPC node. Also determines if there will be an upcoming runtime upgrade by checking the Frequency github repository assets
- Commits unsigned metadata QR imagefiles to the branch if updates exist and creates a pull request
- Notifies a Matrix channel (if specified) that new metadata is available
- A user is then able to pull the branch, sign the files, commit and merge

#### check-deployment
- Compares hosted https://metadata.frequency.xyz/data.json vs generated spec state from the repository.
- If the generated vs hosted state differ
    - Runs the collector to build anew `data.json`
    - Initiates the deploy workflow to redeploy with the updated `data.json`
