# metadata-cli

`metadata-cli` is a utility that helps to maintain metadata-portal repo.

It can:

- fetch the metadata from a running node
- help to sign meta\specs QR codes
- remove unused QR codes from metadata-portal repo
- update and sign metadata with provided key (**not recommended**)

## Update and sign metadata at once

The Update&Sign operation was designed in order to avoid manual signing
for more than 50 networks (including parachains, solo chains, test networks).

Update&Sign operation is dangerous because in requires private key passed as a command line argument.

The example of calling Update&Sign command is presented below
```
make updsigner signing-key=<private key starts from 0x>
```
