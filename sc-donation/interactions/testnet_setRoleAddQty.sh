source interactions/settings_testnet.sh

role="ESDTRoleNFTAddQuantity"

role_enc="0x$(echo -n $role | xxd -p -u | tr -d '\n')"

mxpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="T" \
  --proxy="https://testnet-gateway.multiversx.com" \
  --recall-nonce \
  --function=setSpecialRole \
  --arguments $collection_id_enc $sc_address_enc $role_enc \
  --gas-limit=100000000 \
  --send
