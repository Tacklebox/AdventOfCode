use std::cmp::{Ordering, Reverse};
use std::collections::{BinaryHeap, HashSet, VecDeque};
use std::hash::Hash;

/// Struct to hold search result with path and cost
#[derive(Debug, Clone)]
pub struct SearchResult<T> {
    pub path: Vec<T>,
    pub cost: i64,
}

/// A generic graph search trait to define the behavior of different frontier data structures
pub trait Frontier<T> {
    fn push(&mut self, item: SearchState<T>);
    fn pop(&mut self) -> Option<SearchState<T>>;
    fn is_empty(&self) -> bool;
}

/// Detailed state tracking for path reconstruction
#[derive(Clone)]
pub struct SearchState<T> {
    state: T,
    path: Vec<T>,
    cost: i64,
}

impl<T: Eq + PartialEq> PartialEq for SearchState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl<T: Eq + PartialEq> Eq for SearchState<T> {}

impl<T: Eq + PartialEq> PartialOrd for SearchState<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T: Eq + PartialEq> Ord for SearchState<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.cost.cmp(&other.cost)
    }
}

/// Implement Frontier for Queue (for BFS)
impl<T> Frontier<T> for VecDeque<SearchState<T>> {
    fn push(&mut self, item: SearchState<T>) {
        self.push_back(item)
    }

    fn pop(&mut self) -> Option<SearchState<T>> {
        self.pop_front()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Implement Frontier for Priority Queue (for A*)
impl<T: Clone + std::cmp::Eq> Frontier<T> for BinaryHeap<Reverse<SearchState<T>>> {
    fn push(&mut self, item: SearchState<T>) {
        BinaryHeap::push(self, Reverse(item));
    }

    fn pop(&mut self) -> Option<SearchState<T>> {
        self.pop().map(|Reverse(state)| state)
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

/// Generic graph search function with path tracking
pub fn graph_search<T, F, N>(
    initial_state: T,
    is_goal: F,         // Closure to check if we've reached the goal
    get_next_states: N, // Closure to generate next possible states with cost
    frontier: &mut dyn Frontier<T>,
) -> Option<SearchResult<T>>
where
    T: Eq + Hash + Clone,
    F: Fn(&T) -> bool,
    N: Fn(&T) -> Vec<(T, i64)>,
{
    let mut visited = HashSet::new();

    let initial_search_state = SearchState {
        state: initial_state.clone(),
        path: vec![initial_state],
        cost: 0,
    };

    frontier.push(initial_search_state.clone());

    while !frontier.is_empty() {
        if let Some(current_search_state) = frontier.pop() {
            let current_state = current_search_state.state.clone();

            if is_goal(&current_state) {
                return Some(SearchResult {
                    path: current_search_state.path,
                    cost: current_search_state.cost,
                });
            }

            if visited.contains(&current_state) {
                continue;
            }
            visited.insert(current_state.clone());

            for (next_state, step_cost) in get_next_states(&current_state) {
                if !visited.contains(&next_state) {
                    let mut next_path = current_search_state.path.clone();
                    next_path.push(next_state.clone());

                    let next_search_state = SearchState {
                        state: next_state.clone(),
                        path: next_path,
                        cost: current_search_state.cost + step_cost,
                    };

                    frontier.push(next_search_state);
                }
            }
        }
    }

    None
}
