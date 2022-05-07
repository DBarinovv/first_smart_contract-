"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
Object.defineProperty(exports, "__esModule", { value: true });
const api_1 = require("@gear-js/api");
const fs_1 = require("fs");
require('dotenv').config();
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        const gearApi = yield api_1.GearApi.create();
        const jsonKeyring = fs_1.readFileSync('/Users/louise/smart-contract-academy/01-hello-world/api/keys.json').toString();
        const account = api_1.GearKeyring.fromJson(jsonKeyring, '123456');
        console.log("start deploying program");
        let program = yield uploadProgram(gearApi, process.env.OPT_WASM || "", process.env.META_WASM, account, 0, 0x00);
        console.log("Hello Program ID:", program.programId);
    });
}
const uploadProgram = (api, pathToProgram, pathToMeta, account, value, initPayload) => __awaiter(void 0, void 0, void 0, function* () {
    const code = fs_1.readFileSync(pathToProgram);
    const metaFile = pathToMeta ? fs_1.readFileSync(pathToMeta) : undefined;
    const meta = metaFile ? yield api_1.getWasmMetadata(metaFile) : undefined;
    const gas = yield api.program.gasSpent.init(account.address, code, initPayload, value, meta);
    console.log("GAS SPENT", gas.toHuman());
    const programId = api.program.submit({ code, initPayload, gasLimit: gas }, meta);
    yield api.program.signAndSend(account, (data) => {
        console.log(data.toHuman());
    });
    return programId;
});
main();
//# sourceMappingURL=deploy_program.js.map