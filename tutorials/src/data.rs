//! Tutorial data structures

use std::collections::HashSet;

use gloo::storage::Storage;
use yew::NodeRef;

#[allow(dead_code)]
#[derive(PartialEq, Clone)]
pub(crate) struct TutorialDataNode {
    node: NodeRef,
    name: &'static str,
    text: &'static str,
}

impl TutorialDataNode {
    pub(crate) fn node(&self) -> &NodeRef {
        &self.node
    }

    pub(crate) fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn text(&self) -> &'static str {
        self.text
    }
}

#[derive(PartialEq, Clone)]
pub struct TutorialData {
    name: &'static str,
    nodes: Vec<TutorialDataNode>,
}

impl TutorialData {
    pub fn new(name: &'static str) -> Self {
        Self {
            name,
            nodes: vec![],
        }
    }

    pub fn add_node(&mut self, node: NodeRef, name: &'static str, text: &'static str) {
        self.nodes.push(TutorialDataNode { node, name, text })
    }

    pub fn name(&self) -> &'static str {
        self.name
    }

    pub(crate) fn nodes(&self) -> &[TutorialDataNode] {
        &self.nodes
    }

    fn storage_key(&self) -> String {
        format!("tutorial-{}-viewed", self.name)
    }

    pub(crate) fn mark_node_used(&self, n: usize) {
        if let Some(data) = self.nodes.get(n) {
            let mut hidden: HashSet<String> =
                gloo::storage::LocalStorage::get(self.storage_key()).unwrap_or_default();
            hidden.insert(data.name().to_string());
            gloo::storage::LocalStorage::set(self.storage_key(), hidden).unwrap();
        }
    }
    pub(crate) fn abandon(&self) {
        let mut hidden: HashSet<String> =
            gloo::storage::LocalStorage::get(self.storage_key()).unwrap_or_default();
        for data in self.nodes().iter() {
            hidden.insert(data.name().to_string());
        }
        gloo::storage::LocalStorage::set(self.storage_key(), hidden).unwrap();
    }

    pub(crate) fn first_unseen(&self) -> Option<usize> {
        let hidden: HashSet<String> =
            gloo::storage::LocalStorage::get(self.storage_key()).unwrap_or_default();
        for (n, data) in self.nodes().iter().enumerate() {
            if !hidden.contains(data.name()) {
                return Some(n);
            }
        }
        None
    }
    pub(crate) fn next_unseen(&self, cur: usize) -> Option<usize> {
        let hidden: HashSet<String> =
            gloo::storage::LocalStorage::get(self.storage_key()).unwrap_or_default();
        for (n, data) in self.nodes().iter().enumerate().skip(cur + 1) {
            if !hidden.contains(data.name()) {
                return Some(n);
            }
        }
        None
    }

    pub(crate) fn prev_unseen(&self, cur: usize) -> Option<usize> {
        let hidden: HashSet<String> =
            gloo::storage::LocalStorage::get(self.storage_key()).unwrap_or_default();
        for (n, data) in self
            .nodes()
            .iter()
            .enumerate()
            .rev()
            .skip(self.nodes().len() - cur)
        {
            if !hidden.contains(data.name()) {
                return Some(n);
            }
        }
        None
    }
}
