import fs from "fs";
import { fetchProxyEsdts, jsonStringifyBigint } from "./helpers/other";
import { Prize, prizesFilePath } from "./helpers/prizes";

const main = async () => {
  const esdts = await fetchProxyEsdts(prizesWalletAddress);
  const prizes: Prize[] = [];
  for (const [fullId, data] of Object.entries(esdts)) {
    const parts = fullId.split("-");
    const id = parts.slice(0, 2).join("-");
    const nonce = parseInt(parts[2] ?? "0", 16);
    let totalAmount = BigInt(data.balance);
    const prizeUnit = prizeUnits[id] ?? 1n;
    while (totalAmount > 0) {
      const amount = totalAmount >= prizeUnit ? prizeUnit : totalAmount;
      prizes.push({ id, nonce, amount });
      totalAmount -= amount;
    }
  }
  fs.writeFileSync(prizesFilePath, jsonStringifyBigint(prizes, 2));
};

const prizesWalletAddress =
  "erd1ckqw90h6t08wqdt3zgzjduycl7szc6ex687pvey72e0uze7jvxkq986z0w";

const prizeUnits: Record<string, bigint> = {
  "BSK-baa025": 1_000_000n * 10n ** 16n,
  "EFFORT-a13513": 100n * 10n ** 18n,
  "LKMEX-aab910": 100_000n * 10n ** 18n,
  "LKMEXBET-1385c1": 30n * 10n ** 18n,
  "NUTS-8ad81a": 25_000n * 10n ** 6n,
  "QWT-46ac01": 500n * 10n ** 6n,
  "SUPER-507aa6": 1000n * 10n ** 18n,
  "WATER-9ed400": 1000n * 10n ** 18n,
};

main();
