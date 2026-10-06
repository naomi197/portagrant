import fs from 'node:fs/promises';
import process from 'node:process';
import 'dotenv/config';
import { ApiPromise, WsProvider } from '@polkadot/api';
import { CodePromise } from '@polkadot/api-contract';
import { Keyring } from '@polkadot/keyring';

const ws = process.env.PORTALDOT_WS;
const uri = process.env.DEPLOYER_URI;
if (!ws || !uri) throw new Error('Set PORTALDOT_WS and DEPLOYER_URI in .env');
const wasm = await fs.readFile(process.env.CONTRACT_WASM);
const metadata = JSON.parse(await fs.readFile(process.env.CONTRACT_METADATA, 'utf8'));
const api = await ApiPromise.create({ provider: new WsProvider(ws) });
const keyring = new Keyring({ type: 'sr25519' });
const deployer = keyring.addFromUri(uri);
const code = new CodePromise(api, metadata, wasm);
const gasLimit = api.registry.createType('WeightV2', { refTime: 1_000_000_000, proofSize: 100_000 });
const tx = code.tx.new({ gasLimit, storageDepositLimit: null });
console.log(`Deploying from ${deployer.address} to ${ws}`);
await new Promise((resolve, reject) => tx.signAndSend(deployer, result => {
  if (result.dispatchError) reject(result.dispatchError.toString());
  const record = result.contract && result.contract.address ? { address: result.contract.address.toString(), txHash: result.txHash.toHex() } : null;
  if (record) { console.log(JSON.stringify(record)); resolve(); }
}));
await api.disconnect();