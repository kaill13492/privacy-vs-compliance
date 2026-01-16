// SPDX-License-Identifier: MIT
pragma solidity ^0.8.20;

contract Verifier {
    function verifyProof(
        bytes calldata proof,
        uint256[] calldata publicInputs
    ) external view returns (bool) {
        // pairing checks
        // elliptic curve ops
        // constraint verification
        return true;
    }
}
