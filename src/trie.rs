use std::collections::{BTreeMap, HashSet};

#[derive(Default, Debug)]
struct State {
    edges: BTreeMap<u8, usize>,
    value: Option<u32>,
}

/// Sparse trie.
pub struct Sparse {
    states: Vec<State>,
}

impl Sparse {
    pub fn new() -> Self {
        Self {
            states: vec![State::default()],
        }
    }

    /// Adds a new pattern.
    pub fn add(&mut self, pattern: impl AsRef<[u8]>, value: u32) {
        let pattern = pattern.as_ref();
        let mut state_idx = 0;
        for &b in pattern {
            let new_idx = self.states.len();
            state_idx = *self.states[state_idx].edges.entry(b).or_insert(new_idx);
            if state_idx == new_idx {
                self.states.push(State::default());
            }
        }
        self.states[state_idx].value = Some(value);
    }

    fn find_base(
        search_start: i32,
        is_used: &[bool],
        state: &State,
        used_bases: &HashSet<i32>,
    ) -> Option<i32> {
        let (&k, _) = state.edges.iter().next()?;
        let mut base_cand = search_start - i32::from(k);
        'a: loop {
            if used_bases.contains(&base_cand) {
                base_cand += 1;
                continue;
            }
            for &k in state.edges.keys() {
                let pos = usize::try_from(base_cand + i32::from(k)).unwrap();
                if let Some(&u) = is_used.get(pos) {
                    if u {
                        base_cand += 1;
                        continue 'a;
                    }
                }
            }
            break;
        }
        Some(base_cand)
    }

    /// Builds a compact double-array.
    ///
    /// # Arguments
    ///
    /// * `wildcard_idx` - A wild card index that is used for invalid state. This value is returned
    ///                    if the query matches no pattern.
    ///
    /// # Returns
    ///
    /// The first item is a base array, and the second item is out_check array.
    pub fn build_double_array_trie(&self, wildcard_idx: u32) -> (Vec<i32>, Vec<u32>) {
        let mut bases = vec![i32::MAX];
        let mut out_checks = vec![wildcard_idx << 8];
        let mut is_used = vec![true];
        let mut stack = vec![(0, 0)];
        let mut used_bases = HashSet::new();
        let mut search_start = 0;
        while let Some((state_id, da_pos)) = stack.pop() {
            let state = &self.states[state_id];
            if let Some(val) = state.value {
                let check = out_checks[da_pos] & 0xff;
                out_checks[da_pos] = val << 8 | check;
            }
            for &u in &is_used[usize::try_from(search_start).unwrap()..] {
                if !u {
                    break;
                }
                search_start += 1;
            }
            if let Some(base) = Self::find_base(search_start, &is_used, state, &used_bases) {
                used_bases.insert(base);
                bases[da_pos] = base;
                for (&k, &v) in &state.edges {
                    let child_da_pos = usize::try_from(base + i32::from(k)).unwrap();
                    if child_da_pos >= bases.len() {
                        bases.resize(child_da_pos + 1, i32::MAX);
                        out_checks.resize(child_da_pos + 1, wildcard_idx << 8);
                        is_used.resize(child_da_pos + 1, false);
                    }
                    out_checks[child_da_pos] = wildcard_idx << 8 | u32::from(k);
                    is_used[child_da_pos] = true;
                    stack.push((v, child_da_pos));
                }
            }
        }
        (bases, out_checks)
    }
}
