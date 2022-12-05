import axios from "axios";
import { useEffect, useState } from "react";
import { useAccount } from "wagmi";

const FactoryView = () => {
  const { address, isConnected } = useAccount();

  let [userInfo, setUserInfo] = useState([]);

  useEffect(() => {
    if (isConnected) {
      fetchData();
    }
  }, [isConnected]);

  const fetchData = async () => {
    let headersList = {
      Accept: "*/*",
    };

    let reqOptions = {
      url: `http://localhost:3000/api/v1/user_nft_items/${address}`,
      method: "GET",
      headers: headersList,
    };

    let response = await axios.request(reqOptions);
    console.log(userInfo);
    setUserInfo(response.data);
  };

  return (
    <div className="h-full w-full flex justify-center items-center">
      {userInfo.length === 0 ? (
        <h1 className="text-center text-3xl font-bold mt-10">
          Go create something amazing!
        </h1>
      ) : (
        <div className="mt-10  w-10/12 grid grid-cols-3 auto-cols-max gap-5">
          {userInfo.map((user, i) => {
            return (
              <div key={i} className="w-auto h-full">
                <div className="card w-96 bg-base-200 shadow-xl">
                  <figure className="lg:h-62 h-52">
                    <img alt="nft image." src={user.file} />
                  </figure>
                  <div className="card-body">
                    <h2 className="card-title">Metadata</h2>
                    {Object.entries(user.metadata).map(([key, value]) => {
                      return (
                        <p className="font-medium">
                          {key}: {value}
                        </p>
                      );
                    })}
                  </div>
                </div>
              </div>
            );
          })}
        </div>
      )}
    </div>
  );
};

export default FactoryView;
