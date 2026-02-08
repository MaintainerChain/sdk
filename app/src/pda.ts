import { PublicKey } from "@solana/web3.js";

export function corePDA(programId: PublicKey) {
  return PublicKey.findProgramAddressSync(
    [Buffer.from("core")],
    programId
  );
}
