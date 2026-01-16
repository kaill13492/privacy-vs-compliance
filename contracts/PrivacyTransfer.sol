// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

interface IVerifier {
    function verifyProof(
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external view returns (bool);
}

contract PrivacyTransfer {
    IVerifier public verifier;

    bytes32 public merkleRoot;

    event PrivateTransfer(bytes32 commitment);

    constructor(address _verifier, bytes32 _root) {
        verifier = IVerifier(_verifier);
        merkleRoot = _root;
    }

    function submitTransfer(
        bytes calldata proof,
        uint256 commitment
    ) external {
        uint256;
        publicInputs[0] = commitment;
        publicInputs[1] = uint256(merkleRoot);

        require(
            verifier.verifyProof(proof, publicInputs),
            "Invalid ZK proof"
        );

        emit PrivateTransfer(bytes32(commitment));
    }
}
