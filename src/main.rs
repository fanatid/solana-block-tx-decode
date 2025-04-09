use {
    prost::{
        Message,
        bytes::Buf,
        encoding::{
            DecodeContext, WireType, check_wire_type, decode_key, decode_varint, skip_field,
        },
    },
    solana_storage_proto::convert::generated,
    solana_transaction_status::{ConfirmedBlock, TransactionWithStatusMeta},
    std::time::Instant,
};

fn main() -> anyhow::Result<()> {
    for (block, tx_index) in [("332254855", 42), ("332254908", 42)] {
        let bytes = std::fs::read(format!("./blocks/{block}"))?;
        test_perf(block, &bytes, tx_index)?;
    }

    Ok(())
}

fn test_perf(prefix: &str, bytes: &[u8], tx_index: usize) -> anyhow::Result<()> {
    let ts = Instant::now();
    let block = generated::ConfirmedBlock::decode(bytes)?;
    println!(
        "{prefix} | block height {} | solana/prost: {:?}",
        block.block_height.as_ref().map(|v| v.block_height).unwrap(),
        ts.elapsed()
    );
    let block = ConfirmedBlock::try_from(block)?;
    println!(
        "{prefix} | block height {} | solana/prost+convert(agave): {:?}",
        block.block_height.unwrap(),
        ts.elapsed()
    );

    let ts = Instant::now();
    let block_height = get_block_height(bytes)?.unwrap();
    println!(
        "{prefix} | block height {block_height} | manual: {:?}",
        ts.elapsed()
    );

    let ts = Instant::now();
    let tx = TransactionWithStatusMeta::try_from(
        generated::ConfirmedBlock::decode(bytes)?
            .transactions
            .remove(tx_index),
    )?;
    println!(
        "{prefix} | tx {tx_index} {} | solana/prost: {:?}",
        tx.transaction_signature(),
        ts.elapsed()
    );

    let ts = Instant::now();
    let tx = ConfirmedBlock::try_from(generated::ConfirmedBlock::decode(bytes)?)?
        .transactions
        .remove(tx_index);
    println!(
        "{prefix} | tx {tx_index} {} | solana/prost+convert(agave): {:?}",
        tx.transaction_signature(),
        ts.elapsed()
    );

    let ts = Instant::now();
    let tx = get_tx(bytes, tx_index)?.unwrap();
    println!(
        "{prefix} | tx {tx_index} {} | manual: {:?}",
        tx.transaction_signature(),
        ts.elapsed()
    );

    Ok(())
}

fn get_block_height(mut buf: impl Buf) -> anyhow::Result<Option<u64>> {
    while buf.has_remaining() {
        let (tag, wire_type) = decode_key(&mut buf)?;

        if tag == 7 {
            check_wire_type(WireType::LengthDelimited, wire_type)?;
            let len = decode_varint(&mut buf)? as usize;
            anyhow::ensure!(len <= buf.remaining(), "buffer underflow");
            let value = generated::BlockHeight::decode(buf.take(len))?;
            return Ok(Some(value.block_height));
        }

        skip_field(wire_type, tag, &mut buf, DecodeContext::default())?;
    }

    Ok(None)
}

fn get_tx(mut buf: impl Buf, tx_index: usize) -> anyhow::Result<Option<TransactionWithStatusMeta>> {
    let mut counter = 0;
    while buf.has_remaining() {
        let (tag, wire_type) = decode_key(&mut buf)?;

        if tag == 4 {
            if counter == tx_index {
                check_wire_type(WireType::LengthDelimited, wire_type)?;
                let len = decode_varint(&mut buf)? as usize;
                anyhow::ensure!(len <= buf.remaining(), "buffer underflow");
                let value = generated::ConfirmedTransaction::decode(buf.take(len))?;
                return Ok(Some(TransactionWithStatusMeta::try_from(value)?));
            } else {
                counter += 1;
            }
        }

        skip_field(wire_type, tag, &mut buf, DecodeContext::default())?;
    }

    Ok(None)
}
