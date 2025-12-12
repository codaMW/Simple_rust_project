#[derive(Debug)]

struct Inputs {
    version: u8,
    txid: String,
    in_script: String,
    nsequence: String,
}

#[derive(Debug)]

struct Outputs {
    count: u8,
    amount: f64,
    out_script: String,
}

#[derive(Debug)]

struct Transactions {
    inputs: Inputs,
    outputs: Outputs,
}

#[derive(Debug)]

struct Block {
    height: u64,
    status: String,
    timestamp: String,
    size: String,
    virtual_size: String,
    weight_units: String,
    tx: Transactions,
}

#[allow(unused)]

fn main() {
    let input1 = Inputs {
        version: 1,
        txid: String::from("75464be7e6715f15bb1d50f5471e6b1e6f89587e7925a72b12098acff467dfa1"),
        in_script: String::from(
            "304402206bc3998fb5da716c768c88d9f5c7aa53394e5f30756394830195a3b0e2103be602206c871203e2c969af324b81ee881c9d992b7943f1bc6bcd129515e3f2d258071b01 0316f4c293a7df462ea8178160f4c37dcde2256d76fff0476c01a18e89412919be",
        ),
        nsequence: String::from("0xfffffffd"),
    };

    let output1 = Outputs {
        count: 1,
        amount: 0.00452629,
        out_script: String::from("0014bf6e36255aa87e147c698d61c3f68a715df43088"),
    };

    let trans1 = Transactions {
        inputs: Inputs {
            version: input1.version,
            txid: input1.txid,
            in_script: input1.in_script,
            nsequence: input1.nsequence,
        },
        outputs: Outputs {
            count: output1.count,
            amount: output1.amount,
            out_script: output1.out_script,
        },
    };

    let block1 = Block {
        height: 10,
        status: String::from("in best chain (827580 confirmations)"),
        timestamp: String::from("2025-12-12 16:49:43 GMT +2"),
        size: String::from("0.97kB"),
        virtual_size: String::from("1vkB"),
        weight_units: String::from("3.828KWU"),
        tx: Transactions {
            inputs: trans1.inputs,
            outputs: trans1.outputs,
        },
    };

    println!("\n\nBitcoin Blockchain Simulator using the concept of structs\n\n");

    println!("{:#?}", block1);
}
