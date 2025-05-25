import React from "react"
import styles from "./StackView.css"
import { cls } from "@/utils/cls"
import { ValueState } from "@/ValueState"

export function StackView(props: {className?: string, items: ValueState<string[]>}) {
  return <div className={cls(styles.stackview, props.className)}>
    {props.items.get().map(e => (<div className={styles.item}>
      {e}
    </div>))}
  </div>
}