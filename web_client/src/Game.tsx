export default function Game( { playerName, bankRoll, players, dealer, holeCards,
                                communityCards, possibleBets, bestHand, call, minBet,
				placeBet, pot }) {

  const playerIndex = players.findIndex((p) => p[0] === playerName);
  console.log(playerIndex);
  console.log(players[playerIndex]);
  return (
  
  <div className="container-fluid h-100 p-0">
        
        <div id="row1" className="bg-primary bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center mx-auto" id="player2-col">

                 <div className="container-fluid">
		    <div className="row">
		      {players[(playerIndex+2)%players.length][0] === dealer &&
		        <div className="col">
		          <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
		        </div>
		      }
		      <div className="col d-flex flex-column align-items-end">
                        {players[(playerIndex+2)%players.length][0]} <br />
			{players[(playerIndex+2)%players.length][1]}
                      </div>
		      <div className="col d-flex align-items-start">
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
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center align-middle border" id="communityCardsCol">
                    <div className="">
                      <span>
		        Messages.
		      </span>
                    </div>
                      {communityCards &&

                        <div className="row align-middle">
                          {communityCards.map((c) =>
			    <div className="col align-middle" key={`${c}_row`}>
                              <img src={`/images/cards/${c}.svg`} className='communityCard' alt={`${c}`} key={`${c}_img`} />
                            </div>)}
                        </div>
                   
                      }
                    <div className="align-bottom">
                      <span className="fs-4">
		        pot: {pot}
		      </span>
		    </div>
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
                        {playerIndex && players[playerIndex] && players[playerIndex][0]} <br />
			{playerIndex && players[playerIndex] && players[playerIndex][1]}
                      </div>
		      <div className="col">
                          
			    <button type="submit" className="btn btn-danger" name="Fold">Fold</button>
                            <br />	  
			    <button type="submit" className="btn btn-primary mt-2" disabled={possibleBets.indexOf('AllIn') === -1} name="AllIn">All In</button>
		
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
			<br />
			<button type="submit" className="btn btn-success mt-2" disabled={possibleBets.indexOf('Call') === -1} name="Call">Call</button>
                      </div>
		      
		      <div className="col justify-content-center align-items-center">
                        <button type="submit" className="btn btn-success align-self-center" disabled={possibleBets.indexOf('Raise') === -1} name="Raise">Raise</button>
			<br />
			<input type="number" 
                               id="amountInput" 
                               name="amount"
                               min={call}
                               max={bankRoll}
                               defaultValue={call} 
                               step="10"
                               className="form-control align-self-center bet-spinner mt-2" />
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