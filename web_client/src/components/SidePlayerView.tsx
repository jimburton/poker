/**
Component showing an opponent for display on the side of the screen.
**/
export default function SidePlayerView ( { name, bankRoll, isDealer } ) {

  return (
    <>
      {isDealer &&
        <div className="row">
          <div className="col">
            <img src='/images/dealer.png' className='dealerIcon' alt='dealer button' />
          </div>
	</div>
      }
      <div className="row">
        <div className="col">
          { name } <br />
          { bankRoll }
        </div>
      </div>
      <div className="row">
        <div className="col">
          <img src='/images/cards/back_horizontal.svg' className='opponentCardH'
               alt='back of a playing card' />
	</div>
      </div>
      <div className="row">
        <div className="col">
          <img src='/images/cards/back_horizontal.svg' className='opponentCardH'
               alt='back of a playing card' />
        </div>
      </div>
   </>
  )
}