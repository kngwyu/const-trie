use super::{ByteOrd, InvalidByteError, CHAR_MAX};

pub(crate) fn ordering<P: AsRef<[u8]>>(
    word_iter: impl Iterator<Item = P>,
) -> Result<([ByteOrd; CHAR_MAX], ByteOrd), InvalidByteError> {
    let mut ord = [ByteOrd::EMPTY; CHAR_MAX];
    let mut count = ByteOrd(0);
    for word in word_iter {
        for &b in word.as_ref() {
            let u = b as usize;
            if u >= CHAR_MAX {
                return Err(InvalidByteError(b as char));
            }
            if ord[u].is_empty() {
                ord[u] = count;
                count.0 += 1;
            }
        }
    }
    return Ok((ord, count));
}

pub(crate) fn initial_bytes<P: AsRef<[u8]>>(
    word_iter: impl Iterator<Item = P>,
) -> Result<Vec<u8>, InvalidByteError> {
    let mut set = [false; CHAR_MAX];
    for word in word_iter {
        let b = word.as_ref()[0];
        let u = b as usize;
        if u >= CHAR_MAX {
            return Err(InvalidByteError(b as char));
        }
        set[u] = true;
    }
    let res = (0..CHAR_MAX).filter(|&b| set[b]).map(|u| u as u8).collect();
    return Ok(res);
}

pub(crate) fn transitions(upper_bound: ByteOrd) -> impl Iterator<Item = ByteOrd> {
    (0..upper_bound.0).map(ByteOrd)
}

// from
// https://github.com/BurntSushi/aho-corasick/blob/a738d5fd20d24dc41633c254d044eebba5caf081/src/lib.rs#L425
pub(crate) fn get_two<T>(xs: &mut [T], i: usize, j: usize) -> (&mut T, &mut T) {
    if i < j {
        let (before, after) = xs.split_at_mut(j);
        (&mut before[i], &mut after[0])
    } else {
        let (before, after) = xs.split_at_mut(i);
        (&mut after[0], &mut before[j])
    }
}

#[cfg(test)]
pub(crate) mod test_data {
    // https://en.wikipedia.org/wiki/Deathbird_Stories
    pub(crate) const WORDS: [&'static str; 19] = [
        "The Whimper of Whipped Dogs",
        "Along the Scenic Route",
        "On the Downhill Side",
        "O Ye of Little Faith",
        "Neon",
        "Basilisk",
        "Pretty Maggie Moneyeyes",
        "Corpse",
        "Shattered Like a Glass Goblin",
        "Delusion for a Dragon Slayer",
        "The Face of Helene Bournouw",
        "Bleeding Stones",
        "At the Mouse Circus",
        "The Place with No Name",
        "Paingod",
        "Ernest and the Machine God",
        "Rock God",
        "Adrift Just Off the Islets of Langerhans: Latitude 38 54 N, Longitude 77 00' 13\" W",
        "The Deathbird",
    ];
    pub(crate) const WORDS_SPARCE: [&'static str; 5] = ["ababc", "babcd", "abc", "b", "cba"];
}
