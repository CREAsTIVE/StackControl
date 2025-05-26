import { ValueState } from "@/ValueState";
import React from "react";
import styles from "./CodeInputField.css"
import { cls } from "@/utils/cls";

export function CodeInputField(props: {className?: string, value: ValueState<string>, onSubmit?: () => void}) {
  let textarea = <textarea 
    className={cls(styles.input, props.className)}
    value={props.value.value}
    onChange={e => props.value.set(e.target.value)}
    onKeyDown={k => {
      if (k.key == "Enter" && k.shiftKey) {
        props.onSubmit?.call(null, [])
        k.preventDefault()
      }
    }}
  />;
  return textarea 
}