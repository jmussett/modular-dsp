declare var __dirname: string

import { app, BrowserWindow } from 'electron'

let mainWindow: Electron.BrowserWindow | null

function initialize () {
    function createWindow() {
        mainWindow = new BrowserWindow({
            width: 800,
            height: 600,
            webPreferences: {
                nodeIntegration: true
            }
        })

        mainWindow.loadURL(`file://${__dirname}/index.html`)

        mainWindow.webContents.openDevTools()

        mainWindow.on('close', app.quit)
        mainWindow.on('closed', () => {
            mainWindow = null
        })
    }

    app.on('ready', createWindow)

    app.on('window-all-closed', () => {
        if (process.platform !== 'darwin') {
            app.quit()
        }
    })

    app.on('activate', () => {
        if (mainWindow === null) {
            createWindow()
        }
    })

    console.log(`Electron Version ${app.getVersion()}`)
}

initialize()
