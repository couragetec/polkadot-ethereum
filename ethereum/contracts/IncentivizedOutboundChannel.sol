// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;
pragma experimental ABIEncoderV2;

import "./OutboundChannel.sol";

// IncentivizedOutboundChannel is a channel that sends ordered messages with an increasing nonce. It will have incentivization too.
contract IncentivizedOutboundChannel is OutboundChannel {

    constructor() {
        nonce = 0;
    }

    /**
     * @dev Sends a message across the channel
     */
    function submit(bytes memory payload)
        public
        override
    {
        bytes memory metadata;
        emit Message(msg.sender, nonce, payload);
        nonce = nonce + 1;
    }


    function encodeMetadata(uint128 _fee)
        private
        pure
        returns (bytes memory)
    {
        return abi.encodePacked(_fee.toBytes32LE());
    }
}
