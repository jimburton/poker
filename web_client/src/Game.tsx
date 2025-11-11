export default function Game( { playerName, bankRoll, players, dealer, holeCards,
                                communityCards, possibleBets, bestHand, call, minBet,
				placeBet }) {

  const playerIndex = players.findIndex((p) => p[0] === playerName);
  console.log(playerIndex);
  console.log(players[playerIndex]);
  return (
  
  <div className="container-fluid h-100 p-0">
        
        <div id="row1" className="bg-primary bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center" id="player2-col">

                 <div className="container-fluid">
		    <div className="row">
		      {players[(playerIndex+2)%players.length][0] === dealer &&
		        <div className="col">
		          <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
		        </div>
		      }
		      <div className="col">
                        {players[(playerIndex+2)%players.length][0]} <br />
			{players[(playerIndex+2)%players.length][1]}
                      </div>
		      <div className="col">
		        <img src='/images/cards/back.svg' className='opponentCardV' alt='back of a playing card' />
			<img src='/images/cards/back.svg' className='opponentCardV' alt='back of a playing card' />
		      </div>
		    </div>
		 </div>

                </div>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
            </div>
        </div>

        <div id="row2" className="bg-success bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center" id="player1-col">

                <div className="container-fluid">
		    {players[(playerIndex+1)%players.length][0] === dealer &&
		        <div className="row">
			  <div className="col">
		            <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
			  </div>
		        </div>
		    }
		    <div className="row">
		      <div className="col">
		        {players[(playerIndex+1)%players.length][0]} <br />
			{players[(playerIndex+1)%players.length][1]}
		      </div>
		    </div>
		    <div className="row">
		      <div className="col">
		        <img src='/images/cards/back_horizontal.svg' className='opponentCardH' alt='back of a playing card' />
		      </div>
		    </div>
		    <div className="row">
		      <div className="col">
		        <img src='/images/cards/back_horizontal.svg' className='opponentCardH' alt='back of a playing card' />
		      </div>
		    </div>
		  </div>
		  
            </div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">
                    <span className="fs-4">MIDDLE ROW (80vh)</span>
                </div>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center" id="player3-col">

                  <div className="container-fluid">
		    {players[(playerIndex+3)%players.length][0] === dealer &&
		        <div className="row">
			  <div className="col">
		            <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
			  </div>
		        </div>
		    }
		    <div className="row">
		      <div className="col">
		        {players[(playerIndex+3)%players.length][0]} <br />
			{players[(playerIndex+3)%players.length][1]}
		      </div>
		    </div>
		    <div className="row">
		      <div className="col">
		        <img src='/images/cards/back_horizontal.svg' className='opponentCardH' alt='back of a playing card' />
		      </div>
		    </div>
		    <div className="row">
		      <div className="col">
		        <img src='/images/cards/back_horizontal.svg' className='opponentCardH' alt='back of a playing card' />
		      </div>
		    </div>
		  </div>

                </div>
            </div>
        </div>

        <div id="row3" className="bg-warning bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>

                <form onSubmit={placeBet}>
		
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center" id="player0-col">
                  
                  <div className="container-fluid">
		    <div className="row">
		      {playerName === dealer &&
		        <div className="col">
		          <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
		        </div>
		      }
		      <div className="col">
                        {players[playerIndex][0]} <br />
			{players[playerIndex][1]}
                      </div>
		      <div className="col">
                        <button type="submit" className="btn btn-danger" name="Fold">Fold</button>
                      </div>
		      <div className="col">
                        {holeCards.length == 2 &&
                    
                            <img src={`/images/cards/${holeCards[0]}.svg`} className='holeCard' alt={`${holeCards[0]}`} />
                      
		         }
		       </div>
		       <div className="col">
                        {holeCards.length == 2 &&
                  
                            <img src={`/images/cards/${holeCards[1]}.svg`} className='holeCard' alt={`${holeCards[1]}`} />

		         }
		       </div>
		       <div className="col">
                        <button type="submit" className="btn btn-success" disabled={possibleBets.indexOf('Check') === -1}  name="Check">Check</button>
                      </div>
		      <div className="col">
                        <button type="submit" className="btn btn-success" disabled={possibleBets.indexOf('AllIn') === -1} name="AllIn">All In</button>
                      </div>
		      <div className="col">
                        <button type="submit" className="btn btn-success" disabled={possibleBets.indexOf('Raise') === -1} name="Raise">Raise</button>
			<input type="number" 
                               id="amountInput" 
                               name="amount"
                               min={call}
                               max={bankRoll}
                               defaultValue={minBet} 
                               step="10"
                               className="form-control bet-spinner" />
                      </div>
		      <div className="col">
                        <button type="submit" className="btn btn-success" disabled={possibleBets.indexOf('Call') === -1} name="Call">Call</button>
                      </div>
		     </div>
		   </div>
		   
                </div>

                </form>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
            </div>
        </div>
        
    </div>

  )
}