/// Encode a transaction receipt into bytes.
///
/// According to [`EIP-2718`]:
/// - `Receipt` is either `TransactionType || ReceiptPayload` or
///   `LegacyReceipt`.
/// - `LegacyReceipt` is kept to be RLP encoded bytes; it is `rlp([status,
///   cumulativeGasUsed, logsBloom, logs])`.
/// - `ReceiptPayload` is an opaque byte array whose interpretation is dependent
///   on the `TransactionType` and defined in future EIPs.
///   - As [`EIP-2930`] defined: if `TransactionType` is `1`, `ReceiptPayload`
///     is `rlp([status, cumulativeGasUsed, logsBloom, logs])`.
///   - As [`EIP-1559`] defined: if `TransactionType` is `2`, `ReceiptPayload`
///     is `rlp([status, cumulative_transaction_gas_used, logs_bloom, logs])`.
///
/// [`EIP-2718`]: https://eips.ethereum.org/EIPS/eip-2718#receipts
/// [`EIP-2930`]: https://eips.ethereum.org/EIPS/eip-2930#parameters
/// [`EIP-1559`]: https://eips.ethereum.org/EIPS/eip-1559#specification
pub fn encode_receipt(r: &TxResp, logs_bloom: Bloom) -> Bytes {
    // Status: either 1 (success) or 0 (failure).
    // Only present after activation of [EIP-658](https://eips.ethereum.org/EIPS/eip-658)
    let status: u64 = if matches!(r.exit_reason, ExitReason::Succeed(_)) {
        1
    } else {
        0
    };
    let used_gas = U256::from(r.gas_used);
    let legacy_receipt = {
        let mut rlp = RlpStream::new();
        rlp.begin_list(4);
        rlp.append(&status);
        rlp.append(&used_gas);
        rlp.append(&logs_bloom);
        rlp.append_list(&r.logs);
        rlp.out().freeze()
    };
    match self.type_() {
        x if x == 0x01 || x == 0x02 => [&x.to_be_bytes()[7..], &legacy_receipt].concat().into(),
        _ => legacy_receipt, // legacy (0x00) or undefined type
    }
}
