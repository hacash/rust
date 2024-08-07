
use reqwest::blocking::Client as HttpClient;

const RPC_URL: &str = "http://127.0.0.1:18081/";



pub fn rpc_test() {
    rpc_test_1();
}


pub fn rpc_test_1() {


    // raise fee
    let txbody = hex::decode("").unwrap();

    let res = HttpClient::new().post(RPC_URL.to_owned()+"operate/fee/raise?fee=2:244&fee_prikey=")
    .body(txbody)
    .send().unwrap().text().unwrap();

    println!("{}", res);

}