import fs from "fs";

import { isAirdropList } from "./airdrop_list";
import { MerkleTree } from "./merkle";

try {
  var airdrop_list = JSON.parse(fs.readFileSync("airdrop.json", "utf-8"));
} catch (err) {
  console.error(`couldn't read airdrop.json: ${err}`);
  process.exit(1);
}

if (!isAirdropList(airdrop_list)) {
  console.error(
    "invalid file format (`{ address: string; amount: string }[]`)"
  );
  process.exit(1);
}

const tree = new MerkleTree(airdrop_list);

switch (process.argv.at(2)) {
  case "get-root":
    console.log(tree.getMerkleRoot());
    break;

  case "get-proof":
    const address = process.argv.at(3);
    const amount = process.argv.at(4);
    if (address === undefined) {
      console.error("missing `[ADDRESS]`");
      process.exit(1);
    } else if (amount === undefined) {
      console.error("missing `[AMOUNT]`");
      process.exit(1);
    }
    console.log(tree.getMerkleProof({ address, amount }));
    break;

  default:
    console.error(
      "Usage: merkle-cli [get-root]|[get-proof [ADDRESS] [AMOUNT]]`"
    );
    process.exit(1);
}
