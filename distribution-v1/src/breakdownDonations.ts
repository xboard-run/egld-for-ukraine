import fs from "fs";
import { loadDonorsData } from "./helpers/donorsData";

const main = () => {
  const donorsData = loadDonorsData();
  let totalDonation = 0;
  for (const donorData of donorsData) {
    totalDonation += donorData.donation;
    const destination = findDestinationOrFail(donorData.donationDestinationId);
    destination.donation += donorData.donation;
  }
  const totalDonationStr = totalDonation.toFixed(3);
  const file = fs.createWriteStream("donationsBreakdown.txt", { flags: "w" });
  file.write(`Total donation: ${totalDonationStr} EGLD\n`);
  for (const destination of destinations) {
    const donationStr = destination.donation.toFixed(3);
    const perc = (destination.donation / totalDonation) * 100;
    const percStr = perc.toFixed(2);
    file.write(`- ${destination.name}: ${donationStr} EGLD (${percStr}%)\n`);
  }
};

const findDestinationOrFail = (id: number): Destination => {
  const destination = destinations.find((d) => d.id === id);
  if (!destination) {
    throw "Invalid destination.";
  }
  return destination;
};

const destinations: Destination[] = [
  {
    id: 0,
    name: "Endaoment Ukraine",
    donation: 0,
  },
  {
    id: 1,
    name: "Ukrainian Government",
    donation: 0,
  },
  {
    id: 2,
    name: "Ukraine DAO",
    donation: 0,
  },
  {
    id: 3,
    name: "Unchain.fund",
    donation: 0,
  },
];

type Destination = {
  id: number;
  name: string;
  donation: number;
};

main();
