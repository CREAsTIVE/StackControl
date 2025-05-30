import * as React from "react";
import * as sc from  "stack-control-wasm"
import styles from "./App.css"
import { ValueState } from "@/ValueState";
import { CodeInputField } from "./Components/CodeInputField/CodeInputField";
import { Button } from "./Components/Button/Button";
import { StackView } from "./Components/StackView/StackView";
import { CheckBox } from "./Components/CheckBox/CheckBox";
import documentationData from "@@/documentation.json"
import { CommandDescription } from "./Components/CommandDescription/CommandDescription";

let scope: sc.JSScope = sc.make_scope()

const App = () => {
  const sourceUrl = window.location;
  const params = new URLSearchParams(window.location.search);
  const paramsCode = params.get("code");
  

  let code = new ValueState(paramsCode ? paramsCode : "");
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
          <Button 
            className={styles.button}
            onClick={() => {
              let url = new URL(window.location.href)
              url.searchParams.set("code", code.get());
              navigator.clipboard.writeText(url.toString())

              alert("Link was copied to clipboard")
            }}
            text={<>COPY</>} 
          />
          <CheckBox 
            label="simplify"
            value={simplify}
          />
        </div>
      </div>
    </div>
    <h1>Commands:</h1>
    <div className={styles.docs}>
      {documentationData.map(e => 
        <div className={styles.element}>
          <CommandDescription commandInfo={e}/>
        </div>
      )}
    </div>
  </div>
}

export default App;