import * as React from "react";
import * as sc from  "stack-control-wasm"
import styles from "./App.css"
import { ValueState } from "@/ValueState";
import { CodeInputField } from "./Components/CodeInputField/CodeInputField";
import { Button } from "./Components/Button/Button";
import { StackView } from "./Components/StackView/StackView";
import { CheckBox } from "./Components/CheckBox/CheckBox";

let scope: sc.JSScope = sc.make_scope()

const App = () => {
  let code = new ValueState("");
  let items = new ValueState<string[]>([])
  let simplify = new ValueState(true);

  function compile() {
    let res = sc.execute(code.value, scope, simplify.get());

    items.set(
      res
        .join_stack("||||")
        .split("||||")
    )

    if (simplify) {
      code.set(res.simplified)
    }
  }

  console.log(styles.app)

  return <div className={styles.app}>
    <div className={styles.block}>
      <div className={styles.codeblock}>
        <div className={styles.codearea}>
          <CodeInputField 
            value={code} 
            className={styles.codefield} 
            onSubmit={compile}
          />
          <StackView 
            items={items}
            className={styles.stackview}
          />
        </div>
        <div className={styles.options}>
          <Button 
            className={styles.button}
            onClick={compile}
            text={<>RUN</>} 
          />
          <CheckBox 
            label="simplify"
            value={simplify}
          />
        </div>
      </div>
    </div>
  </div>
}

export default App;