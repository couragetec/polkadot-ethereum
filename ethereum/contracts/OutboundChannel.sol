// SPDX-License-Identifier: MIT
pragma solidity >=0.7.6;
pragma experimental ABIEncoderV2;

// OutboundChannel contains methods that all outgoing channels must implement
abstract contract OutboundChannel {

    uint64 public nonce;

    event Message(
        address source,
        uint64 nonce,
        bytes metadata,
        bytes payload
    );

    /**
     * @dev Sends a message across the channel
     */
    function submit(bytes memory payload)
        public
        virtual;
}
