use std::fs::OpenOptions;
use std::io::Write;

use http_req::request;

use crate::interface::field::*;
use crate::base::field::*;

/*
01000003706600605b1ea2000000000d180d43910569efb6bdf8217eb2030d6b3037b273215b775ced30a27b6e8eeb47ff0f4277e2d5d351db0a2568a54d99adb2e34289fd0105154c71d300000003d8f2b1d7dc8715320000000070b611414ff374eec4e06c5e351f2920bd8288b3f801022e6f726700000000000000000000000e010000000000000000000000000000000000000000000000000000000000000000000200605b1ddc0062db6289b346cca169716b463ad07a414d01b398f40101000100010039110e91ef80d2581dacd215dd8dfcc17d45b5ccfb0102000102f154fad8364e7c8b0dd226e103e26d257dfdaf5193679a27df5469adeef083bf555f124f15892ad2bed28b474fe5944a673aeb05b3db753ea2787fe3b3eb8c5b4061802ad7713e0eb04fe85ff81c5c2bb4dc906da0dfec54a724dba6b4e991ef00000200605b1e3300d33089fa07e4615900a486a004f6a6145528a3e6f401010001000100e991491d3cc854c677d32c05848a6e56cb2c23daf90123000103c1373f79cc7a18f3da23d7f172719bb97f95d33593a8b5620af50e02c09f4cd5d846351a4008b8a8273a20773179de5772151b63c6a4ff352fa3ee6cbbc8fa610df1789cd0bb38534365321a090721ace9e4f842b3727b4380440ca2c8530a740000

01000003706600605b1ea2000000000d180d43910569efb6bdf8217eb2030d6b3037b273215b775ced30a27b6e8eeb47ff0f4277e2d5d351db0a2568a54d99adb2e34289fd0105154c71d300000003d8f2b1d7dc8715320000000070b611414ff374eec4e06c5e351f2920bd8288b3f801022e6f726700000000000000000000000e010000000000000000000000000000000000000000000000000000000000000000000200605b1ddc0062db6289b346cca169716b463ad07a414d01b398f40101000100010039110e91ef80d2581dacd215dd8dfcc17d45b5ccfb0102000102f154fad8364e7c8b0dd226e103e26d257dfdaf5193679a27df5469adeef083bf555f124f15892ad2bed28b474fe5944a673aeb05b3db753ea2787fe3b3eb8c5b4061802ad7713e0eb04fe85ff81c5c2bb4dc906da0dfec54a724dba6b4e991ef00000200605b1e3300d33089fa07e4615900a486a004f6a6145528a3e6f401010001000100e991491d3cc854c677d32c05848a6e56cb2c23daf90123000103c1373f79cc7a18f3da23d7f172719bb97f95d33593a8b5620af50e02c09f4cd5d846351a4008b8a8273a20773179de5772151b63c6a4ff352fa3ee6cbbc8fa610df1789cd0bb38534365321a090721ace9e4f842b3727b4380440ca2c8530a740000

01000003706500605b1d6a000000000a4ac557c4f0fb5b01affa0dc3a639752726b102ca294592b901b561f7fc95464d81e855d5a1020e1b16e28544d879c2478a9222f6030dc44bac6ba200000001e27bb919dc87153200000000ab9488bb8721b7c5d9447f2d239c0cc757409cfff8010248656c6978000000000000000000000400

01
0000037066
00605b1ea2
000000000d180d43910569efb6bdf8217eb2030d6b3037b273215b775ced30a2
7b6e8eeb47ff0f4277e2d5d351db0a2568a54d99adb2e34289fd0105154c71d3
00000003 // tx num
d8f2b1d7
dc871532
0000 // block intro end

00 // tx type
0070b611414ff374eec4e06c5e351f2920bd8288b3f8
01022e6f726700000000000000000000000e01000000000000000000000000000000000000000000000000000000000000000000000000000000000e010000000000000000000000000000000000000000000000000000000000000000000200605b1ddc0062db6289b346cca169716b463ad07a414d01b398f40101000100010039110e91ef80d2581dacd215dd8dfcc17d45b5ccfb0102000102f154fad8364e7c8b0dd226e103e26d257dfdaf5193679a27df5469adeef083bf555f124f15892ad2bed28b474fe5944a673aeb05b3db753ea2787fe3b3eb8c5b4061802ad7713e0eb04fe85ff8

*/


/**
 * req block data from other node by rpc url
 */
 pub fn engine_test_3(engine: Arc<BlockEngine>) {

    // download_block_bytes();
    // return;


    let bytes = std::fs::read("./allblockbytes.data").unwrap();
    let ptrs = std::fs::read("./allblockptrs.data").unwrap();

    println!("block data file len {} {}", bytes.len(), ptrs.len());
    // println!("block data = {}  ptr = {}", hex::encode(&bytes[..262]), hex::encode(&ptrs[0..32]));

    let maxhei: usize = 540920;
    // cur hei
    let mut height: usize = 0;
    let stoptr = engine.store();
    let store = CoreStoreDisk::wrap(stoptr.as_ref());
    let last = store.status();
    let lhei = last.last_height.to_usize();
    height = lhei + 1; // next hei
    // data offset
    let mut ostleft: usize = 0;

    loop {
        // next block
        // println!("height {}", height);
        // read
        let mut ptl = (height-1) * 8;
        if ptl > ptrs.len()-8 {
            break // all end
        }
        let ost = Uint4::from_bytes(ptrs[ptl..ptl+4].try_into().unwrap()).to_usize();
        ptl += 4;
        let bsz = Uint4::from_bytes(ptrs[ptl..ptl+4].try_into().unwrap()).to_usize();
        // println!("height {} ost {} size {}", height, ost, bsz);
        let blkdts = BytesW4::from_vec(bytes[ost .. ost+bsz].to_vec());
        ostleft = ost;
        // println!("hei {} blkdts {}", height, hex::encode(&blkdts));
        // create
        let pkg = match protocol::block::create_pkg(blkdts) {
            Err(e) => {
                println!("create_pkg() height {} error: {}", height, e);
                break;
            },
            Ok(pkg) => pkg
        };
        // println!("serialize {} blkdts {}", height, hex::encode(pkg.objc().serialize()));

        let isthei = pkg.objc().height().to_u64();
        if let Err(e) = engine.insert(pkg) {
            println!("engine.insert() height {} isthei {} error: {}", height, isthei, e);
            break;
        }

        // ok 
        if height % 1000 == 0 {
            println!("insert height {}", height);
        }
        if height > maxhei {
            break // all ok
        }

        // next
        height += 1;
        
    }

    println!("all {} blocks insert ok.", height)




 }


/**
 * req block data from other node by rpc url
 */
pub fn engine_test_2(engine: Arc<BlockEngine>) {

    let mut height = 1;
    let stoptr = engine.store();
    let store = CoreStoreDisk::wrap(stoptr.as_ref());
    let last = store.status();
    let lhei = last.last_height.to_u64();
    height = lhei + 1; // next hei

    loop {

        let url = format!("http://127.0.0.1:33381/query?action=blockdatahex&body=1&id={}", height);
        let mut body = Vec::new();
        let res = request::get(url, &mut body).unwrap();
        let blkdts = hex::decode(body).unwrap();
        if height % 500 == 0 {
            println!("{}", height);
        }
        // println!("Status: {} {}", res.status_code(), res.reason());
        // println!("create block {} {}", height, hex::encode(blkdts.to_vec()));
        // let blkdts = b"";
        let blkdts = BytesW4::from_vec(blkdts.to_vec());
        let pkg = match protocol::block::create_pkg(blkdts) {
            Err(e) => {
                println!("create_pkg() height {} error: {}", height, e);
                break;
            },
            Ok(pkg) => pkg
        };
        if let Err(e) = engine.insert(pkg) {
            println!("engine.insert() height {} error: {}", height, e);
            break;
        }
        // next 
        height += 1;
    }





}




pub fn engine_test_1(engine: Arc<BlockEngine>) {

    // //  1549280970
    // create_block(3, 1549280971, "1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6");
    // return
    


    engine.print_roller();

    let blks = _test_blocks();

    for blk in blks {

        println!("\n---------------------------------------\n");

        let pkg = BlockPackage::new(Box::new(blk));
        if let Err(e) = engine.insert(Box::new(pkg)) {
            println!("{}", e);
        }
        engine.print_roller();
    }

    // delete datadir
    // std::fs::remove_dir_all("./hacash_mainnet_data");


}

fn block_bytes_write_to_file(data: Vec<u8>, ptr: Vec<u8>) {
    // truncate append
    let mut file1 = OpenOptions::new().write(true).append(true).open("./allblockbytes.data").unwrap();
    let _ = file1.write_all(&data);
    let mut file2 = OpenOptions::new().write(true).append(true).open("./allblockptrs.data").unwrap();
    let _ = file2.write_all(&ptr);

}

pub fn download_block_bytes() {

    let mut file1 = OpenOptions::new().write(true).truncate(true).open("./allblockbytes.data").unwrap();
    let _ = file1.write_all(&[]);
    let mut file2 = OpenOptions::new().write(true).truncate(true).open("./allblockptrs.data").unwrap();
    let _ = file2.write_all(&[]);

    let mut height = 1;
    let mut databuf: Vec<Vec<u8>> = vec![];
    let mut dptrnum = Uint4::from(0);
    let mut dataptr: Vec<Vec<u8>> = vec![];
    loop {

        let url = format!("http://127.0.0.1:33381/query?action=blockdatahex&body=1&id={}", height);
        let mut body = Vec::new();
        if let Ok(res) = request::get(url, &mut body) {} else {
            break // end
        };
        if let Ok(blkdts) = hex::decode(&body) {
            let blklen = blkdts.len();
            if blklen == 0 {
                break // end
            }
            
            databuf.push(blkdts);
            dataptr.push( dptrnum.serialize() ); // ost
            dataptr.push( Uint4::from(blklen as u32).serialize() ); // size
            dptrnum += blklen as u64;
            // println!("dptrnum {} {} {} {}", height, blklen, hex::encode(dptrnum.serialize()), hex::encode(blkdts));
            
        } else {
            break // end
        };
        if height % 1000 == 0 {
            println!("{}", height);
            block_bytes_write_to_file(databuf.concat(), dataptr.concat());
            databuf.clear();
            dataptr.clear();
        }
        // next
        height += 1;
        if height > 30 {
            // break
        }
    }

    // end
    block_bytes_write_to_file(databuf.concat(), dataptr.concat());
    databuf.clear();
    dataptr.clear();

    println!("all blocks {} ok.", height)
}


fn _test_blocks() -> Vec<BlockV1> {
    let datas = vec![
        // hei: 1, hash: 001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb56
        "010000000001005c57b130000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e64448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f30000000100000516fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 2, hash: 000c0a2a3761fec7aa214975c1cce407b509a828d16dcf6d3bdb1f612a2466f5
        "010000000002005c57b2e6001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb564448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000007adfffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",


        // hei: 1, hash: b3e25014147b2ee7232323259a86955a2779fa442d0ba86444de4eaa3f392a93
        "010000000001005c57b0ef000000077790ba2fcdeaef4a4299d9b667135bac577ce204dee8388f1b97f7e6ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        // hei: 2, hash: 1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6
        "010000000002005c5826ca001e231cb03f9938d54f04407797b8188f0375eb10f0bcb426dccae87dcadb56ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        // hei: 3, hash: 3a11fee226605b7b760205d15c56c3699dcb25cf442ffe08124beda8fd3814a8
        "010000000003005c5826cb1e78eaa957a162e1b80caa856670fa0df3ba16e694a8edbfc057b71c15fd0ae6ad557702fc70afaf70a855e7b8a4400159643cb5a7fc8a89ba2bce6f818a9b0100000001098b344500000000000000000c1aaa4e6007cc58cfb932052ac0ec25ca356183f80101686172646572746f646f62657474657200",
        

        // hei: 3, hash: 0015920ecbd8048128b9e27a26bd08b488050c78b89291d740889ed4d785f410
        "010000000003005c57b3f3000c0a2a3761fec7aa214975c1cce407b509a828d16dcf6d3bdb1f612a2466f54448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000037afffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 4, hash: 000e9a58542cc77a2442721d839c23be67549940dbe3f0e84e18fb1630d90ae4
        "010000000004005c57b52d0015920ecbd8048128b9e27a26bd08b488050c78b89291d740889ed4d785f4104448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f30000000100000039fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 5, hash: 004fc675773657de765c1a355e9123624d7524bc3cdaaeea298a49c74583fe40
        "010000000005005c57b635000e9a58542cc77a2442721d839c23be67549940dbe3f0e84e18fb1630d90ae44448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000004ebfffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 6, hash: 001a0cad03d0ff1f8b6d8f7d9c1e35bbb1d648c50f16738b4a4029c9e4c5be79
        "010000000006005c57b69b004fc675773657de765c1a355e9123624d7524bc3cdaaeea298a49c74583fe404448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f300000001000005b7fffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 7, hash: 001a62c40e11085237574ab293c3056085f6071dff744c9883fd2c5b92f7f33e
        "010000000007005c57b742001a0cad03d0ff1f8b6d8f7d9c1e35bbb1d648c50f16738b4a4029c9e4c5be794448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000076ffffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 8, hash: 003e7e42d2b866a066477894f9a14d788061265503ee144286c7d4927df3b3f4
        "010000000008005c57b825001a62c40e11085237574ab293c3056085f6071dff744c9883fd2c5b92f7f33e4448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000013afffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
        // hei: 9, hash: 0012beb10c426427407d769c7c4ace246f1c564dabc7787c86ee5b21c3517cdb
        "010000000009005c57b983003e7e42d2b866a066477894f9a14d788061265503ee144286c7d4927df3b3f44448ea1749d50416b41848e62edb30f8570153f80bd463f6b76de8d2948050f3000000010000041ffffffffe000000000c1fa1c032d90fd7afc54deb03941e87b4c59756f801012020202020202020202020202020202000",
    ];
    let mut blks = Vec::with_capacity(datas.len());
    for dt in datas {
        let dt = hex::decode(dt).unwrap();
        let mut blk = BlockV1::default();
        if let Err(e) = blk.parse(&dt, 0) {
            panic!("{}", e)
        }
        blks.push(blk);
    }
    blks
}




pub fn create_block(height: u64, pt: u64, prev: &str) {

    let blktime = Timestamp::from(pt); // 1549250700);
    let blknoncenum = Uint4::from(160117829);
    let reward_addr = Address::from_readable(&"1271438866CSDpJUqrnchoJAiGGBFSQhjd".to_string()).unwrap();
    let mut trsvec = DynVecTransaction::default(); 
    trsvec.push(Box::new(TransactionCoinbase{
        ty: Uint1::from(0),
        address: reward_addr,
        reward: Amount::new_coin(1),
        message: StringTrim16::from_readable(b"hardertodobetter"),
        extend: CoinbaseExtend::default()
    }));
    let mut genesis_block = BlockV1 {
        intro: BlockHeadMeta { 
            head: BlockHead { 
                version: Uint1::from(1), 
                height: BlockHeight::from(height), 
                timestamp: blktime, 
                prevhash: Hash::from_hex(prev.as_bytes()),
                mrklroot: Hash::default(), // 000000...
                transaction_count: Uint4::from(1) // trs 1
            }, 
            meta: BlockMeta { 
                nonce: blknoncenum, 
                difficulty: Uint4::from(0), 
                witness_stage: Fixed2::default() 
            },
        },
        transactions: trsvec
    };
    // set mrklroot
    genesis_block.update_mrklroot();

    
    println!("hash: {}", hex::encode(genesis_block.hash()));
    println!("body: {}", hex::encode(genesis_block.serialize()));


}