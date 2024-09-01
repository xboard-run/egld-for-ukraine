import { BinaryCodec, Tuple, TupleType, Type } from "@elrondnetwork/erdjs";
import fetch from "node-fetch";

export const fetchProxyStorageValues = (
  address: string
): Promise<Record<string, string>> =>
  fetchProxy(`address/${address}/keys`).then((res) => res.data.pairs);

export const fetchProxyEsdts = (
  address: string
): Promise<Record<string, any>> =>
  fetchProxy(`address/${address}/esdt`).then((res) => res.data.esdts);

const fetchProxy = (resource: string) =>
  fetch(`${networkProxyUrl}/${resource}`).then((res) => res.json());

export const decodeTupleOrFail = (buffer: Buffer, types: Type[]) =>
  Codec.decodeTopLevel<Tuple>(buffer, new TupleType(...types)).getFields();

export const numberifyBigint = (n: bigint, decimals: number): number => {
  let nStr = `${n}`.padStart(decimals, "0");
  nStr = nStr.slice(0, -decimals) + "." + nStr.slice(-decimals);
  return Number(nStr);
};

export const jsonStringifyBigint = (value: any, space?: string | number) =>
  JSON.stringify(
    value,
    (_key, value) => (typeof value === "bigint" ? `${value}n` : value),
    space
  );

export const jsonParseBigint = (text: string) =>
  JSON.parse(text, (_key, value) =>
    typeof value === "string" && /^\d+n$/.test(value)
      ? BigInt(value.slice(0, -1))
      : value
  );

const networkProxyUrl = "https://gateway.multiversx.com";

const Codec = new BinaryCodec();
