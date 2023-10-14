


// create

create_list_field_struct_and_impl!("DiamondListMax200", DiamondListMax200, count, Uint1, diamonds, DiamondName);


impl fmt::Display for DiamondListMax200{
    fn fmt(&self,f: &mut fmt::Formatter) -> fmt::Result{
        let arys: Vec<String> = self.diamonds.clone().into_iter().map(|d|d.name()).collect();
        write!(f,"{}", arys.join(","))
    }
}

impl DiamondListMax200 {

    pub fn parse_from_list(stuff: String) -> Result<DiamondListMax200, Error> {
        let mut chars = stuff.chars().collect::<Vec<char>>();
        let sz = chars.len();
        let mut liststr: Vec<char> = vec![];
        let mut liststrlen = 0;
        for i in 0..sz {
            let a = chars[i];
            let u = a as u8;
            if DIAMOND_NAME_VALID_CHARS.contains(&u) {
                liststr.push(a);
                liststrlen += 1;
            }else if liststrlen > 0 && liststr[liststrlen-1] != ' ' {
                liststr.push(' ');
                liststrlen += 1;
            }
        }
        let mut strmerge: String = liststr.into_iter().collect();
        strmerge = strmerge.trim().to_string();
        let arys: Vec<&str> = strmerge.split(" ").collect();
        if arys.len() > 200 {
            return Err("diamond list length cannot over 200".to_string())
        }
        let mut list = DiamondListMax200::new();
        for v in arys.iter() {
            if DiamondName::is_valid(v) {
                list.append(DiamondName::from_u8s(v))
            }else{
                return Err(format!("Stuff '{}' not a valid diamond name", v))
            }
        }
        // ok
        Ok(list)
    }


}