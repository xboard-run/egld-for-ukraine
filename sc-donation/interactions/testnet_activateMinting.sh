source interactions/settings_testnet.sh

erdpy --verbose contract call $sc_address \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="T" \
  --proxy="https://testnet-gateway.elrond.com" \
  --recall-nonce \
  --function=setMintingState \
  --arguments 1 \
  --gas-limit=10000000 \
  --send
