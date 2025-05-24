import * as _ from 'lodash'
import * as React from 'react'
import { createRoot } from 'react-dom/client'
import App from './App';

let root = createRoot(document.getElementById("root")!);
root.render(getRootComponent());

function getRootComponent() {
  return <App />
}

