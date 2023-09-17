use std::collections::{BTreeMap, HashSet};

#[derive(Debug)]
struct State<T> {
    edges: BTreeMap<u8, usize>,
    value: Option<T>,
}

impl<T> Default for State<T> {
    fn default() -> Self {
        Self {
            edges: BTreeMap::default(),
            value: None,
        }
    }
}

/// Sparse trie.
pub struct Sparse<T> {
    states: Vec<State<T>>,
}

impl<T> Sparse<T> {
    pub fn new() -> Self {
        Self {
            states: vec![State::default()],
        }
    }

    /// Adds a new pattern.
    pub fn add(&mut self, pattern: impl AsRef<[u8]>, value: T) {
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
        state: &State<T>,
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
    /// A tuple of a base array, a check array, and a value array.
    pub fn build_double_array_trie(&self, wildcard_value: T) -> (Vec<i32>, Vec<u8>, Vec<T>)
    where
        T: Copy,
    {
        let mut bases = vec![i32::MAX];
        let mut checks = vec![0];
        let mut values = vec![wildcard_value];
        let mut is_used = vec![true];
        let mut stack = vec![(0, 0)];
        let mut used_bases = HashSet::new();
        let mut search_start = 0;
        while let Some((state_id, da_pos)) = stack.pop() {
            let state = &self.states[state_id];
            if let Some(val) = state.value {
                values[da_pos] = val;
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
                        checks.resize(child_da_pos + 1, 0);
                        values.resize(child_da_pos + 1, wildcard_value);
                        is_used.resize(child_da_pos + 1, false);
                    }
                    checks[child_da_pos] = k;
                    is_used[child_da_pos] = true;
                    stack.push((v, child_da_pos));
                }
            }
        }
        // Postprocessing to avoid invalid transitions due to invalid checks.
        'a: for ((da_pos, check), &used) in checks.iter_mut().enumerate().zip(is_used.iter()) {
            if da_pos != 0 && used {
                continue;
            }
            let da_pos = i32::try_from(da_pos).unwrap();
            for k in 0..256 {
                let base = da_pos - k;
                if !used_bases.contains(&base) {
                    // There can be no transition using k from an unused BASE value.
                    *check = u8::try_from(k).unwrap();
                    continue 'a;
                }
            }
            unreachable!("No unused base found");
        }
        (bases, checks, values)
    }
}
