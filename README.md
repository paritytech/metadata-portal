#  Metadata Portal 🌗

Metadata Portal is a self-hosted web page which shows you the latest metadata for a given network. 

This is an important addition to Signer, which can update the metadata inside only through a special video QR code without going online. 
Parity will host its own version of the page for all chains for which we sign the metadata. 
External users (chain owners) will be able to deploy their own versions of metadata portal if they want.

## How does it work?

It all starts with the Github repository. Any user can clone it and run their Metadata Portal. We also host our own version, so let's break down the principles of working on it.

Metadata Portal supports two metadata sources in parallel. Both are equally important for different types of users.

### 1. Parsing it from chain and generating QR codes itself

This flow is important for all users who want to always have the latest metadata in their signing devices to parse and sign their transactions right away.

- Cron job runs every N hours and checks every known network for the latest metadata version
- If nothing has changed, the process is complete
- If any network has a new version of metadata that has not yet been published on the Metadata Portal, a script is run to generate a special video QR code in Rust
- That newly generated QR code commited into the same repo
- A new page is generated and posted as Github Pages

### 2. Showing manually uploaded and signed QR codes via PRs

This flow is for security-oriented users and Parity itself. It allows chain owners to sign their our own metadata updates and host QR codes for their users.

- Release manager generates new signed QR code manually in an air-gapped environment using his own signing device
- He opens a PR and signs commits by his own YubiKey to prove its validity
- Owner of the repository accepts the PR
- Github action is triggered to regenerate and re-deploy the Github Page

## How to add / remove chain to the portal?
It's configured in `config.toml` file. Add/remove `[[chains]]` section with appropriate fields.

## Where to put signed QRs?
By default, `public/qr/signed`. It's configured by `public_dir_path` field in the config file.

⚠️ File names are meaningful and should follow `<chain>_metadata_<version>.apng` format.

## Development

In the project directory, you can run:

### `yarn start`

Runs the app in the development mode.\
Open [http://localhost:3000](http://localhost:3000) to view it in the browser.
