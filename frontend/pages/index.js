import Head from "next/head";
import { useEffect, useState } from "react";
//use to send body with get method
import axios from "axios";
import { useAccount } from "wagmi";

const REGEXTOKENURI = "^((?:ipfs|stroj|https?)://)?([0-z-.]+).([a-z]{2,})$";

const METADATA = [
  {
    trait_type: "Background",
    value: "white",
  },
  {
    trait_type: "Body",
    value: "body_green",
  },
  {
    trait_type: "Eyes",
    value: "green_squint",
  },
  {
    trait_type: "Glasses",
    value: "librarian",
  },
  {
    trait_type: "Hats",
    value: "mmcHat",
  },
  {
    trait_type: "Mouth",
    value: "grin",
  },
  {
    trait_type: "Shirts",
    value: "floral",
  },
  {
    trait_type: "Neck",
    value: "chain",
  },
  {
    trait_type: "Body",
    value: "body_orange",
  },
  {
    trait_type: "Eyes",
    value: "3Eyes",
  },
  {
    trait_type: "Glasses",
    value: "JohnLennon",
  },
  {
    trait_type: "Hats",
    value: "Headphones",
  },
  {
    trait_type: "Mouth",
    value: "tongueOut",
  },
  {
    trait_type: "Neck",
    value: "alien",
  },
];

export default function Home() {
  const [storeIt, setStoreIt] = useState(true);
  const [isDisabled, setIsDisabled] = useState(false);
  const { address, isConnected } = useAccount();

  const handleSubmit = async (e) => {
    e.preventDefault();
    setIsDisabled(true);

   
    let name;
    let headersList;
    let bodyContent;
    let reqOptions;


    const formData = Object.fromEntries(new FormData(e.target));

    if (!isConnected) {
      setIsDisabled(false);
      console.log("Connect your wallet!");
      return;
    }

    if (formData.projectTokenUri) {
      setIsDisabled(false);
      console.log("Need to set up function");
      return 
    }
    // Info for storage charge on mint for this
    if (formData.projectJson?.name || formData.projectImage?.name) {
      name = formData.projectName.toLocaleLowerCase().replaceAll(/\s/g, "-");

      headersList = {
        Accept: "*/*",
      };

      let formdata = new FormData();
      formdata.append("project_name", name);
      formdata.append("file_image", formData.projectImage);
      if (formData.projectJson?.name) {
        formData.append("file_json", formData.projectJson);
      }

      bodyContent = formdata;

      reqOptions = {
        url: "http://localhost:3000/api/v1/create",
        method: "POST",
        headers: headersList,
        data: bodyContent,
      };

      await axios.request(reqOptions).catch((err) => {
        console.log("Error: ", err);
        alert("Unsucessful please try again!");
      });
    }

    headersList = {
      Accept: "*/*",
      "Content-Type": "application/json",
    };

    let metadata;

    metadata = {
      trait_type: {
        Strand:
          METADATA[Math.floor(Math.random() * METADATA.length)].trait_type,
      },
      value: {
        Strand: METADATA[Math.floor(Math.random() * METADATA.length)].value,
      },
    };


    bodyContent = JSON.stringify({
      address: address,
      contract_address: "0x1d2569Bf9A36204b250D45Efb6ffd2f763C012FC",
      metadata: metadata,
      project_name: name,
      object_name: formData.projectImage.name,
    });

    reqOptions = {
      url: "http://localhost:3000/api/v1/object_uri",
      method: "POST",
      headers: headersList,
      data: bodyContent,
    };

    await axios.request(reqOptions).catch((err) => {
      console.log("Error: ", err);
      alert("Unsucessful please try again!");
    });

    setIsDisabled(false);
  };

  return (
    <div className="h-full">
      <Head>
        <title>Home Page</title>
        <meta name="description" content="Generated by create next app" />
        <link rel="icon" href="/favicon.ico" />
      </Head>
      <h1 className=" my-6 text-center text-3xl font-bold">
        Create a Piece of Art On-Chain
      </h1>
      <p className="text-center text-lg font-medium">
        Let your fans be amazed!
      </p>
      <div className="w-full h-auto flex justify-center my-10">
        <form
          className="flex flex-col justify-between w-1/2"
          onSubmit={handleSubmit}
        >
          <input
            type="text"
            name="projectName"
            required
            minLength="3"
            maxLength="63"
            placeholder="Name"
            className="input input-bordered w-full mb-5"
          />

          <input
            type="text"
            name="projectSymbol"
            required
            minLength="1"
            maxLength="4"
            placeholder="Symbol"
            className="input input-bordered w-full my-5"
          />

          <div className="form-control my-5">
            <label className="label cursor-pointer">
              <span className="label-text">
                Do you need to storage for your nft?
              </span>
              <input
                type="checkbox"
                className="toggle toggle-accent"
                checked={storeIt}
                onChange={() => setStoreIt(!storeIt)}
              />
            </label>
          </div>
          {storeIt ? (
            <div className="flex flex-col justify-evenly">
              <label className="label">
                <span className="label-text">Upload json file here</span>
              </label>
              <input
                type="file"
                disabled
                name="projectJson"
                className="file-input file-input-bordered w-full mb-5"
              />

              <label className="label">
                <span className="label-text">Upload image here</span>
              </label>
              <input
                type="file"
                name="projectImage"
                required
                className="file-input file-input-bordered w-full mb-5"
              />
            </div>
          ) : (
            <input
              type="text"
              name="projectTokenUri"
              required
              minLength="1"
              maxLength="255"
              pattern={REGEXTOKENURI}
              placeholder="Token Uri"
              className="input input-bordered w-full mb-5"
            />
          )}

          <button
            className="btn btn-accent mt-5"
            type="submit"
            disabled={isDisabled}
          >
            Submit
          </button>
        </form>
      </div>
    </div>
  );
}
