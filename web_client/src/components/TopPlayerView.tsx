/**
Component showing an opponent for display at the top of the screen.
**/
export default function TopPlayerView ( { name, bankRoll, isDealer } ) {

  return (
    <div className="row">
      {isDealer &&
        <div className="col">
          <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
        </div>
      }
      <div className="col text-end">
        { name } <br />
        { bankRoll }
      </div>
      <div className="col align-items-start">
        <img src='/images/cards/back.svg' className='opponentCardV'
             alt='back of a playing card' />
        <img src='/images/cards/back.svg' className='opponentCardV'
             alt='back of a playing card' />
      </div>
    </div>
  )
}