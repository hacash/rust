

impl RPCServer {


    fn route_rpc(&self, app: &mut Router, pathkind: &str) {

        // app.get(pathkind, query);
    }

    /*
    fn app_router(rpc: Arc<RPCServer>) -> Router {
        let app = Router::new();
        
        // stable rpc
        // app.get("/", console);

        let ctx = rpc.clone();
        app.clone().route("/query", Route::new().get(|req| async move { 
            rpc::balance(ctx.as_ref(), req).await
        }));

        // self.route_rpc(&mut app, "/query",);


        // unstable api

        // ok
        app.clone()
    }
    */


}