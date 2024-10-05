# Leveraged LRT Lending Protocol

## Overview

This project implements a Leveraged Liquid Re-staking Token (LRT) Lending Protocol on the Solana blockchain. It allows users to stake SOL, receive LRT tokens, and use these tokens as collateral for borrowing. The protocol also includes a lending pool where users can deposit SOL and earn interest.

## Features

- Staking: Users can stake SOL and receive LRT tokens.
- Lending Pool: Users can deposit SOL into the lending pool and receive LP tokens.
- Borrowing: LRT holders can borrow SOL using their LRT as collateral.
- Interest Accrual: Borrowers accrue interest on their loans.
- Repayment: Borrowers can repay their loans with interest.
- Interest Distribution: Lenders earn interest from borrowers' repayments.

## Prerequisites

- Rust 1.19.0 or later
- Solana CLI 1.18.0 or later
- Anchor Framework 0.30.1 or later

## Setup

1. Clone the repository:
   ```
   git clone https://github.com/Ozodimgba/leveraged-lrt-lending.git
   cd llrt
   ```

2. Install dependencies:
   ```
   npm install
   ```

3. Build the project:
   ```
   anchor build
   ```

4. Update the program ID in `lib.rs` and `Anchor.toml` with the new program ID generated during the build.

5. Test the project:
   ```
   anchor test
   ```

## Usage

### Initializing the Lending Pool

To initialize a new lending pool:

```typescript
await program.rpc.initializePool({
  accounts: {
    pool: poolAccount.publicKey,
    lpMint: lpMintAccount.publicKey,
    vault: vaultAccount.publicKey,
    authority: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
    rent: anchor.web3.SYSVAR_RENT_PUBKEY,
  },
});
```

### Staking SOL

To stake SOL and receive LRT tokens:

```typescript
await program.rpc.stake(new anchor.BN(amountToStake), {
  accounts: {
    authority: provider.wallet.publicKey,
    vault: vaultAccount.publicKey,
    lsolMint: lsolMintAccount.publicKey,
    userLsolAccount: userLsolAccount.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  },
});
```

### Depositing to the Lending Pool

To deposit SOL into the lending pool:

```typescript
await program.rpc.deposit(new anchor.BN(amountToDeposit), {
  accounts: {
    pool: poolAccount.publicKey,
    lpMint: lpMintAccount.publicKey,
    vault: vaultAccount.publicKey,
    depositorLpAccount: depositorLpAccount.publicKey,
    depositor: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  },
});
```

### Borrowing

To borrow SOL using LRT as collateral:

```typescript
await program.rpc.borrow(new anchor.BN(amountToBorrow), {
  accounts: {
    pool: poolAccount.publicKey,
    vault: vaultAccount.publicKey,
    borrowerLrtAccount: borrowerLrtAccount.publicKey,
    borrowerPosition: borrowerPositionAccount.publicKey,
    borrower: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
    tokenProgram: TOKEN_PROGRAM_ID,
  },
});
```

### Repaying a Loan

To repay a loan with interest:

```typescript
await program.rpc.repay(new anchor.BN(amountToRepay), {
  accounts: {
    pool: poolAccount.publicKey,
    vault: vaultAccount.publicKey,
    borrowerPosition: borrowerPositionAccount.publicKey,
    borrower: provider.wallet.publicKey,
    systemProgram: anchor.web3.SystemProgram.programId,
  },
});
```

## Security Considerations

- This protocol handles financial transactions and should undergo thorough security audits before deployment to mainnet.
- Ensure proper access controls and input validation in all instructions.
- Implement safeguards against potential arithmetic overflows.
- Consider implementing a pause mechanism for emergency situations.

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details. *wink