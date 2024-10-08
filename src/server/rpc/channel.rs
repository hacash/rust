

/******************* channel *******************/



defineQueryObject!{ Q7542,
    id, Option<String>, None,
}

async fn channel(State(ctx): State<ApiCtx>, q: Query<Q7542>) -> impl IntoResponse {
    ctx_mintstate!(ctx, mintstate);
    ctx_mintstore!(ctx, mintstore);
    q_unit!(q, unit);
    q_must!(q, id, s!(""));
    // id

    let Ok(id) = hex::decode(&id) else {
        return api_error("channel id format error")
    };
    if id.len() != ChannelId::width() {
        return api_error("channel id format error")
    }
    let chid = ChannelId::must(&id);
    let Some(channel) = mintstate.channel(&chid) else {
        return api_error("channel not find")
    };

    // return data
    let status = channel.status.uint();
    let mut data = jsondata!{
        "id", chid.hex(),
        "status", status,
        "open_height", channel.open_height.uint(),
        "reuse_version", channel.reuse_version.uint(),
        "arbitration_lock", channel.arbitration_lock_block.uint(),
        "interest_attribution", channel.interest_attribution.uint(),
        "left", json!(jsondata!{
            "address", channel.left_bill.address.readable(),
            "hacash", channel.left_bill.hacsat.amount.to_unit_string(&unit),
            "satoshi", channel.left_bill.hacsat.satoshi.value().uint(),
        }),
        "right", json!(jsondata!{
            "address", channel.right_bill.address.readable(),
            "hacash", channel.right_bill.hacsat.amount.to_unit_string(&unit),
            "satoshi", channel.right_bill.hacsat.satoshi.value().uint(),
        }),
    };

    // if status == 1 // closed  status == 2 || status == 3 
    if let Some(challenging) = channel.if_challenging.if_value() {
        let l_or_r = challenging.assert_address_is_left_or_right.check();
        let assaddr = match l_or_r {
            true => channel.left_bill.address.readable(),
            false => channel.right_bill.address.readable(),
        };
        data.insert("challenging", json!(jsondata!{
            "launch_height", challenging.challenge_launch_height.uint(),
            "assert_bill_auto_number", challenging.assert_bill_auto_number.uint(),
            "assert_address_is_left_or_right", l_or_r,
            "assert_bill", json!(jsondata!{
                "address", assaddr,
                "hacash", challenging.assert_bill.amount.to_unit_string(&unit),
                "satoshi", challenging.assert_bill.satoshi.value().uint(),
            }),
        }));
    }

    // if status == 2 or 3 // closed  status == 2 || status == 3 
    if let Some(distribution) = channel.if_distribution.if_value() {
        data.insert("distribution", json!(jsondata!{
            "hacash", distribution.left_bill.amount.to_unit_string(&unit),
            "satoshi", distribution.left_bill.satoshi.value().uint(),
        }));
    }


    api_data(data)
}
