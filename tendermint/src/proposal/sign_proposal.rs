use super::Proposal;
use crate::chain::Id as ChainId;
use crate::{Error, Kind};
use bytes::BufMut;
use std::convert::TryFrom;
use std::str::FromStr;
use tendermint_proto::privval::RemoteSignerError;
use tendermint_proto::privval::SignProposalRequest as RawSignProposalRequest;
use tendermint_proto::privval::SignedProposalResponse as RawSignedProposalResponse;
use tendermint_proto::DomainType;
use tendermint_proto::Error as DomainTypeError;

/// SignProposalRequest is a request to sign a proposal
#[derive(Clone, PartialEq, Debug)]
pub struct SignProposalRequest {
    /// Proposal
    pub proposal: Proposal,
    /// Chain ID
    pub chain_id: ChainId,
}

impl DomainType<RawSignProposalRequest> for SignProposalRequest {}
impl DomainType<RawSignedProposalResponse> for SignedProposalResponse {}

impl TryFrom<RawSignProposalRequest> for SignProposalRequest {
    type Error = Error;

    fn try_from(value: RawSignProposalRequest) -> Result<Self, Self::Error> {
        if value.proposal.is_none() {
            return Err(Kind::NoProposalFound.into());
        }
        Ok(SignProposalRequest {
            proposal: Proposal::try_from(value.proposal.unwrap())?,
            chain_id: ChainId::from_str(value.chain_id.as_str()).unwrap(),
        })
    }
}

impl From<SignProposalRequest> for RawSignProposalRequest {
    fn from(value: SignProposalRequest) -> Self {
        RawSignProposalRequest {
            proposal: Some(value.proposal.into()),
            chain_id: value.chain_id.as_str().to_string(),
        }
    }
}

impl SignProposalRequest {
    /// Create signable bytes from Proposal.
    pub fn to_signable_bytes<B>(&self, sign_bytes: &mut B) -> Result<bool, DomainTypeError>
    where
        B: BufMut,
    {
        self.proposal.to_signable_bytes(self.chain_id, sign_bytes)
    }

    /// Create signable vector from Proposal.
    pub fn to_signable_vec(&self) -> Result<Vec<u8>, DomainTypeError> {
        self.proposal.to_signable_vec(self.chain_id)
    }
}

/// SignedProposalResponse is response containing a signed proposal or an error
#[derive(Clone, PartialEq)]
pub struct SignedProposalResponse {
    /// Proposal
    pub proposal: Option<Proposal>,
    /// Response error
    pub error: Option<RemoteSignerError>,
}

impl TryFrom<RawSignedProposalResponse> for SignedProposalResponse {
    type Error = Error;

    fn try_from(value: RawSignedProposalResponse) -> Result<Self, Self::Error> {
        Ok(SignedProposalResponse {
            proposal: match value.proposal {
                None => None,
                Some(proposal) => Some(Proposal::try_from(proposal)?),
            },
            error: value.error,
        })
    }
}

impl From<SignedProposalResponse> for RawSignedProposalResponse {
    fn from(value: SignedProposalResponse) -> Self {
        RawSignedProposalResponse {
            proposal: match value.proposal {
                None => None,
                Some(proposal) => Some(proposal.into()),
            },
            error: value.error,
        }
    }
}