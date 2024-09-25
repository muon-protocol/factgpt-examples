// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/utils/cryptography/ECDSA.sol";

import "./muon-utils/MuonClient.sol";

contract SimplePredictionMarket is MuonClient {
    using ECDSA for bytes32;

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

    // muonAppId and MuonPublicKey come from the FactGPT
    // app deployed on the Pion Network
    constructor(
        uint256 _muonAppId, // ID of Muon App
        PublicKey memory _muonPublicKey // Public key of Muon App
    ) MuonClient(_muonAppId, _muonPublicKey){

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
}
