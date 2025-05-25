import React from "react";

export class ValueState<T> {
  value: T;
  private setValue: (val: T) => void;

  constructor(initial?: T) {
    [this.value, this.setValue] = React.useState(initial);
  }

  set(val: T) {
    this.setValue(val);
  }

  get() {return this.value;}
}