import * as React from "react";
import * as sc from  "stack-control-wasm"
import styles from "./App.css"

let scope: sc.JSScope = sc.make_scope()

const App = () => {
  let [text, setText] = React.useState("");

  console.log("STYLE: " + styles.app)

  return <div className={styles.app}>
    <input onChange={(e) => setText(e.target.value)} value={text} />
    <button onClick={() => {
      console.log(sc.execute(text, scope, false).join_stack(", "))
    }}>Compile</button>
  </div>
}

export default App;