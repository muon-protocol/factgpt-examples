// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import "./muon-utils/MuonClientBase.sol";

contract SimplePredictionMarket is MuonClientBase {
    using ECDSA for bytes32;

    // Muon App settings for FactGPT app
    uint256 public muonAppId;
    PublicKey public muonPublicKey;

    // Muon App settings for Dispute app
    uint256 public disputeMuonAppId;
    PublicKey public disputeMuonPublicKey;


    /**
     * Displays text and other metadata that should
     * be shown on the UI.
     * It could be saved off-chain.
     */
    string public displayText = "US Presidential Election 2024 (Trump or Harris)";

    /**
     * Prompt text that will be sent to AIs and custom GPT models
     * to determine the outcome.
     */
    string public prompt = "Was Trump the winner of the US election 2024?";

    /**
     * Outcome
     * True = Trump
     * False = Harris
     */
    bool public outcome;

    /**
     * Outcome can't be set before this timestamp
     */
    uint256 public outcomeDate = 1730838599;
    
    /**
     * After calling `setOutcome`, people can place a 
     * dispute during this period.
     */
    uint256 public disputePeriod = 12 hours;

    /*
    * To dispute, users can create a proposal on snapshot.org
    * and let the token holders vote.
    * The results of on-chain voting can be pushed
    * to the chain by a Muon app in a trustless and permissionless
    * way to resolve the dispute.
    */
    string public disputeSnapshotId;
    bool public disputed = false;
    bool public disputeResolved = false;


    // muonAppId and MuonPublicKey come from the FactGPT
    // app deployed on the Pion Network
    constructor(
        uint256 _muonAppId, // ID of Muon App
        PublicKey memory _muonPublicKey, // Public key of Muon App

        uint256 _disputeMuonAppId,
        PublicKey memory _disputeMuonPublicKey
    ){
        validatePubKey(_muonPublicKey.x);
        muonAppId = _muonAppId;
        muonPublicKey = _muonPublicKey;

        validatePubKey(_disputeMuonPublicKey.x);
        disputeMuonAppId = _disputeMuonAppId;
        disputeMuonPublicKey = _disputeMuonPublicKey;
    }

    /**
     * These functions can be called by users, projects,
     * or an automation service in a permissionless and trustless
     * manner.
     * MuonSignature originates from the Pion Network.
     */
    function setOutcome(
        bool _outcome,
        bytes calldata reqId,
        SchnorrSign calldata muonSignature
    ) public{
        require(block.timestamp > outcomeDate, "err: time < outcomeDate");
        bytes32 hash = keccak256(
            abi.encodePacked(
                muonAppId,
                reqId,
                _outcome
            )
        );
        bool verified = muonVerify(reqId, uint256(hash), muonSignature, muonPublicKey);
        require(verified, "Muon TSS sig not verified");

        outcome = _outcome;
    }

    /**
     * These functions can be called by users, projects,
     * or an automation service in a permissionless and trustless
     * manner.
     * MuonSignature originates from the Pion Network.
     */
    function dispute(
        string memory snapshotId,
        bytes calldata reqId,
        SchnorrSign calldata muonSignature
    ) public{
        // TODO: the user should lock a colateral or something
        // to place a dispute

        require(block.timestamp > outcomeDate, "err: time < outcomeDate");
        require(block.timestamp < outcomeDate + disputePeriod, "disputePeriod ended");
        bytes32 hash = keccak256(
            abi.encodePacked(
                disputeMuonAppId,
                reqId,
                snapshotId
            )
        );
        bool verified = muonVerify(reqId, uint256(hash), muonSignature, disputeMuonPublicKey);
        require(verified, "Muon TSS sig not verified");

        disputed = true;
        disputeSnapshotId = snapshotId;
    }

    /**
     * A Muon app loads the results of voting from
     * the snapshot proposal and signs a transaction.
     * If the dispute is accepted, the outcome will change.
     */
    function disputeResolve(
        bool accepted,
        bytes calldata reqId,
        SchnorrSign calldata muonSignature
    ) public{
        require(disputed && !disputeResolved, "Not disputed");
        //TODO: set and check a period to resolve the dispute
        bytes32 hash = keccak256(
            abi.encodePacked(
                disputeMuonAppId,
                reqId,
                accepted
            )
        );
        bool verified = muonVerify(reqId, uint256(hash), muonSignature, disputeMuonPublicKey);
        require(verified, "Muon TSS sig not verified");

        if(accepted){
            outcome = !outcome;
        }
        disputeResolved = true;
    }
}
