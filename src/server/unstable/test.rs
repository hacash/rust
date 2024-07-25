


////////////////// test //////////////////




defineQueryObject!{ Q86489,
    name, Option<String>, None,
}

async fn testapi1234563847653475(State(ctx): State<ApiCtx>, q: Query<Q86489>) -> impl IntoResponse {

    let mut data = jsondata!{
        "test", 1,
    };
    api_data(data)
}

