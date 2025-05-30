import React from "react"
import styles from "./CommandDescription.css"

export type CommandInfo = {
  key: string,
  aliases: string[],
  description: string
}

export function CommandDescription(props: {commandInfo: CommandInfo}) {
  return <div className={styles.container}>
    <div className={styles.header}>
      <div className={styles.key}>{props.commandInfo.key}</div>
      <div className={styles.aliases}>
        {props.commandInfo.aliases.map(e => (
          <div className={styles.alias}>
            {e}  
          </div>
        ))}
      </div>
    </div>
    <div className={styles.hr}/>
    <div>
      <div>
        {props.commandInfo.description}
      </div>
    </div>
  </div>
}