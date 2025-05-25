import React from "react";
import styles from "./CheckBox.css"
import { ValueState } from '../../ValueState';

export function CheckBox(props: {value: ValueState<boolean>, label?: string}) {
  return <div className={styles.container}>
    {props.label ? (<>
      {props.label}
    </>) : (<></>)}

    <input 
      className={styles.checkbox} 
      type="checkbox" 
      onChange={e => props.value.set(e.target.checked)}
      checked={props.value.get()}
    />
  </div>
}