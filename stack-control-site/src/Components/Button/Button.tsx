import React from "react"
import styles from "./Button.css"
import { cls } from "@/utils/cls"

export function Button(props: {className?: string, text: JSX.Element, onClick: () => void}) {
  return <button onClick={props.onClick} className={cls(styles.button, props.className)}>
    {props.text}
  </button>
}