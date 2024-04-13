

/**
 * create vm action
 */
pub fn create(buf: &[u8]) -> Ret<(Box<dyn VMAction>, usize)> {
    // println!("create vm action {}", hex::encode(buf));
    let cds = buf_clip_mvsk!(buf, 1);
    let code = cds[0] as u8;
    if code == 0xFE {
        // Abort Error
    }
    // try extend actions
    let kid = protocol::action::cut_kind(buf) ? ;
    // core
    let extactobj1 = protocol::action::try_create_vm(kid, buf) ? ;
    if let Some(res) = extactobj1 {
        return Ok(res)
    }
    // mint
    let mut extactobj2 = mint::action::try_create_vm(kid, buf) ? ;
    if let Some(res) = extactobj2 {
        return Ok(res)
    }
    // not find
    errf!("cannot find any action by code {} or kind {}", code, kid)

}

/*
00010058b9ceaea0e0bd4cfcca96ef2cec052234a5e6d3f401010001039ffe91e6b39c21c32d282272ce83ce852d021cd34044181e94dad754b0f7c7d5eaf574dd8ca9cdd83b14fb12ad86749dc7bf569550c7ad486158a6598946231a3612b343603f69811ff7b4c3977ee6faaae383626de1be53a3d8f9e38676d82c0000

000100dfcb797888073a08166b42c12eeb136ec6aa7f47f501050001039ffe91e6b39c21c32d282272ce83ce852d021cd34044181e94dad754b0f7c7d59f52976d776230764b1440a3e4a7c678d2391a4d8e12d56d300f55f6630a028820f001a40635bb7eb7e0dd5e0450c095a08fb7d6beb23dc723c63472c625c5de0000

0001002d4f9d5b1b5c655d2c1b5aeb069775c8383c802ff80107000102f8b50ac4d94b6edc65350282636667f020f8f570d66f5bfc257ad177f36aba2fb9bc040583e0004576fa0065d3087273fac801e1d35aabc4459a95ed006d2d9e78ba7e8557c3746fdf175377e54c692780083ad54ca40112cd59dd2a6a18c7610000

01005c6fefc4000c1fa1c032d90fd7afc54deb03941e87b4c59756f4010100 01

00010058b9ceaea0e0bd4cfcca96ef2cec052234a5e6d3f601010001039ffe91e6b39c21c32d282272ce83ce852d021cd34044181e94dad754b0f7c7d569346d85c3e589c05fd8c18249b811bed76096e81ebcf13e43c67960d1922a0422fe26f886794c1933b6ff3648ac6bb034a047842de7d6a8fa33ad942783a4ff0000


*/



