
/**
* Diamond Status
*/
pub const DIAMOND_STATUS_NORMAL                : Uint1 = Uint1::from(1);
pub const DIAMOND_STATUS_LENDING_TO_SYSTEM     : Uint1 = Uint1::from(2);
pub const DIAMOND_STATUS_LENDING_TO_USER       : Uint1 = Uint1::from(3);


/**
* Diamond Inscripts
*/
StructFieldList!{ Inscripts, 
	count, Uint1, lists, BytesW1
}

impl Inscripts {
	pub fn array(&self) -> Vec<String> {
		let mut resv = Vec::with_capacity(self.lists.len());
		for li in &self.lists {
			let rdstr = bytes_try_to_readable_string(li.as_ref());
			resv.push(match rdstr {
				None => hex::encode(li.as_ref()),
				Some(s) => s,
			});
		}
		resv
	}
}


/**
* Diamond
*/
StructFieldStruct!{ DiamondSto, 
	status    : Uint1
	address   : Address
	prev_engraved_height : BlockHeight
	inscripts : Inscripts
 }


/**
* DiamondSmelt
*/
StructFieldStruct!{ DiamondSmelt, 
	diamond                   : DiamondName
	number                    : DiamondNumber
	belong_height             : BlockHeight
	belong_hash               : Hash // block
	prev_hash                 : Hash // block
	miner_address             : Address
	bid_fee                   : Amount
	nonce                     : Fixed8
	// custom_message           : HashOptional
	average_bid_burn          : Uint2
	life_gene                 : Hash
}



/**
* DiamondOwnedForm
*/
StructFieldStruct!{ DiamondOwnedForm, 
	names : BytesW4
}
impl DiamondOwnedForm {

	pub fn readable(&self) -> String {
		String::from_utf8_lossy( self.names.as_ref() ).to_string()
	}
	
	pub fn push_one(&mut self, dian: &DiamondName) {
		let mut bytes = dian.serialize();
		self.names.append(&mut bytes);
	}

	pub fn drop_one(&mut self, dian: &DiamondName) -> Ret<usize> {
		let mut list = DiamondNameListMax200::default();
		list.push(dian.clone());
		self.drop(&list)
	}

	pub fn push(&mut self, dian: &DiamondNameListMax200) {
		let mut bytes = dian.form();
		self.names.append(&mut bytes);
	}

	// return balance quantity
	pub fn drop(&mut self, dian: &DiamondNameListMax200) -> Ret<usize> {

		let l = DiamondName::width();
		let srclen = dian.count().to_usize();
		let mut dianset = dian.hashset();
		let dstsz = self.names.length();
		let dstlen = dstsz / l;
		let mut rmleft = 0;
		for i in 0..dstlen {
			let x = i*l;
			let y = x+l;
			let dia = DiamondName::cons(self.names.bytes[x..y].try_into().unwrap());
			if dianset.contains(&dia) {
				dianset.remove(&dia);
				if x == rmleft {
					// drop head, do nohing
				}else{
					let cvd = rmleft .. rmleft+l;
					self.names.bytes.copy_within(cvd, x);
				}
				rmleft += l;
			}
			if dianset.is_empty() {
				break // all finish
			}
		}
		if rmleft/l != srclen {
			println!("rmleft/l={}, srclen={}, dstlen={}", rmleft/l, srclen, dstlen);
			return errf!("drop {} not match", srclen)
		}
		self.names.bytes = self.names.bytes.split_off(rmleft);
		self.names.count -= srclen * l;
		Ok(self.names.count.to_usize())


		/*
		let l = DiamondName::width();
		let srclen = dian.count().to_usize();
		let dstsz = self.names.length();
		let dstlen = dstsz / l;
		let mut tcnum = dstsz;
		let mut findn = 0;
		// println!("{:?}", (0..dstlen).rev());
		for i in (0..dstlen).rev() {
			let x = i*l;
			let y = x+l;
			let dia = &self.names.bytes[x..y];
			if dian.contains(dia) {
				// println!("i={}, x={}, tcnum={}", i, x, tcnum);
				if y == tcnum {
					// drop tail, do nohing
				}else{
					// replace delete posi
					let cvd = tcnum-l .. tcnum;
					self.names.bytes.copy_within(cvd, x);
				}
				tcnum -= l;
				findn += 1;
			}
			if findn >= srclen {
				break // finish
			}
		}
		if tcnum/l + srclen != dstlen {
			// println!("tcnum/l={}, srclen={}, dstlen={}", tcnum/l, srclen, dstlen);
			return errf!("drop {} not match", srclen)
		}
		self.names.bytes.truncate(tcnum);
		self.names.count -= srclen * l;
		Ok(())
		*/


		/*
		let dstlen = self.names.length() / 6;
		let mut oldlist = HashSet::with_capacity(dstlen);
		for i in 0..dstlen {
			let rg = i*6 .. i*6+6;
			let dia = DiamondName::cons(self.names.bytes[rg].try_into().unwrap());
			oldlist.insert(dia);
		}
		let dplist = dian.hashset();
		let newlist: HashSet<_> = oldlist.difference(&dplist).collect(); // oldlist - dplist
		let resbts = newlist.iter().map(|a|a.serialize()).collect::<Vec<_>>().concat();
		self.names = BytesW4::from_vec(resbts);
		Ok(())
		*/

		/*
		let prvlen = self.names.length();
		let mut dstlen = self.names.length() / 6;
		let srclen = dian.count().to_usize();
		let dialist = dian.list();
		let mut splicergs: Vec<Range<usize>> = Vec::with_capacity(srclen);
		for x in dialist {
			for i in 0..dstlen {
				let rg = i*6 .. i*6+6;
				let dia = &self.names.bytes[rg.clone()];
				if dia == x.as_ref() {
					self.names.bytes.splice(rg, vec![]);
					dstlen -= 1;
					break // this find end
				} 
			}
		}
		// check 
		if prvlen - self.names.bytes.len() != srclen * 6 {
			return errf!("drop {} not match", srclen)
		}
		// sub length
		self.names.count -= srclen * 6;
		Ok(())
		*/

		/*
		let prvlen = self.names.length();
		let mut dstlen = self.names.length() / 6;
		let srclen = dian.count().to_usize();
		let dialist = dian.list();
		let mut splicergs: Vec<Range<usize>> = Vec::with_capacity(srclen);
		for x in dialist {
			for i in 0..dstlen {
				let rg = i*6 .. i*6+6;
				let dia = &self.names.bytes[rg.clone()];
				if dia == x.as_ref() {
					let ol = splicergs.len();
					if ol > 0 && splicergs[ol-1].end == rg.start {
						splicergs[ol-1] = (splicergs[ol-1].start) .. (rg.end);
					}else if ol > 0 && splicergs[ol-1].start == rg.end {
							splicergs[ol-1] = (rg.start) .. (splicergs[ol-1].end);
					}else{
						splicergs.push(rg);
					}
					// dstlen -= 1;
					break // this find end
				} 
			}
		}
		splicergs.sort_by(|a, b| a.start.cmp(&b.start));
		let mut sbrg = 0;
		for rg in &splicergs {
			let sp = rg.end - rg.start;
			let nrg = (rg.start-sbrg) .. (rg.end-sbrg);

			self.names.bytes.splice(nrg, vec![]);
			
			sbrg += sp;
		}
		// check 
		if prvlen - self.names.bytes.len() != srclen * 6 {
			return errf!("drop {} not match", srclen)
		}
		// sub length
		self.names.count -= srclen * 6;
		Ok(())
		*/
		
		
	}

}



