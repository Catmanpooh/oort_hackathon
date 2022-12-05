# oort_hackathon
Using oort to interact with storj creating a nft factory.


## Inspiration
Working on a many nft projects, there are a few difficulties I notice most new coming users have. They have a hard time getting up and running quickly. They usually have to find a developer to do most the task. Why not give the user the freedom to just upload files give the collection a name and everything is handled for them? This is what I have created.

## What it does
The user goes to the site and with the simple interface they have a few options give a name a symbol and some files. The system will store the files on a decentralize storage system. After this the information that was give is then minted to the Olympus blockchain.

## How we built it
A rust api  was created for connecting to the oort dss. It uses surrealdb to save some contract and other user information. Next js is used for the frontend with axios to make request to the backend. Orrt with storj as the decentralized storage provider.

## Challenges we ran into
Biggest challenge is integrating in the blockchain portion of this project. I sent the contract I created over to a developer in the telegram (Let me give him or her a shoutout Oachis). The person told me that my contract bytecode was to big. This required me to do some redesigning to the system.

## Accomplishments that we're proud of
Pretty proud of using rust to create a api to interact with oort. More than likely it is over kill but I had a blast creating playing around with it.

## What we learned
I learned a lot about aws-s3 crate and how to interact with oort dss. 

## What's next for Boss Hog Nft Factory
Getting the contract up and running on the mainnet-beta once it is available. Add a nicer gallery page, things get a little weird for me dealing with css, when it works but when it doesn't yeaaa. Also look more into using other storage available on oort. 
