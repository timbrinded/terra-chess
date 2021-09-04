import * as path from "path";
import BN from "bn.js";
import chalk from "chalk";
import * as chai from "chai";
import chaiAsPromised from "chai-as-promised";
import { LocalTerra, LCDClient, MnemonicKey, MsgInstantiateContract,  MsgExecuteContract, Wallet, MsgStoreCode} from "@terra-money/terra.js";
import {
  toEncodedBinary,
  sendTransaction,
  storeCode,
  instantiateContract,
  queryNativeTokenBalance,
  queryTokenBalance,
} from "./helpers";

chai.use(chaiAsPromised);
const { expect } = chai;

//----------------------------------------------------------------------------------------
// Variables
//----------------------------------------------------------------------------------------

//const terra = new LocalTerra();

const terra = new LCDClient({
  chainID: "bombay-10",
  URL: "https://bombay-lcd.terra.dev"
});

const mk1 = new MnemonicKey({
    mnemonic: 'tide blood snow giant desert tape wash pluck toward december casino maple clump click grunt model country peasant noodle tourist plastic service power opera'
});

const mk2 = new MnemonicKey({
  mnemonic: 'oven orient borrow machine valley rice transfer wrist jacket symbol maple enemy either hole friend void foot dumb leisure paddle symptom shy sure print'
});


const user1 = terra.wallet(mk1);
const user2 = terra.wallet(mk2);

let chessAddress: string = "terra1fydtl60hm63wkumpj2m06k93ztdnq2grphr0f0";
type MoveEvent = { old_x: number, old_y: number, new_x: number, new_y: number }

//----------------------------------------------------------------------------------------
// Setup
//----------------------------------------------------------------------------------------

async function setupTest() {
  // Step 1. Upload TerraSwap Token code
  process.stdout.write("Uploading TerraSwap Terra Chess Code... ");

  const contractNum = await storeCode(
    terra,
    user1,
    path.resolve(__dirname, "../artifacts/terra_chess.wasm")
  );

  console.log(chalk.green("Done!"), `${chalk.blue("codeId")}=${contractNum}`);

  // Step 2. Instantiate TerraSwap Token contract
  process.stdout.write("Instantiating TerraSwap Token contract... \n");

  const response = await instantiateContract(terra, user1, user1, contractNum, {"admin":user1.key.accAddress});

  chessAddress = response.logs[0].events[0].attributes[3].value;

  console.log(chalk.green("Done!"), `${chalk.blue("contractAddress")}=${chessAddress}`);

}


async function startMatch(host: Wallet, opponent: Wallet, opening_move: MoveEvent) {
  process.stdout.write("Starting a match between players... ");
  console.log("user1 and user2\n");

  const msg = new MsgExecuteContract(user1.key.accAddress,chessAddress, {"start_match":{"first_move":{"new":[opening_move.new_x,opening_move.new_y],"original":[opening_move.old_x,opening_move.old_y]},"opponent":opponent.key.accAddress}});

  await sendTransaction(terra, host, [msg]);

}

 async function submitPlay(sender:Wallet, host: Wallet, opponent: Wallet, move: MoveEvent) {
  process.stdout.write("Submitting play... \n");

  const msg = new MsgExecuteContract(sender.key.accAddress,chessAddress, {"play_move":{"host":host.key.accAddress,"opponent":opponent.key.accAddress,"your_move":{"new":[move.new_x,move.new_y],"original":[move.old_x,move.old_y]}}});
  
  await sendTransaction(terra, sender, [msg]);

 }

 async function queryMatch(host: Wallet, opponent: Wallet): Promise<string> {
  process.stdout.write("Querying current state of match between players... ");

  const msg = new MsgExecuteContract(user1.key.accAddress,chessAddress, {"CheckMatch":{"host":"terra17pwuad5t4th8tw39kyuwmcujty2mceevu6f7rf","opponent":"terra1gqwlwpuaj9s9ncu2t88387zdr2z2a7zdm9c205"}});

  let res = await terra.wasm.contractQuery<string>(chessAddress, msg);

  return res.toString();
 }

//----------------------------------------------------------------------------------------
// Main
//----------------------------------------------------------------------------------------

(async () => {
  console.log(chalk.yellow("\nStep 1. Info"));
  console.log(`Use ${chalk.cyan(user1.key.accAddress)} as user 1`);
  console.log(`Use ${chalk.cyan(user2.key.accAddress)} as user 2`);

  console.log(chalk.yellow("\nStep 2. Setup"));

  //await setupTest();

  console.log(chalk.yellow("\nStep 3. Tests"));

  //Round 1
  let move: MoveEvent = {old_x:4,old_y:1,new_x:4,new_y:3};
  await startMatch(user1,user2,move);
  move = {old_x:4,old_y:6,new_x:4,new_y:4};
  await submitPlay(user2,user1,user2,move);

  //Round 2
  move = {old_x:3,old_y:0,new_x:7,new_y:4};
  await submitPlay(user1,user1,user2,move);
  move = {old_x:6,old_y:7,new_x:5,new_y:5};
  await submitPlay(user2,user1,user2,move);
  
  //Round 3
  move = {old_x:5,old_y:0,new_x:2,new_y:3};
  await submitPlay(user1,user1,user2,move);
  move = {old_x:1,old_y:7,new_x:2,new_y:5};
  await submitPlay(user2,user1,user2,move);

  // Deathblow!! This move will checkmate black and end the game.
  // Finished games will be removed from the stage storage so 

  // move = {old_x:7,old_y:4,new_x:5,new_y:6};
  // await submitPlay(user1,user1,user2,move);

  console.log("Finished!!");
})();
