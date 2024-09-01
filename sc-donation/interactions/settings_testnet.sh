source interactions/settings.sh

sc_address="erd1qqqqqqqqqqqqqpgqf3kk27q8ccv39yk72d2wfj7zcscp7kx7tfssthygd0"
collection_id="EGLD4UKR-176180"

sc_address_enc="0x$(erdpy wallet bech32 --decode $sc_address)"
collection_id_enc="0x$(echo -n $collection_id | xxd -p -u | tr -d '\n')"
