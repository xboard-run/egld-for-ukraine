import fs from "fs";
import Chance from "chance";
import { loadDonorsData } from "./helpers/donorsData";
import { jsonStringifyBigint } from "./helpers/other";
import { loadPrizes, Prize } from "./helpers/prizes";

const main = () => {
  const prizes = loadPrizes();
  const donorsData = loadDonorsData();

  const donors: Donor[] = [];
  for (const donorData of donorsData) {
    const weight = Math.round(Math.sqrt(donorData.donation) * 100);
    donors.push({ address: donorData.address, weight });
  }

  const donorsPrizes: Record<string, Prize[]> = {};
  for (const donor of donors) {
    donorsPrizes[donor.address] = [];
  }

  for (const prize of prizes) {
    const elligibleDonors = donors.filter((donor) => {
      const donorPrizes = donorsPrizes[donor.address];
      return !donorPrizes.some((p) => p.id === prize.id);
    });
    const donor = drawDonor(elligibleDonors);
    donorsPrizes[donor.address].push(prize);
  }

  fs.writeFileSync(
    "prizesDistribution.json",
    jsonStringifyBigint(donorsPrizes, 2)
  );
};

const drawDonor = (donors: Donor[]): Donor => {
  let totalWeight = 0;
  for (const donor of donors) {
    totalWeight += donor.weight;
  }
  const randomNum = chance.integer({ min: 1, max: totalWeight });
  let weightAcc = 0;
  for (const donor of donors) {
    weightAcc += donor.weight;
    if (weightAcc >= randomNum) {
      return donor;
    }
  }
  throw "No donor drawn.";
};

const chance = new Chance(0);

type Donor = { address: string; weight: number };

main();
