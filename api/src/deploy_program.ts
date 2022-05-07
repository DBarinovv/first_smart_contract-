import { GearApi, GearKeyring, getWasmMetadata } from '@gear-js/api';
import { readFileSync } from 'fs';

require('dotenv').config();

async function main() {
    const gearApi = await GearApi.create();
    const jsonKeyring = readFileSync('/Users/louise/smart-contract-academy/01-hello-world/api/keys.json').toString();
    const account = GearKeyring.fromJson(jsonKeyring, '123456');
    console.log("start deploying program");
    
    let program = await uploadProgram(
        gearApi,
        process.env.OPT_WASM || "",
        process.env.META_WASM,
        account,
        0,
        0x00
    )
    console.log("Hello Program ID:", program.programId);
}

const uploadProgram = async (
    api: GearApi, 
    pathToProgram: string, 
    pathToMeta?: string,
    account?: any, 
    value?: any, 
    initPayload?: any) => {
    const code = readFileSync(pathToProgram);
    const metaFile = pathToMeta ? readFileSync(pathToMeta) : undefined;
    const meta = metaFile ? await getWasmMetadata(metaFile) : undefined;

    const gas = await api.program.gasSpent.init(
        account.address,
        code,
        initPayload,
        value, 
        meta
    );
    console.log("GAS SPENT", gas.toHuman());

    const programId = api.program.submit({ code, initPayload, gasLimit: gas }, meta);
    await api.program.signAndSend(account, (data) => {
        console.log(data.toHuman());
    });

    return programId;
};

main()
