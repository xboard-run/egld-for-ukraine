import fs from "fs";
import { jsonParseBigint } from "./other";

export const loadPrizes = (): Prize[] =>
  jsonParseBigint(fs.readFileSync(prizesFilePath, "utf-8"));

export const prizesFilePath = "prizes.json";

export type Prize = {
  id: string;
  nonce: number;
  amount: bigint;
};
