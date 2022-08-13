import keccak256 from "keccak256";
import { MerkleTree as MerkleTreeJs } from "merkletreejs";

import { AirdropList } from "./airdrop_list";

export class MerkleTree {
  private tree: MerkleTreeJs;

  constructor(accounts: AirdropList) {
    let leaves = accounts.map((a) => keccak256(a.address + a.amount));
    leaves.sort();
    this.tree = new MerkleTreeJs(leaves, keccak256, { sort: true });
  }

  getMerkleTree() {
    return this.tree;
  }

  getMerkleRoot() {
    return this.tree.getHexRoot().replace("0x", "");
  }

  getMerkleProof(account: { address: string; amount: string }): string[] {
    return this.tree
      .getHexProof(keccak256(account.address + account.amount))
      .map((v) => v.replace("0x", ""));
  }

  verify(proof: string[], account: { address: string; amount: string }) {
    let leaf_terra = keccak256(account.address + account.amount);
    let is_valid = this.tree.verify(proof, leaf_terra, this.tree.getHexRoot());
    return is_valid;
  }
}
