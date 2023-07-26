use context::Context;
use loaded_dice::LoadedDiceSampler;
use newtypes::{NTermID, NodeID};
use rand::rngs::StdRng;
use rand::thread_rng;
use rand::SeedableRng;
use serde::{Deserialize, Deserializer, Serialize};
use std::collections::HashMap;
use std::fmt;
use tree::Tree;

#[derive(Serialize, Clone, Deserialize)]
pub struct RecursionInfo {
    recursive_parents: HashMap<NodeID, NodeID>,
    #[serde(skip_serializing, deserialize_with = "deserialize_sampler")]
    sampler: Option<LoadedDiceSampler<StdRng>>,
    depth_by_offset: Vec<usize>,
    node_by_offset: Vec<NodeID>,
    weights: Vec<f64>,
}

fn build_sampler_from_weights(weights: &Vec<f64>) -> LoadedDiceSampler<StdRng> {
    LoadedDiceSampler::new(
        weights.clone(),
        StdRng::from_rng(thread_rng()).expect("StdRng::from_rng err!"),
    )
}

fn deserialize_sampler<'de, D>(
    deserializer: D,
) -> Result<Option<LoadedDiceSampler<StdRng>>, D::Error>
where
    D: Deserializer<'de>,
{
    // Deserialize the weights field from the JSON data
    let weights: Vec<f64> = Deserialize::deserialize(deserializer)?;

    // Build the sampler using the weights
    let sampler = build_sampler_from_weights(&weights);

    Ok(Some(sampler))
}

impl fmt::Debug for RecursionInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RecursionInfo")
            .field("recursive_parents", &self.recursive_parents)
            .field("depth_by_offset", &self.depth_by_offset)
            .field("node_by_offset", &self.node_by_offset)
            .finish()
    }
}

impl RecursionInfo {
    #[must_use]
    pub fn new(t: &Tree, n: NTermID, ctx: &Context) -> Option<Self> {
        let (recursive_parents, node_by_offset, depth_by_offset) =
            RecursionInfo::find_parents(t, n, ctx)?;
        let sampler = RecursionInfo::build_sampler(&depth_by_offset);
        Some(Self {
            recursive_parents,
            sampler: Some(sampler.0),
            depth_by_offset,
            node_by_offset,
            weights: sampler.1,
        })
    }

    // constructs a tree where each node points to the first ancestor with the same nonterminal (e.g. each node points the next node above it, were the pair forms a recursive occurance of a nonterminal).
    // This structure is an ''inverted tree''. We use it later to sample efficiently from the set
    // of all possible recursive pairs without occuring n^2 overhead. Additionally, we return a
    // ordered vec of all nodes with nonterminal n and the depth of this node in the freshly
    // constructed 'recursion tree' (weight). Each node is the end point of exactly `weigth` many
    // differnt recursions. Therefore we use the weight of the node to sample the endpoint of a path trough the
    // recursion tree. Then we just sample the length of this path uniformly as (1.. weight). This
    // yields a uniform sample from the whole set of recursions inside the tree. If you read this, Good luck you are on your own.
    fn find_parents(
        t: &Tree,
        nt: NTermID,
        ctx: &Context,
    ) -> Option<(HashMap<NodeID, NodeID>, Vec<NodeID>, Vec<usize>)> {
        let mut stack = vec![(None, 0)];
        let mut res = None;
        for (i, rule) in t.rules.iter().enumerate() {
            let node = NodeID::from(i);
            let (mut maybe_parent, depth) = stack.pop().expect("RAND_3404900492");
            if ctx.get_nt(rule) == nt {
                if let Some(parent) = maybe_parent {
                    let (mut parents, mut ids, mut weights) =
                        res.unwrap_or_else(|| (HashMap::new(), vec![], vec![]));
                    parents.insert(node, parent);
                    ids.push(node);
                    weights.push(depth);
                    res = Some((parents, ids, weights));
                }
                maybe_parent = Some(node);
            }
            for _ in 0..ctx.get_num_children(rule) {
                stack.push((maybe_parent, depth + 1));
            }
        }
        res
    }

    fn build_sampler(depths: &[usize]) -> (LoadedDiceSampler<StdRng>, Vec<f64>) {
        let mut weights = depths.iter().map(|x| *x as f64).collect::<Vec<_>>();
        let norm: f64 = weights.iter().sum();
        assert!(norm > 0.0);
        for v in &mut weights {
            *v /= norm;
        }

        (build_sampler_from_weights(&weights), weights)
    }

    pub fn get_random_recursion_pair(&mut self) -> (NodeID, NodeID) {
        let offset = self.sampler.clone().unwrap().sample();
        self.get_recursion_pair_by_offset(offset)
    }

    #[must_use]
    pub fn get_recursion_pair_by_offset(&self, offset: usize) -> (NodeID, NodeID) {
        let node1 = self.node_by_offset[offset];
        let mut node2 = node1;
        for _ in 0..(self.depth_by_offset[offset]) {
            node2 = self.recursive_parents[&node1];
        }
        (node2, node1)
    }

    #[must_use]
    pub fn get_number_of_recursions(&self) -> usize {
        self.node_by_offset.len()
    }
}
