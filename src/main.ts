declare var __dirname: string

import { app, BrowserWindow } from 'electron'

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

    console.log(`Electron Version ${app.getVersion()}`)
}

initialize()
