use std::collections::{BTreeMap, HashSet, VecDeque};

#[derive(Default, Debug)]
struct State {
    edges: BTreeMap<u8, usize>,
    value: Option<u32>,
    tail: Option<VecDeque<u8>>,
    parent: usize,
    incoming: u8,
}

pub struct Sparse {
    states: Vec<State>,
}

impl Sparse {
    pub fn new() -> Self {
        Self {
            states: vec![State::default()],
        }
    }

    pub fn add(&mut self, pattern: impl AsRef<[u8]>, value: u32) {
        let pattern = pattern.as_ref();
        let mut state_idx = 0;
        for &b in pattern {
            let new_idx = self.states.len();
            let next_idx = *self.states[state_idx].edges.entry(b).or_insert(new_idx);
            if next_idx == new_idx {
                self.states.push(State {
                    parent: state_idx,
                    incoming: b,
                    ..Default::default()
                });
            }
            state_idx = next_idx;
        }
        self.states[state_idx].value = Some(value);
        self.states[state_idx].tail = Some(VecDeque::new());
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

    fn update_tails(&mut self) {
        for i in (1..self.states.len()).rev() {
            let mut parent = self.states[i].parent;
            if self.states[parent].edges.len() >= 2 || self.states[parent].value.is_some() {
                loop {
                    self.states[parent].tail.take();
                    if parent == 0 {
                        break;
                    }
                    parent = self.states[parent].parent;
                }
                continue;
            }
            let Some(mut tail) = self.states[i].tail.take() else {
                continue;
            };
            let value = self.states[i].value.take().unwrap();
            tail.push_front(self.states[i].incoming);
            self.states[parent].value.replace(value);
            self.states[parent].tail.replace(tail);
            self.states[parent].edges.clear();
        }
    }

    pub fn build_double_array_trie(
        mut self,
        wildcard_idx: u32,
    ) -> (Vec<i32>, Vec<u32>, Vec<Vec<u8>>) {
        self.update_tails();
        let mut bases = vec![i32::MAX];
        let mut out_checks = vec![wildcard_idx << 8];
        let mut tails = vec![vec![]];
        let mut is_used = vec![true];
        let mut stack = VecDeque::new();
        stack.push_back((0, 0));
        let mut used_bases = HashSet::new();
        let mut search_start = 0;
        while let Some((state_id, da_pos)) = stack.pop_front() {
            let state = &self.states[state_id];
            if let Some(value) = state.value {
                let c = out_checks[da_pos] & 0xff;
                out_checks[da_pos] = value << 8 | c;
            }
            if let Some(tail) = &state.tail {
                tails[da_pos] = tail.iter().copied().collect();
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
                        tails.resize(child_da_pos + 1, vec![]);
                        is_used.resize(child_da_pos + 1, false);
                    }
                    out_checks[child_da_pos] = wildcard_idx << 8 | u32::from(k);
                    is_used[child_da_pos] = true;
                    stack.push_back((v, child_da_pos));
                }
            }
        }
        (bases, out_checks, tails)
    }
}
