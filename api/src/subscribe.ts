import { GearApi } from '@gear-js/api';
export const events = async () => {
    const gearApi = await GearApi.create();
    
    gearApi.gearEvents.subscribeToProgramEvents(({ method, data: { info, reason } }) => {
        console.log(`
            ${method}:
            programId: ${info.programId.toHex()}
            initMessageId: ${info.messageId.toHex()}
            origin: ${info.origin.toHex()}
            ${reason ? `reason: ${reason.toHuman()}` : ''}
        `);
    });
}

async function main() { 
    await events();
}

main()