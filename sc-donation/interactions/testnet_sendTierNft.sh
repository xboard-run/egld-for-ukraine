source interactions/settings_testnet.sh

method="acceptTierNft"

method_enc="0x$(echo -n $method | xxd -p -u | tr -d '\n')"

mxpy --verbose contract call $owner_address \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="T" \
  --proxy="https://testnet-gateway.multiversx.com" \
  --recall-nonce \
  --function=MultiESDTNFTTransfer \
  --arguments $sc_address_enc 1 $collection_id_enc $1 1 $method_enc \
  --gas-limit=10000000 \
  --send
