export type AirdropList = { address: string; amount: string }[];

export function isAirdropList(o: any): o is AirdropList {
  if (!Array.isArray(o)) {
    return false;
  }
  return o
    .map((t) => {
      const res =
        "address" in t &&
        "amount" in t &&
        typeof t.address === "string" &&
        typeof t.amount === "string";
      return res;
    })
    .every((r) => {
      return r === true;
    });
}
