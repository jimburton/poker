export default function Game( { players, dealer, holeCards }) {
       console.log(holeCards[0]);
  return (
  
  <div className="container-fluid h-100 p-0">
        
        <div id="row1" className="bg-primary bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">1A (10%)</div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">1B (80% Content Area)</div>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">1C (10%)</div>
            </div>
        </div>

        <div id="row2" className="bg-success bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">2A</div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">
                    <span className="fs-4">MIDDLE ROW (80vh)</span>
                </div>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">2C</div>
            </div>
        </div>

        <div id="row3" className="bg-warning bg-opacity-75">
            <div className="row g-0 h-100">
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">3A</div>
                
                <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">
		{holeCards.length == 2 &&

                  <>
                    <img src={`/images/cards/${holeCards[0]}.svg`} className='holeCard' alt={`${holeCards[0]}`} />
                    <img src={`/images/cards/${holeCards[1]}.svg`} className='holeCard' alt={`${holeCards[1]}`} />
		  </>
		}
                </div>
                
                <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center">3C</div>
            </div>
        </div>
        
    </div>

  )
}