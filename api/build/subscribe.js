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
exports.events = () => __awaiter(void 0, void 0, void 0, function* () {
    const gearApi = yield api_1.GearApi.create();
    gearApi.gearEvents.subscribeToProgramEvents(({ method, data: { info, reason } }) => {
        console.log(`
            ${method}:
            programId: ${info.programId.toHex()}
            initMessageId: ${info.messageId.toHex()}
            origin: ${info.origin.toHex()}
            ${reason ? `reason: ${reason.toHuman()}` : ''}
        `);
    });
});
function main() {
    return __awaiter(this, void 0, void 0, function* () {
        yield exports.events();
    });
}
main();
//# sourceMappingURL=subscribe.js.map
