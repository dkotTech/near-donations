# near-donations
Web3 donations for near protocol

# Build
```
$ cd near-contract
$ ./build.sh
```

# Deployment
```
$ cd near-contract
$ ./build.sh
$ near deploy --wasmFile res/donations_alert.wasm --accountId <Contract Account ID>
$ near call <Contract Account ID> new '{"beneficiary":"<Account ID>"}' --accountId <Contract Account ID>
```

# Examples
[Donation page](https://qawsew.testnet.page/)

[Stats page](https://qawsew.testnet.page/stats)

[Alerts](https://qawsew.testnet.page/alerts?account_id=qawsew.testnet)

[Alerts start from new donation](https://qawsew.testnet.page/alerts?account_id=qawsew.testnet&to_last=true)

[Alerts start from donation id](https://qawsew.testnet.page/alerts?account_id=qawsew.testnet&donation_id=11)
