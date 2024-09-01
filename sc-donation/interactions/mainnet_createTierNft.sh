source interactions/settings_mainnet.sh

json=`cat "ipfs/$1.json"`
regex='name": "([^"]*)"'
[[ $json =~ $regex ]]
nft_name=${BASH_REMATCH[1]}
nft_attributes="metadata:$cid/$1.json"
nft_png_uri="https://ipfs.io/ipfs/$cid/$1.png"
nft_json_uri="https://ipfs.io/ipfs/$cid/$1.json"

nft_name_enc="0x$(echo -n $nft_name | xxd -p -u | tr -d '\n')"
nft_attributes_enc="0x$(echo -n $nft_attributes | xxd -p -u | tr -d '\n')"
nft_png_uri_enc="0x$(echo -n $nft_png_uri | xxd -p -u | tr -d '\n')"
nft_json_uri_enc="0x$(echo -n $nft_json_uri | xxd -p -u | tr -d '\n')"

mxpy --verbose contract call $owner_address \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="1" \
  --proxy="https://gateway.elrond.com" \
  --recall-nonce \
  --function=ESDTNFTCreate \
  --arguments $collection_id_enc 1 $nft_name_enc 0 0 $nft_attributes_enc $nft_png_uri_enc $nft_json_uri_enc \
  --gas-limit=10000000 \
  --send
