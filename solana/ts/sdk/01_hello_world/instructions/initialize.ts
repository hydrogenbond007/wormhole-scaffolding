import {
  Connection,
  PublicKey,
  PublicKeyInitData,
  TransactionInstruction,
} from "@solana/web3.js";
import {
  deriveAddress,
  getWormholeDerivedAccounts,
} from "@certusone/wormhole-sdk/solana";
import { createHelloWorldProgramInterface } from "../program";
import { deriveConfigKey } from "../accounts";

export async function createInitializeInstruction(
  connection: Connection,
  programId: PublicKeyInitData,
  payer: PublicKeyInitData,
  wormholeProgramId: PublicKeyInitData
): Promise<TransactionInstruction> {
  const program = createHelloWorldProgramInterface(connection, programId);
  const wormholeAccounts = getWormholeDerivedAccounts(
    program.programId,
    wormholeProgramId
  );
  return program.methods
    .initialize()
    .accounts({
      owner: new PublicKey(payer),
      config: deriveConfigKey(programId),
      wormholeProgram: new PublicKey(wormholeProgramId),
      ...wormholeAccounts,
    })
    .instruction();
}
