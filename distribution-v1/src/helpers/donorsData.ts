import fs from "fs";
import { BigUIntType, U8Type, U64Type, BytesType } from "@elrondnetwork/erdjs";
import { decodeTupleOrFail, numberifyBigint } from "./other";

export const loadDonorsData = (): DonorData[] =>
  JSON.parse(fs.readFileSync(donorsFilePath, "utf-8"));

export const decodeScDonorData = (data: string): ScDonorData => {
  const fields = decodeTupleOrFail(Buffer.from(data, "hex"), [
    new U64Type(),
    new BytesType(),
    new BytesType(),
    new BytesType(),
    new U8Type(),
    new BigUIntType(),
    new U8Type(),
  ]);
  return {
    id: Number(fields[0].value.valueOf().toString(10)),
    pseudo: fields[1].value.valueOf().toString(),
    twitterHandle: fields[2].value.valueOf().toString(),
    message: fields[3].value.valueOf().toString(),
    donationDestinationId: Number(fields[4].value.valueOf().toString(10)),
    donation: numberifyBigint(
      BigInt(fields[5].value.valueOf().toString(10)),
      18
    ),
    lastClaimedTierId: Number(fields[6].value.valueOf().toString(10)),
  };
};

export const donorsFilePath = "donors.json";

export type DonorData = { address: string } & ScDonorData;

type ScDonorData = {
  id: number;
  pseudo: string;
  twitterHandle: string;
  message: string;
  donationDestinationId: number;
  donation: number;
  lastClaimedTierId: number;
};
