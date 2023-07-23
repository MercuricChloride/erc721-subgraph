use crate::{
    pb::schema::{Approvals, Transfers},
    ADDRESS,
};
use substreams::Hex;
use substreams_entity_change::tables::Tables;

pub fn transfers_to_table_changes(tables: &mut Tables, transfers: &Transfers) {
    for transfer in transfers.transfers.iter() {
        // handle the transfer
        let key = format!("{}-{}", transfer.tx_hash, transfer.token_id);
        let row = tables.update_row("Transfer", key);
        row.set("from", transfer.from.clone());
        row.set("to", transfer.to.clone());
        row.set("tokenId", transfer.token_id.clone());

        // handle the accounts
        tables.create_row("Account", transfer.from.clone());
        tables.create_row("Account", transfer.to.clone());

        // handle updating the token owner
        tables
            .update_row("Token", format!("{}", transfer.token_id))
            .set("collection", ADDRESS.to_string())
            .set("owner", transfer.to.clone());
    }
}

pub fn approvals_to_table_changes(tables: &mut Tables, approvals: &Approvals) {
    for approval in approvals.approvals.iter() {
        // handle the approval
        let key = format!("{}-{}", approval.tx_hash, approval.token_id);
        let row = tables.update_row("Approval", key);
        row.set("owner", approval.owner.clone());
        row.set("approved", approval.approved.clone());
        row.set("tokenId", approval.token_id.clone());

        // handle creation of accounts
        tables.create_row("Account", approval.owner.clone());
        tables.create_row("Account", approval.approved.clone());
    }
}

pub fn format_hex(address: &[u8]) -> String {
    format!("0x{}", Hex(address).to_string())
}
