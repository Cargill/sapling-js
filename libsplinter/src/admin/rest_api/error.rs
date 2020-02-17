// Copyright 2018-2020 Cargill Incorporated
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::error::Error;

use crate::circuit::store;

#[derive(Debug)]
pub enum ProposalRouteError {
    NotFound(String),
    InternalError(String),
}

impl Error for ProposalRouteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            ProposalRouteError::NotFound(_) => None,
            ProposalRouteError::InternalError(_) => None,
        }
    }
}

impl std::fmt::Display for ProposalRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProposalRouteError::NotFound(msg) => write!(f, "Proposal not found: {}", msg),
            ProposalRouteError::InternalError(msg) => write!(f, "Ran into internal error: {}", msg),
        }
    }
}

#[derive(Debug)]
pub enum CircuitRouteError {
    NotFound(String),
    CircuitStoreError(store::CircuitStoreError),
}

impl Error for CircuitRouteError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            CircuitRouteError::NotFound(_) => None,
            CircuitRouteError::CircuitStoreError(err) => Some(err),
        }
    }
}

impl std::fmt::Display for CircuitRouteError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CircuitRouteError::NotFound(msg) => write!(f, "Circuit not found: {}", msg),
            CircuitRouteError::CircuitStoreError(err) => write!(f, "{}", err),
        }
    }
}

impl From<store::CircuitStoreError> for CircuitRouteError {
    fn from(err: store::CircuitStoreError) -> Self {
        CircuitRouteError::CircuitStoreError(err)
    }
}