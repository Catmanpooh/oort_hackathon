import "../styles/globals.css";

import { WagmiConfig, createClient, configureChains, chain } from "wagmi";
import { jsonRpcProvider } from "wagmi/providers/jsonRpc";
import Navbar from "./components/Navbar";

const { chains, provider } = configureChains(
  [chain.mainnet, chain.hardhat],
  [
    jsonRpcProvider({
      rpc: () => ({
        http: `https://beta-rpc.mainnet.computecoin.com`,
      }),
    }),
  ]
);

const client = createClient({
  provider,
});

function MyApp({ Component, pageProps }) {
  return (
    <WagmiConfig client={client}>
      <Navbar/>
      <Component {...pageProps} />
    </WagmiConfig>
  );
}

export default MyApp;
