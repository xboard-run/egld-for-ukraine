collection_name="EGLDforUkraine"
collection_ticker="EGLD4UKR"
can_upgrade="canUpgrade"
can_add_special_roles="canAddSpecialRoles"
bool_true="true"

collection_name_enc="0x$(echo -n $collection_name | xxd -p -u | tr -d '\n')"
collection_ticker_enc="0x$(echo -n $collection_ticker | xxd -p -u | tr -d '\n')"
can_upgrade_enc="0x$(echo -n $can_upgrade | xxd -p -u | tr -d '\n')"
can_add_special_roles_enc="0x$(echo -n $can_add_special_roles | xxd -p -u | tr -d '\n')"
bool_true_enc="0x$(echo -n $bool_true | xxd -p -u | tr -d '\n')"

mxpy --verbose contract call erd1qqqqqqqqqqqqqqqpqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqqzllls8a5w6u \
  --keyfile="keyfile.json" \
  --passfile="passfile.txt" \
  --chain="1" \
  --proxy="https://gateway.elrond.com" \
  --recall-nonce \
  --value 50000000000000000 \
  --function=issueSemiFungible \
  --arguments $collection_name_enc $collection_ticker_enc $can_upgrade_enc $bool_true_enc $can_add_special_roles_enc $bool_true_enc \
  --gas-limit=100000000 \
  --send
