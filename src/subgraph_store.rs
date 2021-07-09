use graph::data::subgraph::*;
use graph::{
    blockchain::BlockPtr,
    components::store::DeploymentLocator,
    prelude::{DeploymentHash, SubgraphStore},
};
use std::result::Result;
use std::sync::Arc;
use crate::writable_store::MockWritableStore;

pub struct MockSubgraphStore {}

impl SubgraphStore for MockSubgraphStore {
    fn find_ens_name(
        &self,
        _hash: &str,
    ) -> std::result::Result<Option<String>, graph::prelude::QueryExecutionError> {
        Ok(Some(String::from("ds")))
    }

    fn is_deployed(&self, _id: &DeploymentHash) -> Result<bool, anyhow::Error> {
        unreachable!()
    }

    fn create_subgraph_deployment(
        &self,
        _name: SubgraphName,
        _schema: &graph::prelude::Schema,
        _deployment: graph::prelude::SubgraphDeploymentEntity,
        _node_id: graph::prelude::NodeId,
        _network: String,
        _mode: graph::prelude::SubgraphVersionSwitchingMode,
    ) -> Result<DeploymentLocator, graph::prelude::StoreError> {
        unreachable!()
    }

    fn create_subgraph(&self, _name: SubgraphName) -> Result<String, graph::prelude::StoreError> {
        unreachable!()
    }

    fn remove_subgraph(&self, _name: SubgraphName) -> Result<(), graph::prelude::StoreError> {
        unreachable!()
    }

    fn reassign_subgraph(
        &self,
        _deployment: &DeploymentLocator,
        _node_id: &graph::prelude::NodeId,
    ) -> Result<(), graph::prelude::StoreError> {
        unreachable!()
    }

    fn assigned_node(
        &self,
        _deployment: &DeploymentLocator,
    ) -> Result<Option<graph::prelude::NodeId>, graph::prelude::StoreError> {
        unreachable!()
    }

    fn assignments(
        &self,
        _node: &graph::prelude::NodeId,
    ) -> Result<Vec<DeploymentLocator>, graph::prelude::StoreError> {
        unreachable!()
    }

    fn subgraph_exists(&self, _name: &SubgraphName) -> Result<bool, graph::prelude::StoreError> {
        unreachable!()
    }

    fn input_schema(
        &self,
        _subgraph_id: &DeploymentHash,
    ) -> Result<Arc<graph::prelude::Schema>, graph::prelude::StoreError> {
        unreachable!()
    }

    fn api_schema(
        &self,
        _subgraph_id: &DeploymentHash,
    ) -> Result<Arc<graph::prelude::ApiSchema>, graph::prelude::StoreError> {
        unreachable!()
    }

    fn writable(
        &self,
        _deployment: &DeploymentLocator,
    ) -> Result<Arc<dyn graph::components::store::WritableStore>, graph::prelude::StoreError> {
        let mock_writable_store = MockWritableStore {};
        Ok(Arc::from(mock_writable_store))
    }

    fn writable_for_network_indexer(
        &self,
        _id: &DeploymentHash,
    ) -> Result<Arc<dyn graph::components::store::WritableStore>, graph::prelude::StoreError> {
        unreachable!()
    }

    fn least_block_ptr(&self, _id: &DeploymentHash) -> Result<Option<BlockPtr>, anyhow::Error> {
        unreachable!()
    }

    fn locators(&self, _hash: &str) -> Result<Vec<DeploymentLocator>, graph::prelude::StoreError> {
        unreachable!()
    }
}