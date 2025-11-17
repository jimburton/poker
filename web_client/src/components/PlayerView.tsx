/**
Component that shows the player's hole cards and buttons for placing a bet..
**/

export default function PlayerView ( { placeBet, name, bankRoll, holeCards, call,
                                       possibleBets, isDealer, minBet, setMinBet } ) {
				       
  const canCheck = possibleBets.indexOf('Check') !== -1;
  const canAllIn = possibleBets.indexOf('AllIn') !== -1;
  const canCall = possibleBets.indexOf('Call') !== -1;
  const canRaise = possibleBets.indexOf('Raise') !== -1;
  const inputMin = Math.max(minBet, call);

  return (

       <form onSubmit={placeBet}>	
        <div className="custom-col-width-center grid-cell d-flex
                        justify-content-center align-items-center" id="player0-col">
          <div className="container-fluid">
            <div className="row">
              {isDealer &&
                <div className="col">
                  <img src='/images/dealer.png' className='dealerIcon'
                       alt='dealer button' />
                </div>
              }
              <div className="col">
                { name } <br />
                { bankRoll }
              </div>
              <div className="col">          
                <button type="submit" className="btn btn-danger" name="Fold">
                  Fold
                </button>
                <br />	  
                <button type="submit" className="btn btn-primary mt-2"
                        disabled={ !canAllIn } name="AllIn">All In</button>	
              </div>	      
              <div className="col">
                {holeCards.length == 2 &&
                  <img src={`/images/cards/${holeCards[0]}.svg`}
                       className='holeCard backlit_image' alt={`${holeCards[0]}`} />
                }
              </div>
              <div className="col">
                {holeCards.length == 2 &&    
                  <img src={`/images/cards/${holeCards[1]}.svg`}
                       className='holeCard' alt={`${holeCards[1]}`} />
                }
              </div>
              <div className="col">
                <button type="submit" className="btn btn-success"
                        disabled={ !canCheck }  name="Check">Check</button>
                <br />
                <button type="submit" className="btn btn-success mt-2"
                        disabled={ !canCall } name="Call">
                  Call
                </button>
              </div>	      
              <div className="col justify-content-center align-items-center">
                 <button type="submit" className="btn btn-success align-self-center"
			 disabled={ !canRaise } name="Raise">Raise</button>
                <br />
                <input type="number" 
                       id="amountInput" 
                       name="amount"
                       min={inputMin}
                       max={bankRoll}
                       defaultValue={inputMin}
		       value={inputMin}
		       onChange={e => setMinBet(e.target.value)}
                       step="10"
                       className="form-control align-self-center bet-spinner mt-2" />
              </div>
            </div>
          </div>
        </div>
      </form>
 
  )
}