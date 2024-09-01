erdpy --verbose contract deploy \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="1" \
  --proxy="https://gateway.elrond.com" \
  --recall-nonce \
  --bytecode="sc-donation/output/donation.wasm" \
  --arguments 100000000000000000 3 \
  --gas-limit=50000000 \
  --send
