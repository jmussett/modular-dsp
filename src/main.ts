declare var __dirname: string

import * as nativeDsp from '../native/index.node'

import { app, BrowserWindow, ipcMain as ipc } from 'electron'

let mainWindow: Electron.BrowserWindow

function initialize () {
    function onReady() {
        mainWindow = new BrowserWindow({
            width: 800,
            height: 600
        })

        const fileName = `file://${__dirname}/index.html`
        mainWindow.loadURL(fileName)
        mainWindow.on('close', app.quit)
    }

    app.on('ready', onReady)
    app.on('window-all-closed', app.quit)

    ipc.on('play', () => {
        console.log(nativeDsp.play())
    })

    console.log(`Electron Version ${app.getVersion()}`)
}

initialize()
