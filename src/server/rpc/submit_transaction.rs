
defineQueryObject!{ Q4396,
    __nnn__, Option<bool>, None,
}

async fn submit_transaction(State(ctx): State<ApiCtx>, q: Query<Q4396>, body: Bytes) -> impl IntoResponse {
    // body bytes
    let bddts = q_body_data_may_hex!(q, body);
    // println!("get tx body: {}", hex::encode(&bddts));
    // parse
    let txpkg = transaction::create_pkg( BytesW4::from_vec(bddts) );
    if let Err(e) = txpkg {
        return api_error(&format!("transaction parse error: {}", &e))
    }
    let txpkg = txpkg.unwrap();
    // try submit
    let is_async = true;
    if let Err(e) = ctx.hcshnd.submit_transaction(&txpkg, is_async) {
        return api_error(&e)
    }
    // ok
    api_ok()
}

/*


02
0066d52c90
00674e11e34c472ebfba2d34528fccd8aba826f2c4
f40101
0001
0001
00681990afd226b1cbc6c5f085cfdc2092d0843241
f60101
0001
037bb06e880a8afb03f4035bdcd9354e798a0cbdee613bebe17d2c8db14f0eb734bf
b88afa0fefd6e5f4135ad03cc09b4c406ea4c94397b60ff3da7a1b48876a876853702c5a07bb3d9c5b4509d8dd91059a3985a2627063a4d6fa205e559111bb
0000


020066d5340c00674e11e34c472ebfba2d34528fccd8aba826f2c4f401010001000100681990afd226b1cbc6c5f085cfdc2092d0843241f6010100000000

*/