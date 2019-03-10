import * as React from 'react'
import { render } from 'react-dom'
import { ipcRenderer as ipc } from 'electron'

let frequency = 0

let play = () => ipc.send('play')
let changeFrequency = () => ipc.send('changeFrequency', frequency)

render(
  <div>
    <h1>Hello World!</h1>
    <button onClick={() => play()}>Play</button>
    <span>
      <input ref={x => { frequency = Number(x) }}/>
      <button onClick={() => changeFrequency()}>Change Frequency</button>
    </span>
  </div>,
  document.getElementsByTagName('body')[0]
)
