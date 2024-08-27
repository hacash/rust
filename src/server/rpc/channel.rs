

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
    let mut data = jsondata!{
        "id", chid.hex(),
        
    };


    api_data(data)
}
