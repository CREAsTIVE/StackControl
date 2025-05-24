import * as React from "react";
import * as sc from  "stack-control-wasm"

let scope: sc.JSScope = sc.make_scope()

const App = () => {
  sc.ensure()
  let [text, setText] = React.useState("");


  return <div>
    <input onChange={(e) => setText(e.target.value)} value={text} />
    <button onClick={() => {
      console.log(sc.execute(text, scope, false).join_stack(", "))
    }} >Compile</button>
  </div>
}

export default App;