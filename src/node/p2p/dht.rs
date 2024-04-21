

/**
 * return: maybe drop one
 */
fn insert_peer_to_dht_list(lklist: PeerList, max: usize, 
    compare: &PeerID, peer: Arc<Peer>
) -> Option<Arc<Peer>> {

    let mut list = lklist.lock().unwrap();
    let length = list.len();
    let mut insert_idx = length;
    for i in 0..length {
        let disnum = compare_peer_id_topology_distance(compare, &peer.pid, &list[i].pid);
        if disnum == 1 {
            insert_idx = i;
            break;
        }
    }
    // insert
    list.insert(insert_idx, peer);
    // drop the tail
    if list.len() > max {
        return list.pop()
    }
    // isert ok no drop
    None
}


/**
 * left closer ret 1
 * right closer ret -1
 * same ret 0
 */
fn compare_peer_id_topology_distance(compare: &PeerID, left: &PeerID, right: &PeerID) -> i8 {
    for i in 0..compare.len() {
		let ds1 = calculate_one_byte_topology_distance(compare[i], left[i]);
		let ds2 = calculate_one_byte_topology_distance(compare[i], right[i]);
		if ds1 < ds2 {
			return 1
		} else if ds1 > ds2 {
			return -1
		}
		// diff next byte
	}
    // same
	return 0
}


fn calculate_one_byte_topology_distance(dst: u8, src: u8) -> u8 {
    if dst > src {
        return dst - src
    }else if dst < src {
        return src - dst
    }
    // dst == src
    return 0
}