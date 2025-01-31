use std::ops::Add;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct RuleID(usize);

#[derive(PartialEq, PartialOrd, Eq, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct NodeID(usize);

#[derive(PartialEq, Eq, Clone, Copy, Debug, Hash, Serialize, Deserialize)]
pub struct NTermID(usize);

impl RuleID {
    #[must_use]
    pub fn to_i(&self) -> usize {
        self.0
    }
}

impl From<usize> for RuleID {
    fn from(i: usize) -> Self {
        RuleID(i)
    }
}

impl From<RuleID> for usize {
    fn from(rule_id: RuleID) -> Self {
        rule_id.0
    }
}

impl Add<usize> for RuleID {
    type Output = RuleID;
    fn add(self, rhs: usize) -> RuleID {
        RuleID(self.0 + rhs)
    }
}

impl NodeID {
    #[must_use]
    pub fn to_i(&self) -> usize {
        self.0
    }
}

impl From<usize> for NodeID {
    fn from(i: usize) -> Self {
        NodeID(i)
    }
}

impl From<NodeID> for usize {
    fn from(node_id: NodeID) -> Self {
        node_id.0
    }
}

impl Add<usize> for NodeID {
    type Output = NodeID;
    fn add(self, rhs: usize) -> NodeID {
        NodeID(self.0 + rhs)
    }
}

impl NTermID {
    #[must_use]
    pub fn to_i(&self) -> usize {
        self.0
    }
}

impl From<usize> for NTermID {
    fn from(i: usize) -> Self {
        NTermID(i)
    }
}

impl From<NTermID> for usize {
    fn from(term_id: NTermID) -> Self {
        term_id.0
    }
}

impl Add<usize> for NTermID {
    type Output = NTermID;
    fn add(self, rhs: usize) -> NTermID {
        NTermID(self.0 + rhs)
    }
}

#[cfg(test)]
mod tests {
    use newtypes::NTermID;
    use newtypes::NodeID;
    use newtypes::RuleID;

    #[test]
    fn rule_id() {
        let r1: RuleID = 1337.into();
        let r2 = RuleID::from(1338);
        let i1: usize = r1.into();
        assert_eq!(i1, 1337);
        let i2: usize = 1338;
        assert_eq!(i2, r2.into());
        let r3 = r2 + 3;
        assert_eq!(r3, 1341.into());
    }

    #[test]
    fn node_id() {
        let r1: NodeID = 1337.into();
        let r2 = NodeID::from(1338);
        let i1: usize = r1.into();
        assert_eq!(i1, 1337);
        let i2: usize = 1338;
        assert_eq!(i2, r2.into());
        let r3 = r2 + 3;
        assert_eq!(r3, 1341.into());
    }

    #[test]
    fn nterm_id() {
        let r1: NTermID = 1337.into();
        let r2 = NTermID::from(1338);
        let i1: usize = r1.into();
        assert_eq!(i1, 1337);
        let i2: usize = 1338;
        assert_eq!(i2, r2.into());
        let r3 = r2 + 3;
        assert_eq!(r3, 1341.into());
    }
}
