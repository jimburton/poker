/**
The main view component.
**/
import { useState, useCallback } from "react";
import TopPlayerView from './TopPlayerView';
import SidePlayerView from './SidePlayerView';
import CommunityCardView from './CommunityCardView';
import PlayerView from './PlayerView';
import MessageQueue from './MessageQueue';

export default function GameView ( { playerName, bankRoll, players, dealer, holeCards,
                                communityCards, possibleBets, bestHandCards, call, minBet,
                                placeBet, pot, messageQueue, setMinBet }) {
        
  const playerIndex = players.findIndex((p) => p[0] === playerName);
  const playerName_top = players[(playerIndex+2)%players.length][0];
  const playerBankRoll_top = players[(playerIndex+2)%players.length][1];
  const isDealer_top = playerName_top === dealer;
  const playerName_left = players[(playerIndex+1)%players.length][0];
  const playerBankRoll_left = players[(playerIndex+1)%players.length][1];
  const isDealer_left = playerName_left === dealer;
  const playerName_right = players[(playerIndex+3)%players.length][0];
  const playerBankRoll_right = players[(playerIndex+3)%players.length][1];
  const isDealer_right = playerName_right === dealer;
  const isDealer_player = playerName === dealer;

  return (
  
  <div className="container-fluid h-100 p-0">
    <div id="row1" className="bg-primary bg-opacity-75 mx-auto">
      <div className="row g-0 h-100 mx-auto">
        <div className="custom-col-width grid-cell d-flex
            justify-content-center align-items-center">
        </div>
                
        <div className="custom-col-width-center grid-cell d-flex
                        justify-content-center align-items-center mx-auto"
             id="player2-col">
          <div className="container-fluid mx-auto">

            <TopPlayerView name={playerName_top}
                           bankRoll={playerBankRoll_top}
                           isDealer={isDealer_top} />

          </div>
        </div>
                
        <div className="custom-col-width grid-cell d-flex justify-content-center
            align-items-center">
        </div>
      </div>
    </div>

    <div id="row2" className="bg-success bg-opacity-75">
      <div className="row g-0 h-100">
        <div className="custom-col-width grid-cell d-flex justify-content-center
                        align-items-center" id="player1-col">
          <div className="container-fluid">

            <SidePlayerView name={playerName_left}
                bankRoll={playerBankRoll_left}
                            isDealer={isDealer_left} />

          </div>
        </div>
        <div className="custom-col-width-center grid-cell d-flex flex-column h-100"
             id="communityCardsCol">
          <div id="messages" className="layout-section message-height
                            p-3 rounded-t-xl d-flex
          justify-content-center align-items-center
          text-center">
            <div className="text-sm italic">

               <MessageQueue messages={messageQueue} />

           </div>
         </div>

           <CommunityCardView communityCards={communityCards} bestHandCards={bestHandCards} />

         <div id="potDiv" 
              className="layout-section pot-height bg-gray-800 p-2 rounded-b-xl
                         d-flex justify-content-center align-items-center
                         text-center">
         <div className="text-yellow-300 text-base">
           <span className="font-bold text-lg">Pot Total</span><br/>
             {pot}
         </div>
       </div>
     </div>
     <div className="custom-col-width grid-cell d-flex justify-content-center
                     align-items-center" id="player3-col">
        <div className="container-fluid">

         <SidePlayerView name={playerName_right}
                         bankRoll={playerBankRoll_right}
                         isDealer={isDealer_right} />

      </div>
    </div>
  </div>
</div>
<div id="row3" className="bg-warning bg-opacity-75">
  <div className="row g-0 h-100">
    <div className="custom-col-width grid-cell d-flex justify-content-center
        align-items-center"> </div>

      <PlayerView placeBet={placeBet}
                  name={playerName}
                  bankRoll={bankRoll}
                  holeCards={holeCards}
                  call={call}
                  possibleBets={possibleBets}
                  isDealer={isDealer_player}
		  minBet={minBet}
		  setMinBet={setMinBet}
		  bestHandCards={bestHandCards} />

      <div className="custom-col-width grid-cell d-flex justify-content-center
          align-items-center">
      </div>
    </div>
  </div>
</div>

  )
}