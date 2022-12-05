const NFTMarketFactory = artifacts.require("NFTMarketFactory");
const MarketItems = artifacts.require("MarketItems");

module.exports = function (deployer) {
  deployer.deploy(NFTMarketFactory);
  deployer.deploy(MarketItems);
};
