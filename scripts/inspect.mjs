import fs from 'node:fs/promises';
import process from 'node:process';
import 'dotenv/config';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { ContractPromise } from '@polkadot/api-contract';

const api = await ApiPromise.create({ provider: new WsProvider(process.env.PORTALDOT_WS) });
const metadata = JSON.parse(await fs.readFile(process.env.CONTRACT_METADATA, 'utf8'));
const contract = new ContractPromise(api, metadata, process.env.CONTRACT_ADDRESS);
const { result, output } = await contract.query.getGrant(process.env.QUERY_ORIGIN, { gasLimit: -1 }, Number(process.env.GRANT_ID || 0));
if (result.isOk) console.log(JSON.stringify(output?.toHuman() ?? null, null, 2)); else console.error(result.toString());
await api.disconnect();