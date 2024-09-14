
/**
 * Diamond Constant
 */
// start with the 20001st diamond and enable the 32-bit MSG byte
pub const DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE: u32 = 2_0000;

// Starting from the 30001st diamond, destroy 90% of the bidding cost
pub const DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES: u32 = 3_0000;

// The average bidding cost of 30001 ~ 40000 diamonds is adopted, and the previous setting is 10 HAC
pub const DIAMOND_ABOVE_NUMBER_OF_STATISTICS_AVERAGE_BIDDING_BURNING: u32 = 4_0000;

// 40001 diamond, start with Sha3_ Hash (diamondreshash + blockhash) determines diamond shape and color matching
pub const DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BLOCK_HASH: u32 = 4_0000;

// 41001 diamond, start with Sha3_ Hash (diamondreshash + blockhash + bidfee) includes the bidding fee to participate in the decision of diamond shape color matching
pub const DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BIDDING_FEE: u32 = 4_1000;


/**
* Action DiamondMint
*/
StructFieldStruct!{ DiamondMintHead,
	diamond              : DiamondName    
	number               : DiamondNumber    
	prev_hash            : Hash         
	nonce                : Fixed8        
	address              : Address    
}

StructFieldStructSetParseSerializeSize!{
    self, buf, seek, {
        // parse
        let mut skn: usize = seek;
        skn = self.kind.parse(buf, skn)?;
        skn = self.head.parse(buf, skn)?;
        if self.head.number.to_u32() > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE {
            skn = self.custom_message.parse(buf, skn)?;
        }
        return Ok(skn)
    }, {
        // serialize
        let mut buf = vec![
            self.kind.serialize(),
            self.head.serialize(),
        ];
        if self.head.number.to_u32() > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE {
            buf.push( self.custom_message.serialize() );
        }
        return buf.concat();
    }, {
        // size
        let mut sz = self.kind.size()
            + self.head.size();
        if self.head.number.to_u32() > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE {
            sz += self.custom_message.size();
        }
        return sz 
    }, 
    DiamondMint, 
    // con
    kind                 : Uint2  
    head                 : DiamondMintHead
	// customer message                                   
	custom_message       : Hash
}


// 
impl DiamondMint {

    pub fn with(name: DiamondName, number: DiamondNumber) -> DiamondMint {
        DiamondMint{
            kind: Uint2::from(4),
            head: DiamondMintHead{
                diamond: name,
                number: number,
                prev_hash: Hash::default(),
                nonce: Fixed8::default(),
                address: Address::default(),
            },
            custom_message: Hash::default(),
        }
    }

}


/**
 * DiamondMint Action Def
 */
ActionDefineWithStruct!{
    DiamondMint : 4,
    ACTLV_TOP_UNIQUE, // level
    6 + 3 + 32 + 8 + 21 + 32, // gas
    (self, ctx, state, store, gas), // params
    { self.head.number.to_u32() > DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES }, // burn 90
    [], // req sign
    diamond_mint(self, ctx, state, store)
}



/**
 * DiamondMint Exec
 */
fn diamond_mint(this: &DiamondMint, ctx: &dyn ExecContext, sta: &mut dyn State, sto: &dyn Store) -> Ret<Vec<u8>> {
    require_address_version_privkey!(&this.head.address);

    let mut state = MintState::wrap(sta);
    let store = MintStoreDisk::wrap(sto);

    let pending_height = ctx.pending_height();
    let pending_hash = ctx.pending_hash();

    let number = this.head.number;
    let dianum = number.to_u32();
    let name = this.head.diamond;
    let namestr = name.readable();
    let prev_hash = this.head.prev_hash;
    let nonce = this.head.nonce;
    let address = this.head.address;
    let mut custom_message = Vec::new();
    if dianum > DIAMOND_ABOVE_NUMBER_OF_CREATE_BY_CUSTOM_MESSAGE {
        custom_message = this.custom_message.serialize();
    }
    // check mine
    let (sha3hx, mediumhx, diahx) = x16rs::mine_diamond(dianum, &prev_hash, &nonce, &address, &custom_message);

    let not_fast_sync = false == ctx.fast_sync();
    if not_fast_sync {

        // check
        if pending_hash.is_not_zero() && pending_height % 5 != 0 {
            return errf!("diamond must be contained in block height are highly divisible by 5")
        }
        // number
        let prevdia = state.latest_diamond();
        let neednextnumber = 1 + prevdia.number.to_u32();
        if dianum != neednextnumber {
            return errf!("diamond number need {} but got {}", neednextnumber, dianum)
        }
        // prev hash
        if dianum > 1 && prevdia.born_hash != prev_hash {
            return errf!("diamond prev hash need {} but got {}", prevdia.born_hash, prev_hash)
        }

        // latest
        let latest_diamond = state.latest_diamond();
        let latestdianum = latest_diamond.number.to_u32();
        if dianum != 1 + latestdianum {
            return errf!("latest diamond number need {} but got {}", dianum - 1, latestdianum)
        }

        // difficulty
        let diffok = x16rs::check_diamond_difficulty(dianum, &sha3hx, &mediumhx);
        if ! diffok {
            return errf!("diamond difficulty not match")
        }

        // name
        let dianame = x16rs::check_diamond_hash_result(diahx);
        let Some(dianame) = dianame else {
            let dhx = match String::from_utf8(diahx.to_vec()) {
                Err(_) => hex::encode(diahx),
                Ok(d) => d
            };
            return errf!("diamond hash result {} not a valid diamond name", dhx)
        };
        let dianame = Fixed6::cons(dianame);
        if name != dianame {
            return errf!("diamond name need {} but got {}", dianame.readable(), namestr)
        }

        // exist
        let hav = state.diamond(&name);
        if let Some(_) = hav {
            return errf!("diamond {} already exist", namestr)
        }

    }

    // tx fee
    let tx_bid_fee = ctx.tx_fee();

    // total count 
    let mut ttcount = state.total_count();
    ttcount.minted_diamond += 1u64;
    if dianum > DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES {
        let mut sub = tx_bid_fee.clone();
        if sub.unit() > 1 {
            sub.unit_sub(1);
        }
        let burn = tx_bid_fee.clone().sub(&sub)?; // 90%
        ttcount.hacd_bid_burn_zhu += Uint8::from_u64(burn.to_zhu_unsafe() as u64);
    }

    // gene
    let (life_gene, visual_gene) = calculate_diamond_gene(dianum, &mediumhx, &diahx, &pending_hash, &tx_bid_fee);

    // bid_burn    
    let average_bid_burn = calculate_diamond_average_bid_burn(dianum, ttcount.hacd_bid_burn_zhu.to_u64());

    // save diamond smelt
    let diasmelt = DiamondSmelt {
        diamond: name.clone(),
        number: number.clone(),
        born_height: BlockHeight::from_u64(pending_height),
        born_hash: pending_hash.clone(),
        prev_hash: prev_hash.clone(),
        miner_address: this.head.address.clone(),
        bid_fee: tx_bid_fee.clone(),
        nonce: nonce.clone(),
        average_bid_burn: average_bid_burn,
        life_gene: life_gene,
    };
    state.set_latest_diamond(&diasmelt);
    store.put_diamond_smelt(&name, &diasmelt);

    // save diamond
    let diaitem = DiamondSto {
        status: DIAMOND_STATUS_NORMAL,
        address: this.head.address.clone(),
        prev_engraved_height: BlockHeight::default(), // 0
        inscripts: Inscripts::default() // none
    };
    state.set_diamond(&name, &diaitem);
    state.set_diamond_ptr(&number, &name);

    // add diamond belong
    diamond_owned_push_one(&mut state, &address, &name);
    
    // save count
    state.set_total_count(&ttcount);
    drop(state);

    // add balance
    let mut core_state = CoreState::wrap(sta);
    hacd_add(&mut core_state, &this.head.address, &DiamondNumber::from(1))?;

    // ok
    Ok(vec![])
}





/*************** util ***************/

const HEX_CHARS: &[u8; 16] = b"0123456789ABCDEF";


/**
 * calculate diamond visual gene
*/
pub fn calculate_diamond_visual_gene(name: &[u8;6], life_gene: &[u8;32]) -> (DiamondVisualGene) {
    
    let mut genehexstr = [b'0'; 20];
    // step 1
    let searchgx = |x| {
        for (i, a) in DIAMOND_NAME_VALID_CHARS.iter().enumerate() {
            if *a == x {
                return HEX_CHARS[i]
            }
        }
        panic!("not supply diamond char!!!")
    };

    for i in 0..6 {
        genehexstr[i+2] = searchgx( name[i] );
    }

    // step 2
    let mut idx = 8;
    for i in 20..31 {
        let k = (life_gene[i] as usize) % 16;
        genehexstr[idx] = HEX_CHARS[k];
        idx += 1;
    }
    // last bit of hash as shape selection
    let mut genehex = hex::decode(genehexstr).unwrap();
    genehex[0] = life_gene[31];
    
    // ok
    DiamondVisualGene::cons(genehex.try_into().unwrap())
}

/**
 * calculate diamond visual gene
*/
pub fn calculate_diamond_gene(dianum: u32, diamhash: &[u8;32], diamondstr: &[u8;16], pedding_block_hash: &Hash, diabidfee: &Amount) -> (DiamondLifeGene, DiamondVisualGene) {
    
    
    // cacl vgenehash
    let mut vgenehash = diamhash.clone();
    if dianum > DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BLOCK_HASH {
        let mut vgenestuff = diamhash.to_vec();
        vgenestuff.append( &mut pedding_block_hash.to_vec() ); // add block hash
        if dianum > DIAMOND_ABOVE_NUMBER_OF_VISUAL_GENE_APPEND_BIDDING_FEE {
            vgenestuff.append( &mut diabidfee.serialize() ); // add bidfee
        }
        vgenehash = x16rs::calculate_hash(vgenestuff);
    }

    let dianame = diamondstr[10..16].try_into().unwrap();
    // ok ret
    (
        DiamondLifeGene::cons(vgenehash.try_into().unwrap()),
        calculate_diamond_visual_gene(&dianame, &vgenehash), 
    )
}


/**
 * calculate diamond average bid burn
 */
pub fn calculate_diamond_average_bid_burn(diamond_number: u32, hacd_burn_zhu: u64) -> Uint2 {

    // old
    if diamond_number <= DIAMOND_ABOVE_NUMBER_OF_STATISTICS_AVERAGE_BIDDING_BURNING {
        return Uint2::from(10)
    }

    // average
    let bsnum = diamond_number - DIAMOND_ABOVE_NUMBER_OF_BURNING90_PERCENT_TX_FEES;
    let bidfee = hacd_burn_zhu / 1_0000_0000 / (bsnum as u64) + 1;
    // ok
    Uint2::from(bidfee as u16)
}

