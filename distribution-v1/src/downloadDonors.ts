import { Address } from "@elrondnetwork/erdjs";
import fs from "fs";
import {
  decodeScDonorData,
  DonorData,
  donorsFilePath,
} from "./helpers/donorsData";
import { fetchProxyStorageValues } from "./helpers/other";

const main = async () => {
  const data = await fetchProxyStorageValues(e4uDonationAddress);
  const startKey = Buffer.from("donors_data").toString("hex");
  const donorsData: DonorData[] = [];
  for (const [key, value] of Object.entries(data)) {
    if (key.startsWith(startKey)) {
      const address = Address.fromHex(key.slice(startKey.length)).toString();
      const scDonorData = decodeScDonorData(value);
      donorsData.push({ address, ...scDonorData });
    }
  }
  donorsData.sort((a, b) =>
    a.donation > b.donation ? -1 : a.donation < b.donation ? 1 : 0
  );
  fs.writeFileSync(donorsFilePath, JSON.stringify(donorsData, null, 2));
};

const e4uDonationAddress =
  "erd1qqqqqqqqqqqqqpgqf3kk27q8ccv39yk72d2wfj7zcscp7kx7tfssthygd0";

main();
