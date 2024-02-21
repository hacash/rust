
#[wasm_bindgen]
pub fn hac_to_mei(amount: String) -> String {
    let mins = Amount::new_small(1, 240);
    let res3 =  Amount::from_string_unsafe(&amount);
    if let Ok(hac) = res3 {
        if hac.less_than(&mins) {
            return "≈0.00000001".to_string()
        }else{
            let zhu = hac.to_zhu_unsafe();
            return (zhu / 100000000.0).to_string()

        }
        return hac.to_mei_string_unsafe()
    }
    // error
    return "[ERROR]".to_string()
}


