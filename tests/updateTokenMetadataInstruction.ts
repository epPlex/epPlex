import { TransactionInstruction, PublicKey, AccountMeta } from "@solana/web3.js";
import * as borsh from "@coral-xyz/borsh";
import { s16, struct, u8  } from '@solana/buffer-layout';
import { publicKey } from '@solana/buffer-layout-utils';


// Invalid instruction at first
// https://explorer.solana.com/tx/4smGbr2G25BYKtccAzrH38hbM1heT9xjSEDKWbEFqYtsWqU9Be6BnUzFRZVL11frJWgxqwTWvBgWWUqhhmzBqsFS?cluster=devnet#ix-5

enum Instruction {
    MetadataPointer = 39
}

enum MetadataInstruction {
    Initialize = 0,
    Update = 1
}

// This requires another instruction since it is nested within the token program within a folder
export interface UpdateTokenMetadataData {
    instruction: Instruction.MetadataPointer;
    metadataPointerInstruction: MetadataInstruction.Initialize
    authority: PublicKey;
    metadataAddress: PublicKey;
}

/** TODO: docs */
export const updateTokenMetadataInstructionData = struct<UpdateTokenMetadataData>([
    u8('instruction'),
    u8('metadataPointerInstruction'),
    publicKey('authority'),
    publicKey('metadataAddress'),
]);


/**
 * Construct an InitializePermanentDelegate instruction
 *
 * @param mint               Token mint account
 * @param authority  Authority that may sign for `Transfer`s and `Burn`s on any account
 * @param metadataAddress  Authority that may sign for `Transfer`s and `Burn`s on any account
 * @param programId          SPL Token program account
 *
 * @return Instruction to add to a transaction
 */
export function updateTokenMetadataInstruction (
    mint: PublicKey,
    authority: PublicKey | null,
    metadataAddress: PublicKey | null,
    programId: PublicKey
): TransactionInstruction {
    const keys = [{ pubkey: mint, isSigner: true, isWritable: true }];

    const data = Buffer.alloc(updateTokenMetadataInstructionData.span);
    updateTokenMetadataInstructionData.encode(
        {
            instruction: Instruction.MetadataPointer,
            metadataPointerInstruction: MetadataInstruction.Initialize,
            authority: authority || new PublicKey(0),
            metadataAddress: metadataAddress || new PublicKey(0),
        },
        data
    );

    return new TransactionInstruction({ keys, programId, data });
}

