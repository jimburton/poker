export default function StartGame( { submit }) {
  return (

    <div className="row">
      <h1>Start a game</h1>
      <form onSubmit={submit}>
        <label htmlFor="name">Name</label>
	<input type="text" name="name" id="name" />
        <button type="submit">Start game</button>
      </form>
    </div>
    )
}