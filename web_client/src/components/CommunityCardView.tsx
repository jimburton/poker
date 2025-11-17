/**
Component displaying the community cards.
**/
export default function CommunityCardView({ communityCards, bestHandCards }) {

    return (

        <div id="communityCards"
            className="layout-section cards-height bg-green-700 p-4 
                    d-flex justify-content-center align-items-center text-center">
            <div className="text-white">

                {communityCards.map((c) =>
                    <img src={`/images/cards/${c}.svg`} className={`communityCard${bestHandCards.includes(c) ? ' backlit_image' : ''}`}
                        alt={`${c}`}
                        key={`${c}_img`} />
                )}
            </div>
        </div>
    )
}