source interactions/settings_mainnet.sh

mxpy --verbose contract call $sc_address \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="1" \
  --proxy="https://gateway.elrond.com" \
  --recall-nonce \
  --function=setDonationState \
  --arguments 1 \
  --gas-limit=10000000 \
  --send
