/**
The component that allows a user to start a game.
**/
export default function StartGame({ submit }) {
    return (

        <div className="container-fluid h-100 p-0">

            <div id="row1" className="bg-primary bg-opacity-75">
                <div className="row g-0 h-100">
                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>

                    <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center"><h1>Start a game</h1></div>

                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
                </div>
            </div>

            <div id="row2" className="bg-success bg-opacity-75">
                <div className="row g-0 h-100">
                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>

                    <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">
                        <form onSubmit={submit}>
                            <input className="form-control" type="text" placeholder="Name" name="name" id="name" />
                            <button type="submit" className="btn btn-primary m-2">Start game</button>
                        </form>
                    </div>

                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
                </div>
            </div>

            <div id="row3" className="bg-warning bg-opacity-75">
                <div className="row g-0 h-100">
                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>

                    <div className="custom-col-width-center grid-cell d-flex justify-content-center align-items-center">
                    </div>

                    <div className="custom-col-width grid-cell d-flex justify-content-center align-items-center"> </div>
                </div>
            </div>

        </div>
    )
}