// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;
pragma experimental ABIEncoderV2;

import "./OutboundChannel.sol";

// BasicOutboundChannel is a basic channel that just sends messages with a nonce.
contract BasicOutboundChannel is OutboundChannel {
    constructor() {
        nonce = 0;
    }

    /**
     * @dev Sends a message across the channel
     */
    function submit(bytes memory payload) public override {
        bytes memory metadata;
        emit Message(msg.sender, nonce, metadata, payload);
        nonce = nonce + 1;
    }
}
