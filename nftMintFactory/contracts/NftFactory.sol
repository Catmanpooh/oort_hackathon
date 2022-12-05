// SPDX-License-Identifier: MIT
pragma solidity ^0.8.0;

import "@openzeppelin/contracts/token/ERC721/extensions/ERC721URIStorage.sol";
import "@openzeppelin/contracts/utils/Counters.sol";

contract MarketItems is ERC721URIStorage {
    using Counters for Counters.Counter;
    Counters.Counter private _tokenIds;

    constructor(string memory name, string memory symbol) ERC721(name, symbol)  {}

    function nftItem(address player, string memory tokenURI)
        public
        returns (uint256)
    {
        uint256 newItemId = _tokenIds.current();
        _safeMint(player, newItemId);
        _setTokenURI(newItemId, tokenURI);

        _tokenIds.increment();
        return newItemId;
    }
}


contract NFTMarketFactory {

    address[] public contracts;
    address public lastContractAddress;       

    function getContractCount() public view returns(uint contractCount) {
        return contracts.length;
    }

     // deploy a new purchase contract
    function deploy(string memory name, string memory symbol) public returns(address newContract){
         MarketItems c = new MarketItems(name,symbol);
         address cAddr = address(c);
         contracts.push(cAddr);
         lastContractAddress = cAddr;

         return cAddr;
    }      

    function nftItem(MarketItems nftAddress, string memory tokenURI) public {

      nftAddress.nftItem(msg.sender, tokenURI);
    }       
}