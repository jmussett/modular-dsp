import { ipcRenderer as ipc } from 'electron'
import * as React from 'react'
import { render } from 'react-dom'

let socket = new WebSocket('ws://localhost:3012')

interface IState {
    frequency: number
}

class WaveTable extends React.Component<{}, IState> {
    constructor(props: {}) {
        super(props)
        this.state = { frequency: 0 }
    }
    changeFrequency(e: React.ChangeEvent<HTMLInputElement>) {
        socket.send(JSON.stringify({
            commands: [{
                type: 'InputParameter',
                data: ['frequency', parseInt(e.target.value, 0)]
            }]
        }))
    }
    render() {
        return <div>
          <h1>Hello World!</h1>
          <span>
            <input type='number' min={0} step={10} defaultValue='0' onChange={this.changeFrequency}/>
          </span>
        </div>
    }
}

render(
    <WaveTable/>,
    document.getElementsByTagName('body')[0]
)
