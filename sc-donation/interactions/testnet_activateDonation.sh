source interactions/settings_testnet.sh

mxpy --verbose contract call $sc_address \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="T" \
  --proxy="https://testnet-gateway.multiversx.com" \
  --recall-nonce \
  --function=setDonationState \
  --arguments 1 \
  --gas-limit=10000000 \
  --send
